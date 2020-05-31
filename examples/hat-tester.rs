//
//  hat-tester - A tester for mbus-httpd
//  Copyright (C) 2019  packom.net
//
//  This program is free software: you can redistribute it and/or modify
//  it under the terms of the GNU General Public License as published by
//  the Free Software Foundation, either version 3 of the License, or
//  (at your option) any later version.
//
//  This program is distributed in the hope that it will be useful,
//  but WITHOUT ANY WARRANTY; without even the implied warranty of
//  MERCHANTABILITY or FITNESS FOR A PARTICULAR PURPOSE.  See the
//  GNU General Public License for more details.
//
//  You should have received a copy of the GNU General Public License
//  along with this program.  If not, see <https://www.gnu.org/licenses/>.
//
#![allow(missing_docs, unused_variables, trivial_casts)]

use swagger::{AuthData, ContextBuilder, EmptyContext, Push, XSpanIdString};

use clap::{App, Arg, crate_version};
#[allow(unused_imports)]
use futures::{future, stream, Future, Stream};
use mbus_api::models::{Address, Baudrate, Device, Hat};
use mbus_api::Api;
use mbus_api::Client;
#[allow(unused_imports)]
use mbus_api::{
    ApiError, ApiNoContext, MbusApiResponse, ContextWrapperExt, GetResponse, HatOffResponse,
    HatOnResponse, HatResponse, ScanResponse,
};
use regex::Regex;
use std::thread::sleep;
use std::time;
use tokio_core::reactor;
use std::process::exit;
use swagger::{make_context, make_context_ty};

const PRODUCT: &str = "M-Bus Master";
const PRODUCT_ID: &str = "0x0001";
const VENDOR: &str = "packom.net";

macro_rules! outputsf {
    ($f: ident) => {
        if ! $f {
            println!("==> Success")
        } else {
            println!("==> Failed")
        }
    }
}
            
