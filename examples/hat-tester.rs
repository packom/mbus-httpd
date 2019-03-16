#![allow(missing_docs, unused_variables, trivial_casts)]

extern crate openapi_client;
#[allow(unused_extern_crates)]
extern crate futures;
#[allow(unused_extern_crates)]
#[macro_use]
extern crate swagger;
#[allow(unused_extern_crates)]
extern crate uuid;
#[macro_use]
extern crate clap;
extern crate tokio_core;
extern crate regex;

use swagger::{ContextBuilder, EmptyContext, XSpanIdString, Push, AuthData};

#[allow(unused_imports)]
use futures::{Future, future, Stream, stream};
use tokio_core::reactor;
#[allow(unused_imports)]
use openapi_client::{ApiNoContext, ContextWrapperExt,
                      ApiError,
                      ApiResponse,
                      GetResponse,
                      HatResponse,
                      HatOffResponse,
                      HatOnResponse,
                      ScanResponse
                     };
use clap::{App, Arg};
use openapi_client::models::{Address, Baudrate, Device, Hat};
use openapi_client::Client;
use openapi_client::Api;
use std::thread::sleep;
use std::time;
use regex::Regex;

trait Log {
    fn log_string(&self) -> String;
    fn log(&self);
}

impl Log for Hat {
    fn log_string(&self) -> String {
        let mut output: String = String::new();
        output = output + &format!("  Product:     {}\n", self.product.clone().unwrap_or("None".to_string()));
        output = output + &format!("  Product ID:  {}\n", self.product_id.clone().unwrap_or("None".to_string()));
        output = output + &format!("  Product Ver: {}\n", self.product_ver.clone().unwrap_or("None".to_string()));
        output = output + &format!("  UUID:        {}\n", self.uuid.clone().unwrap_or("None".to_string()));
        output = output + &format!("  Vendor:      {}\n", self.vendor.clone().unwrap_or("None".to_string()));
        output
    }

    fn log(&self) {
        print!("{}", self.log_string());
    }
}

macro_rules! context {
    () => {
        {
            let context: make_context_ty!(ContextBuilder, EmptyContext, Option<AuthData>, XSpanIdString) =
            make_context!(ContextBuilder, EmptyContext, None as Option<AuthData>, XSpanIdString(self::uuid::Uuid::new_v4().to_string()));
            context
        }
    }
}

fn get_hat(log: bool, succeed: bool, core: &mut reactor::Core, client: &mut Client<hyper::client::FutureResponse>) {   
    if log { print!("Get hat ... ") };
    let result = core.run(client.hat(&context!())).expect("failed to contact server");
    match result {
        HatResponse::OK(hat) => { 
            if log { println!("success:") };
            if log { hat.log() };
            if ! succeed { panic!(1) };
        },
        HatResponse::NotFound(_) => {
            if log { println!("no hat installed") };
            if succeed { panic!(1) };
        },
    }
}

fn get(
    log: bool,
    succeed: bool,
    device: Device,
    baudrate: Baudrate,
    address: Address,
    core: &mut reactor::Core,
    client: &mut Client<hyper::client::FutureResponse>
) {   
    if log {
        print!("Get info from device {}, baudrate {}, address {} ... ", String::from(device.clone()), baudrate, i32::from(address.clone()))
    }
    let result = core.run(client.get(device.to_string(), baudrate, i32::from(address), &context!())).expect("failed to contact server");
    match result {
        GetResponse::OK(info) => { 
            if log { println!("success") };
            if ! succeed { panic!(1) };
        },
        GetResponse::BadRequest(e) => {
            if log { println!("internal error {}", e)} ;
            if succeed { panic!(1) };
        },
        GetResponse::NotFound(_) => {
            if log { println!("no device found")} ;
            if succeed { panic!(1) };
        },
    }
}

fn hat_off(log: bool, succeed: bool, core: &mut reactor::Core, client: &mut Client<hyper::client::FutureResponse>) {   
    if log { print!("Hat off ... ") };
    let result = core.run(client.hat_off(&context!())).expect("failed to contact server");
    match result {
        HatOffResponse::OK => { 
            if log { println!("success") };
            if ! succeed { panic!(1) };
        },
        HatOffResponse::NotFound(_) => {
            if log { println!("no hat installed")} ;
            if succeed { panic!(1) };
        },
    }
}

fn hat_on(log: bool, succeed: bool, core: &mut reactor::Core, client: &mut Client<hyper::client::FutureResponse>) {   
    if log { print!("Hat on  ... ") };
    let result = core.run(client.hat_on(&context!())).expect("failed to contact server");
    match result {
        HatOnResponse::OK => { 
            if log { println!("success") };
            if ! succeed { panic!(1) };
        },
        HatOnResponse::NotFound(_) => {
            if log { println!("no hat installed") };
            if succeed { panic!(1) };
        },
    }
}