fn main() {
    httpd_util::reg_for_sigs();

    let matches = App::new("mbus-httpd-hat-tester")
        .author("packom.net, mbus@packom.net")
        .version(crate_version!())
        .about("An app to test the function of an M-Bus Master Hat.\nSee https://www.packom.net/m-bus-master-hat/\n(C) Copyright 2019  packom.net")
        .arg(Arg::with_name("https")
            .long("https")
            .help("Whether to use HTTPS or not"))
        .arg(Arg::with_name("host")
            .long("host")
            .takes_value(true)
            .default_value("localhost")
            .help("Hostname to contact"))
        .arg(Arg::with_name("port")
            .long("port")
            .takes_value(true)
            .default_value("80")
            .help("Port to contact"))
        .arg(Arg::with_name("device")
            .long("device")
            .takes_value(true)
            .default_value("ttyAMA0")
            .help("Device M-Bus is attached to, e.g. ttyAMA0"))
        .arg(Arg::with_name("baudrate")
            .long("baudrate")
            .takes_value(true)
            .default_value("2400")
            .help("Baudrate to communicate with M-Bus"))
        .arg(Arg::with_name("address")
            .long("address")
            .takes_value(true)
            .default_value("48")
            .help("M-Bus slave address to communicate with 1 <= address <= 250"))
        .arg(Arg::with_name("get-repetitions")
            .long("get-reps")
            .takes_value(true)
            .default_value("1")
            .help("Number of times to repeat the get test (with the hat on)"))
        .arg(Arg::with_name("repetitions")
            .long("reps")
            .takes_value(true)
            .default_value("1")
            .help("Number of times to repeat all the tests"))
        .arg(Arg::with_name("uuid")
            .long("uuid")
            .takes_value(true)
            .help("Test the installed Hat has the provided UUID"))
        .arg(Arg::with_name("product-ver")
            .long("product-ver")
            .takes_value(true)
            .default_value("0x0002")
            .help("Hat Product Version"))
        .arg(Arg::with_name("scan")
            .long("scan")
            .help("Whether to scan the M-Bus"))
        .arg(Arg::with_name("check-scan")
            .long("check-scan")
            .help("Whether to check scan finds address"))
        .arg(Arg::with_name("hat")
            .long("hat")
            .help("Whether to test Hat specific functions"))
        .arg(Arg::with_name("hard")
            .long("hard")
            .help("Hard mode - any error exits"))
        .arg(Arg::with_name("verbose")
            .long("verbose")
            .help("Verbose logging"))
        .get_matches();

    // Handle arguments
    let device: Device = Device::from(
        matches
            .value_of("device")
            .unwrap()
            .parse::<String>()
            .expect("Invalid valid for device"),
    );
    let baudrate: Baudrate = matches
        .value_of("baudrate")
        .unwrap()
        .parse()
        .expect("Invalid valid for baudrate");
    let address = Address::from(
        matches
            .value_of("address")
            .unwrap()
            .parse::<i32>()
            .expect("Invalid valid for baudrate"),
    );
    if i32::from(address.clone()) < 1 || i32::from(address.clone()) > 250 {
        println!("Address outside allowed range");
        exit(1);
    }
    let scan_b = matches.is_present("scan");
    let match_addr = if matches.is_present("check-scan") {
        address.clone()
    } else {
        Address::from(0)
    };
    let hat_b = matches.is_present("hat");
    let reps = matches
        .value_of("repetitions")
        .unwrap()
        .parse::<i32>()
        .expect("Invalid repetitions value");
    if reps < 1 {
        println!("Invalid repetitions value");
        exit(1);
    }
    let get_reps = matches
        .value_of("get-repetitions")
        .unwrap()
        .parse::<i32>()
        .expect("Invalid repetitions value");
    if get_reps < 1 {
        println!("Invalid repetitions value");
        exit(1);
    }
    let uuid: Option<String> = if matches.is_present("uuid") {
        Some(matches.value_of("uuid").unwrap().to_string())
    } else {
        None
    };
    let product_ver = matches.value_of("product-ver").unwrap();
    let match_hat = Hat {
        product: Some(PRODUCT.to_string()),
        product_id: Some(PRODUCT_ID.to_string()),
        product_ver: Some(product_ver.to_string()),
        uuid: uuid,
        vendor: Some(VENDOR.to_string()),
    };
    let hard = matches.is_present("hard");
    let verbose = matches.is_present("verbose");
    let mut errors = Errors::new(hard);

    // Create client
    let mut core = reactor::Core::new().unwrap();
    let is_https = matches.is_present("https");
    let base_url = format!(
        "{}://{}:{}",
        if is_https { "https" } else { "http" },
        matches.value_of("host").unwrap(),
        matches.value_of("port").unwrap()
    );
    let mut client = if matches.is_present("https") {
        // Using Simple HTTPS
        mbus_api::Client::try_new_https(&base_url)
            .expect("Failed to create HTTPS client")
    } else {
        // Using HTTP
        mbus_api::Client::try_new_http(&base_url)
            .expect("Failed to create HTTP client")
    };

    // Run tests
    println!("====> Run tests {} times", reps);
    for rep in 0..reps {
        println!("===> Repetition {}", rep + 1);
        // Run tests
        if hat_b {
            println!("==> Test can get hat details when hat is off and on");
            let mut failed = false;
            let sleep_time = time::Duration::from_millis(1000);
            hat_off(verbose, true, &mut core, &mut client)
                .or_else(|e| { failed = true; errors.off() } )
                .ok();
            sleep(sleep_time);
            get_hat(verbose, true, &match_hat, &mut core, &mut client)
                .or_else(|e| { failed = true; errors.hat() } )
                .ok();
            sleep(sleep_time);
            hat_on(verbose, true, &mut core, &mut client)
                .or_else(|e| { failed = true; errors.on() } )
                .ok();
            sleep(sleep_time);
            get_hat(verbose, true, &match_hat, &mut core, &mut client)
                .or_else(|e| { failed = true; errors.hat() } )
                .ok();
            sleep(sleep_time);
            outputsf!(failed);

            println!("==> Test fast hat switching");
            let mut failed = false;
            let mut sleep_time = time::Duration::from_millis(10);
            for i in 1..100 {
                hat_off(false, true, &mut core, &mut client)
                    .or_else(|e| { failed = true; errors.off() } )
                    .ok();
                sleep(sleep_time);
                hat_on(false, true, &mut core, &mut client)
                    .or_else(|e| { failed = true; errors.on() } )
                    .ok();
                sleep(sleep_time);
                sleep_time = sleep_time + time::Duration::from_millis(2);
            }
            hat_off(false, true, &mut core, &mut client)
                .or_else(|e| { failed = true; errors.off() } )
                .ok();
            sleep(sleep_time);
            outputsf!(failed);
        }

        if scan_b {
            println!("==> Scan bus");
            let mut failed = false;
            let sleep_time = time::Duration::from_millis(1000);
            if hat_b {
                hat_on(false, true, &mut core, &mut client)
                    .or_else(|e| { failed = true; errors.on() } )
                    .ok();
                sleep(sleep_time);
            }
            scan(
                verbose,
                true,
                device.clone(),
                baudrate.clone(),
                match_addr.clone(),
                &mut core,
                &mut client,
            )
            .or_else(|e| { failed = true; errors.scan() } )
            .ok();
            if hat_b {
                hat_off(false, true, &mut core, &mut client)
                    .or_else(|e| { failed = true; errors.off() } )
                    .ok();
                sleep(sleep_time);
            }
            outputsf!(failed);
        }

        println!("===> Run test {} times", get_reps);
        let sleep_time = time::Duration::from_millis(1000);
        if hat_b {
            hat_on(false, true, &mut core, &mut client)
                .or_else(|e| errors.on() )
                .ok();
            sleep(sleep_time);
        }
        let sleep_time = time::Duration::from_millis(10);
        for rep in 0..get_reps {
            println!("==> Get data from slave repetition {}", rep+1);
            let mut failed = false;
            get(
                verbose,
                true,
                device.clone(),
                baudrate.clone(),
                address.clone(),
                &mut core,
                &mut client,
            )
            .or_else(|e| { failed = true; errors.get() } )
            .ok();
            outputsf!(failed);
            sleep(sleep_time);
        }
        let sleep_time = time::Duration::from_millis(1000);
        if hat_b {
            hat_off(false, true, &mut core, &mut client)
                .or_else(|e| errors.off() )
                .ok();
            sleep(sleep_time);
        }

        if hat_b {
            println!("==> Get data from slave with bus off");
            let mut failed = false;
            let sleep_time = time::Duration::from_millis(1000);
            hat_off(false, true, &mut core, &mut client)
                .or_else(|e| { failed = true; errors.off() } )
                .ok();
            sleep(sleep_time);
            get(
                verbose,
                false,
                device.clone(),
                baudrate.clone(),
                address.clone(),
                &mut core,
                &mut client,
            )
            .or_else(|e| { failed = true; errors.get() } )
            .ok();
            outputsf!(failed);
        }

        if hat_b {
            println!("==> Leaving hat off");
            let mut failed = false;
            hat_off(false, true, &mut core, &mut client)
                .or_else(|e| { failed = true; errors.off() } )
                .ok();
            outputsf!(failed);
        }
    }

    println!("===> Failures");
    errors.log();
}

struct Errors {
    hat: i32,
    hat_off: i32,
    hat_on: i32,
    get: i32,
    scan: i32,
    hard: bool,
}

impl Errors {
    fn new(hard: bool) -> Self {
        Errors {
            hat: 0,
            hat_off: 0,
            hat_on: 0,
            get: 0,
            scan: 0,
            hard,
        }
    }

    fn hat(&mut self) -> Result<(), ()> {
        if self.hard {
            println!("Hat failed");
            exit(1);
        }
        self.hat += 1;
        Ok(())
    }

    fn off(&mut self) -> Result<(), ()> {
        if self.hard {
            println!("Hat off failed");
            exit(1);
        }
        self.hat_off += 1;
        Ok(())
    }

    fn on(&mut self) -> Result<(), ()> {
        if self.hard {
            println!("Hat on failed");
            exit(1);
        }
        self.hat_on += 1;
        Ok(())
    }

    fn get(&mut self) -> Result<(), ()> {
        if self.hard {
            println!("Get failed");
            exit(1);
        }
        self.get += 1;
        Ok(())
    }

    fn scan(&mut self) -> Result<(), ()> {
        if self.hard {
            println!("Scan failed");
            exit(1);
        }
        self.scan += 1;
        Ok(())
    }

    fn log(&self) {
        println!("  Hat info: {}", self.hat);
        println!("  Hat off:  {}", self.hat_off);
        println!("  Hat on:   {}", self.hat_on);
        println!("  Get data: {}", self.get);
        println!("  Scan:     {}", self.scan);
    }
}