fn scan(
    log: bool,
    succeed: bool,
    device: Device,
    baudrate: Baudrate,
    core: &mut reactor::Core,
    client: &mut Client<hyper::client::FutureResponse>
) {   
    if log {
        print!("Scan device {}, baudrate {} ... ", String::from(device.clone()), baudrate);
    }
    let result = core.run(client.scan(device.to_string(), baudrate, &context!())).expect("failed to contact server");
    match result {
        ScanResponse::OK(results) => { 
            if log { 
                print!("success, found devices ");
                for addr in Regex::new("[0-9]{1,3}").unwrap().find_iter(&results) {
                    print!("{}", addr.as_str().parse::<i32>().unwrap());
                }
                println!("");
            }
            if ! succeed { panic!(1) };
        },
        ScanResponse::BadRequest(e) => {
            if log { println!("internal error {}", e)} ;
            if succeed { panic!(1) };
        },
        ScanResponse::NotFound(_) => {
            if log { println!("no device found")} ;
            if succeed { panic!(1) };
        },
    }
}

fn main() {
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
            .help("M-Bus slave address to communicate with"))
        .arg(Arg::with_name("scan")
            .long("scan")
            .help("Whether to scan the M-Bus"))
        .get_matches();

    let mut core = reactor::Core::new().unwrap();
    let is_https = matches.is_present("https");
    let base_url = format!("{}://{}:{}",
                           if is_https { "https" } else { "http" },
                           matches.value_of("host").unwrap(),
                           matches.value_of("port").unwrap());
    let mut client = if matches.is_present("https") {
        // Using Simple HTTPS
        openapi_client::Client::try_new_https(core.handle(), &base_url, "examples/ca.pem")
            .expect("Failed to create HTTPS client")
    } else {
        // Using HTTP
        openapi_client::Client::try_new_http(core.handle(), &base_url)
            .expect("Failed to create HTTP client")
    };

    let device: Device = Device::from(matches.value_of("device").unwrap().parse::<String>().expect("Invalid valid for device"));
    let baudrate: Baudrate = matches.value_of("baudrate").unwrap().parse().expect("Invalid valid for baudrate");
    let address = Address::from(matches.value_of("address").unwrap().parse::<i32>().expect("Invalid valid for baudrate"));
    let scan_b = matches.is_present("scan");

    // Run tests
    println!("==> Test can get hat details when hat is off and on");
    let sleep_time = time::Duration::from_millis(1000);
    hat_off(true, true, &mut core, &mut client);
    sleep(sleep_time);
    get_hat(true, true, &mut core, &mut client);
    sleep(sleep_time);
    hat_on(true, true, &mut core, &mut client);
    sleep(sleep_time);
    get_hat(true, true, &mut core, &mut client);
    sleep(sleep_time);
    println!("==> Success");

    println!("==> Test fast hat switching");
    let sleep_time = time::Duration::from_millis(10);
    for i in 1..100 {
        hat_off(false, true, &mut core, &mut client);
        sleep(sleep_time);
        hat_on(false, true, &mut core, &mut client);
        sleep(sleep_time);
    }
    hat_off(false, true, &mut core, &mut client);
    sleep(sleep_time);
    println!("==> Success");

    if scan_b {
        println!("==> Scan bus");
        let sleep_time = time::Duration::from_millis(1000);
        hat_on(false, true, &mut core, &mut client);
        sleep(sleep_time);
        scan(true, true, device.clone(), baudrate.clone(), &mut core, &mut client);
        hat_off(false, true, &mut core, &mut client);
        sleep(sleep_time);
        println!("==> Success");
    }
    
    println!("==> Get data from slave");
    let sleep_time = time::Duration::from_millis(1000);
    hat_on(false, true, &mut core, &mut client);
    sleep(sleep_time);
    get(true, true, device.clone(), baudrate.clone(), address.clone(), &mut core, &mut client);
    hat_off(false, true, &mut core, &mut client);
    sleep(sleep_time);
    println!("==> Success");
    
    println!("==> Get data from slave with bus off");
    let sleep_time = time::Duration::from_millis(1000);
    hat_off(true, true, &mut core, &mut client);
    sleep(sleep_time);
    get(true, false, device.clone(), baudrate.clone(), address.clone(), &mut core, &mut client);
    println!("==> Success");

    println!("==> Leaving hat off");
    hat_off(true, true, &mut core, &mut client);
    println!("==> Success");

}