trait Process<T> {
    fn log_string(&self) -> String;
    fn log(&self);
    fn validate(&self, log: bool, match_v: &T);
}

impl Process<Hat> for Hat {
    fn log_string(&self) -> String {
        let mut output: String = "Hat details:\n".to_string();
        output = output
            + &format!(
                "  Product:     {}\n",
                self.product.clone().unwrap_or("None".to_string())
            );
        output = output
            + &format!(
                "  Product ID:  {}\n",
                self.product_id.clone().unwrap_or("None".to_string())
            );
        output = output
            + &format!(
                "  Product Ver: {}\n",
                self.product_ver.clone().unwrap_or("None".to_string())
            );
        output = output
            + &format!(
                "  UUID:        {}\n",
                self.uuid.clone().unwrap_or("None".to_string())
            );
        output = output
            + &format!(
                "  Vendor:      {}\n",
                self.vendor.clone().unwrap_or("None".to_string())
            );
        output
    }

    fn log(&self) {
        print!("{}", self.log_string());
    }

    fn validate(&self, log: bool, match_hat: &Hat) {
        if match_hat.product != self.product {
            if log {
                println!("Incorrect Hat Product");
                self.log()
            }
            exit(1);
        }
        if match_hat.product_id != self.product_id {
            if log {
                println!("Incorrect Hat Product ID");
                self.log();
            }
            exit(1);
        }
        if match_hat.product_ver != self.product_ver {
            if log {
                println!("Incorrect Hat Product Ver");
                self.log();
            }
            exit(1);
        }
        if match_hat.uuid.is_some() {
            if match_hat.uuid.clone().unwrap() != self.uuid.clone().expect("No Hat UUID returned") {
                if log {
                    println!("Incorrect Hat UUID");
                    self.log();
                }
                exit(1);
            }
        }
        if match_hat.vendor != self.vendor {
            if log {
                println!("Incorrect Hat Vendor");
                self.log();
            }
            exit(1);
        }
        if log {
            println!("Validated Hat details");
        }
    }
}

macro_rules! succeed {
    ($s:ident) => {
        if $s {
            Ok(())
        } else {
            Err(())
        }
    };
}

macro_rules! fail {
    ($s:ident) => {
        if $s {
            Err(())
        } else {
            Ok(())
        }
    };
}

macro_rules! context {
    () => {{
        let context: make_context_ty!(
            ContextBuilder,
            EmptyContext,
            Option<AuthData>,
            XSpanIdString
        ) = make_context!(
            ContextBuilder,
            EmptyContext,
            None as Option<AuthData>,
            XSpanIdString(uuid::Uuid::new_v4().to_string())
        );
        context
    }};
}

fn get_hat(
    log: bool,
    succeed: bool,
    match_hat: &Hat,
    core: &mut reactor::Core,
    client: &mut Client<hyper::client::ResponseFuture>,
) -> Result<(), ()> {
    if log {
        print!("Get hat ... ")
    };
    let result = core
        .run(client.hat(&context!()))
        .expect("failed to contact server");
    match result {
        HatResponse::OK(hat) => {
            if log {
                println!("success:")
            };
            if !succeed {
                return Err(());
            };
            hat.validate(log, match_hat);
            Ok(())
        }
        HatResponse::NotFound(_) => {
            if log {
                println!("no hat installed")
            };
            fail!(succeed)
        }
    }
}

fn get(
    log: bool,
    succeed: bool,
    device: Device,
    baudrate: Baudrate,
    address: Address,
    core: &mut reactor::Core,
    client: &mut Client<hyper::client::ResponseFuture>,
) -> Result<(), ()> {
    if log {
        print!(
            "Get info from device {}, baudrate {}, address {} ... ",
            String::from(device.clone()),
            baudrate,
            i32::from(address.clone())
        )
    }
    let result = core
        .run(client.get(
            device.to_string(),
            baudrate,
            i32::from(address),
            &context!(),
        ))
        .expect("failed to contact server");
    match result {
        GetResponse::OK(info) => {
            if log {
                println!("success")
            };
            succeed!(succeed)
        }
        GetResponse::BadRequest(e) => {
            if log {
                println!("internal error {}", e)
            };
            fail!(succeed)
        }
        GetResponse::NotFound(_) => {
            if log {
                println!("no device found")
            };
            fail!(succeed)
        }
    }
}

fn hat_off(
    log: bool,
    succeed: bool,
    core: &mut reactor::Core,
    client: &mut Client<hyper::client::ResponseFuture>,
) -> Result<(), ()> {
    if log {
        print!("Hat off ... ")
    };
    let result = core
        .run(client.hat_off(&context!()))
        .expect("failed to contact server");
    match result {
        HatOffResponse::OK => {
            if log {
                println!("success")
            };
            succeed!(succeed)
        }
        HatOffResponse::NotFound(_) => {
            if log {
                println!("no hat installed")
            };
            fail!(succeed)
        }
    }
}

fn hat_on(
    log: bool,
    succeed: bool,
    core: &mut reactor::Core,
    client: &mut Client<hyper::client::ResponseFuture>,
) -> Result<(), ()> {
    if log {
        print!("Hat on  ... ")
    };
    let result = core
        .run(client.hat_on(&context!()))
        .expect("failed to contact server");
    match result {
        HatOnResponse::OK => {
            if log {
                println!("success")
            };
            succeed!(succeed)
        }
        HatOnResponse::NotFound(_) => {
            if log {
                println!("no hat installed")
            };
            fail!(succeed)
        }
    }
}

fn scan(
    log: bool,
    succeed: bool,
    device: Device,
    baudrate: Baudrate,
    address: Address,
    core: &mut reactor::Core,
    client: &mut Client<hyper::client::ResponseFuture>,
) -> Result<(), ()> {
    if log {
        print!(
            "Scan device {}, baudrate {} ... ",
            String::from(device.clone()),
            baudrate
        );
    }
    let result = core
        .run(client.scan(device.to_string(), baudrate, &context!()))
        .expect("failed to contact server");
    match result {
        ScanResponse::OK(results) => {
            let mut match_addr = false;
            for addr in Regex::new("[0-9]{1,3}").unwrap().find_iter(&results) {
                let addr = addr.as_str().parse::<i32>().unwrap();
                if log {
                    print!("{} ", addr);
                }
                if addr == i32::from(address.clone()) {
                    match_addr = true;
                }
                if log {
                    println!("");
                }
            }
            if !succeed {
                exit(1)
            };
            if match_addr {
                Ok(())
            } else {
                if log {
                    println!("Didn't find address {:?}", address);
                }
                return Err(());
            }
        }
        ScanResponse::BadRequest(e) => {
            if log {
                println!("internal error {}", e)
            };
            fail!(succeed)
        }
        ScanResponse::NotFound(_) => {
            if log {
                println!("no device found")
            };
            fail!(succeed)
        }
    }
}
