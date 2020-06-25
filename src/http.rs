//
//  mbus-httpd - An HTTP microservice exposing M-Bus Functionality
//  Copyright (C) 2019-2020 packom.net
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

#![allow(dead_code)]

use mbus_api::models;
use mbus_api::{
    GetMultiResponse, GetResponse, HatOffResponse, HatOnResponse, HatResponse, MbusApiResponse,
    ScanResponse,
};
use std::env;
use std::fs;
use std::path::Path;
use std::process::Command;
use std::str;
use std::sync::Mutex;
use sysfs_gpio::{Direction, Pin};

use lazy_static::lazy_static;
use log::info;

const LIBMBUS_PATH_VAR: &str = "LIBMBUS_PATH";
const LIBMBUS_PATH_DEF: &str = "/usr/local/bin/";
const LIBMBUS_GET_VAR: &str = "LIBMBUS_GET";
const LIBMBUS_GET_DEF: &str = "mbus-serial-request-data";
const LIBMBUS_GET_MULTI_VAR: &str = "LIBMBUS_GET_MULTI";
const LIBMBUS_GET_MULTI_DEF: &str = "mbus-serial-request-data-multi-reply";
const LIBMBUS_SCAN_VAR: &str = "LIBMBUS_SCAN";
const LIBMBUS_SCAN_DEF: &str = "mbus-serial-scan";
const LD_LIBRARY_PATH_VAR: &str = "LD_LIBRARY_PATH";

const DEV_PREFIX: &str = "/dev/";
const HAT_PATH: &str = "/proc/device-tree/hat/";
const HAT_PRODUCT: &str = "product";
const HAT_PRODUCT_ID: &str = "product_id";
const HAT_PRODUCT_VER: &str = "product_ver";
const HAT_UUID: &str = "uuid";
const HAT_VENDOR: &str = "vendor";
const MBUS_MASTER_HAT_PID: &str = "0x0001";
const MBUS_MASTER_HAT_VENDOR: &str = "packom.net";
const MBUS_MASTER_POWER_GPIO: u64 = 26;

pub fn get_env() -> Vec<&'static str> {
    vec![
        LIBMBUS_PATH_VAR,
        LIBMBUS_GET_VAR,
        LIBMBUS_GET_MULTI_VAR,
        LIBMBUS_SCAN_VAR,
        LD_LIBRARY_PATH_VAR,
    ]
}

lazy_static! {
    static ref LIBMBUS_PATH: String = {
        match env::var(LIBMBUS_PATH_VAR) {
            Ok(v) => v,
            Err(_) => LIBMBUS_PATH_DEF.to_string(),
        }
    };
    static ref LIBMBUS_GET: String = {
        match env::var(LIBMBUS_GET_VAR) {
            Ok(v) => v,
            Err(_) => LIBMBUS_GET_DEF.to_string(),
        }
    };
    static ref LIBMBUS_GET_MULTI: String = {
        match env::var(LIBMBUS_GET_MULTI_VAR) {
            Ok(v) => v,
            Err(_) => LIBMBUS_GET_MULTI_DEF.to_string(),
        }
    };
    static ref LIBMBUS_SCAN: String = {
        match env::var(LIBMBUS_SCAN_VAR) {
            Ok(v) => v,
            Err(_) => LIBMBUS_SCAN_DEF.to_string(),
        }
    };
    static ref GPIO: Pin = Pin::new(MBUS_MASTER_POWER_GPIO);
    static ref BUS: Mutex<i32> = Mutex::new(0);
}

pub(crate) fn api() -> MbusApiResponse {
    info!("API {}", "api");
    let rsp = match fs::read_to_string("/static/api.yaml")
        .map_err(|e| format!("Failed to read file: {}", e))
    {
        Ok(s) => MbusApiResponse::OK(s),
        Err(e) => MbusApiResponse::NotFound(e),
    };
    info!("API {} -> {:?}", "get_api", rsp);
    rsp
}

fn hat_exists() -> bool {
    if Path::new(HAT_PATH).exists() {
        true
    } else {
        info!("No hat path: {}", HAT_PATH);
        false
    }
}

fn hat_get_value(field: &str) -> Option<String> {
    let filename = HAT_PATH.to_owned() + field;
    let path = Path::new(&filename);
    if path.exists() {
        fs::read_to_string(path)
            .map_err(|e| {
                info!("Failed to read hat value: {}, {}", field, e);
                e
            })
            // Strip off NULL teminator from file
            .map(|s| Some(s.trim_end_matches("\u{0000}").to_string()))
            .unwrap_or(None)
    } else {
        info!("Hat value not present: {}", field);
        None
    }
}

pub(crate) fn hat() -> HatResponse {
    info!("API {}", "hat");

    let rsp = if hat_exists() {
        HatResponse::OK(models::Hat {
            product: hat_get_value(HAT_PRODUCT),
            product_id: hat_get_value(HAT_PRODUCT_ID),
            product_ver: hat_get_value(HAT_PRODUCT_VER),
            uuid: hat_get_value(HAT_UUID),
            vendor: hat_get_value(HAT_VENDOR),
        })
    } else {
        HatResponse::NotFound("Hat not present".to_string())
    };

    info!("API {} -> {:?}", "hat", rsp);
    rsp
}

fn hat_is_mbus_master() -> bool {
    if !hat_exists() {
        return false;
    }

    let pid = hat_get_value(HAT_PRODUCT_ID).unwrap_or("".to_string());
    let vendor = hat_get_value(HAT_VENDOR).unwrap_or("".to_string());

    if pid != MBUS_MASTER_HAT_PID {
        info!("Hat product ID not M-Bus Master: {}", pid);
        return false;
    }

    if vendor != MBUS_MASTER_HAT_VENDOR {
        info!("Hat vendor not M-Bus Master: {}", vendor);
        return false;
    }

    true
}

fn hat_power(val: u8) -> Result<(), String> {
    if hat_is_mbus_master() {
        if !GPIO.is_exported() {
            GPIO.export()
                .map_err(|e| format!("Failed to get GPIO control: {}", e))?;
        }
        GPIO.set_direction(Direction::Out)
            .map_err(|e| format!("Failed to set GPIO as output: {}", e))?;
        GPIO.set_value(val)
            .map_err(|e| format!("Failed to set GPIO value: {}, {}", val, e))?;
        Ok(())
    } else {
        Err("M-Bus Master Hat not installed".to_string())
    }
}

pub(crate) fn hat_off() -> HatOffResponse {
    info!("API {}", "hat_off");

    let rsp = match hat_power(0) {
        Ok(_) => HatOffResponse::OK,
        Err(e) => HatOffResponse::NotFound(e),
    };

    info!("API {} -> {:?}", "hat_off", rsp);
    rsp
}

pub(crate) fn hat_on() -> HatOnResponse {
    info!("API {}", "hat_on");

    let rsp = match hat_power(1) {
        Ok(_) => HatOnResponse::OK,
        Err(e) => HatOnResponse::NotFound(e),
    };

    info!("API {} -> {:?}", "hat_on", rsp);
    rsp
}

fn check_address(address: &String) -> Result<(), String> {
    let len = address.len();
    let err_s = format!("Not a valid primary or secondary address");
    if len == 16 {
        let chars = address.chars();
        if chars.map(|c| c.is_ascii_hexdigit()).any(|b| b == false) {
            Err(err_s)
        } else {
            Ok(())
        }
    } else if len >= 1 && len <= 3 {
        let int = address.parse::<i32>();
        match int {
            Ok(int) => {
                if int >= 0 && int <= 255 {
                    Ok(())
                } else {
                    Err(err_s)
                }
            }
            _ => Err(err_s),
        }
    } else {
        Err(err_s)
    }
}

pub(crate) fn get(device: &String, baudrate: &models::Baudrate, address: &String) -> GetResponse {
    info!("API {} : {:?} {:?} {:?}", "get", device, baudrate, address);

    // Check parameters
    match check_address(address) {
        Ok(_) => (),
        Err(s) => return GetResponse::BadRequest(s),
    };

    let lock = BUS.try_lock();
    if lock.is_err() {
        return GetResponse::NotFound("Bus is currently in use".to_string());
    }

    // Construct mbus command like this:
    // mbus-serial-request-data [-d] [-b BAUDRATE] device mbus-address
    let cmd = LIBMBUS_PATH.to_owned() + &LIBMBUS_GET;
    let dev = DEV_PREFIX.to_owned() + device;
    info!("Executing: {} -b {} {} {}", cmd, baudrate, dev, address);
    // XXX Todo - execute as a future
    let rsp = match Command::new(cmd)
        .arg("-b")
        .arg(baudrate.to_string())
        .arg(dev)
        .arg(address)
        .output()
    {
        Ok(o) => {
            if o.status.success() {
                match String::from_utf8(o.stdout) {
                    // Should already be XML
                    Ok(s) => {
                        let x = serde_xml_rs::to_string(&s).unwrap();
                        println!("s: {:?}", s);
                        println!("x: {:?}", x);
                        GetResponse::OK(s)
                    }
                    // Somehow failed to convert stdout to a string!
                    Err(e) => GetResponse::NotFound(format!("Failed to query M-Bus: {:?}", e)),
                }
            } else {
                // Process returned an error code
                let code = match o.status.code() {
                    Some(c) => c.to_string(),
                    None => "None".to_string(),
                };
                // Get stderr (not stdout)
                let stderr = match str::from_utf8(&o.stderr) {
                    Ok(s) => s.to_string(),
                    Err(e) => format!("Failed to parse stderr {:?}", e),
                };
                GetResponse::NotFound(format!(
                    "Failed to query M-Bus: Internal error, return code {}, stderr {}",
                    code, stderr
                ))
            }
        }
        // Actually executing the process failed - couldn't find the process?
        Err(e) => GetResponse::NotFound(format!("Failed to query M-Bus: Internal error {:?}", e)),
    };

    info!("API {} -> {:?}", "get", rsp);
    rsp
}

pub(crate) fn get_multi(
    device: &String,
    baudrate: &models::Baudrate,
    address: &String,
    maxframes: &i32,
) -> GetMultiResponse {
    info!(
        "API {} : {:?} {:?} {:?} {:?}",
        "get", device, baudrate, address, maxframes
    );

    // Check parameters
    match check_address(address) {
        Ok(_) => (),
        Err(s) => return GetMultiResponse::BadRequest(s),
    };

    let lock = BUS.try_lock();
    if lock.is_err() {
        return GetMultiResponse::NotFound("Bus is currently in use".to_string());
    }

    // Construct mbus command like this:
    // mbus-serial-request-data [-d] [-b BAUDRATE] device mbus-address
    let cmd = LIBMBUS_PATH.to_owned() + &LIBMBUS_GET_MULTI;
    let dev = DEV_PREFIX.to_owned() + device;
    info!(
        "Executing: {} -b {} -f {} {} {}",
        cmd, baudrate, maxframes, dev, address
    );
    // XXX Todo - execute as a future
    let rsp = match Command::new(cmd)
        .arg("-b")
        .arg(baudrate.to_string())
        .arg(dev)
        .arg(address)
        .output()
    {
        Ok(o) => {
            if o.status.success() {
                match str::from_utf8(&o.stdout) {
                    // Should already be XML
                    Ok(s) => GetMultiResponse::OK(s.to_string()),
                    // Somehow failed to convert stdout to a string!
                    Err(e) => GetMultiResponse::NotFound(format!("Failed to query M-Bus: {:?}", e)),
                }
            } else {
                // Process returned an error code
                let code = match o.status.code() {
                    Some(c) => c.to_string(),
                    None => "None".to_string(),
                };
                // Get stderr (not stdout)
                let stderr = match str::from_utf8(&o.stderr) {
                    Ok(s) => s.to_string(),
                    Err(e) => format!("Failed to parse stderr {:?}", e),
                };
                GetMultiResponse::NotFound(format!(
                    "Failed to query M-Bus: Internal error, return code {}, stderr {}",
                    code, stderr
                ))
            }
        }
        // Actually executing the process failed - couldn't find the process?
        Err(e) => {
            GetMultiResponse::NotFound(format!("Failed to query M-Bus: Internal error {:?}", e))
        }
    };

    info!("API {} -> {:?}", "get_multi", rsp);
    rsp
}

pub(crate) fn scan(device: &String, baudrate: &models::Baudrate) -> ScanResponse {
    info!("API {} : {:?} {:?}", "scan", device, baudrate);

    let lock = BUS.try_lock();
    if lock.is_err() {
        return ScanResponse::NotFound("Bus is currently in use".to_string());
    }

    // Construct libmbus exec like this:
    // mbus-serial-scan [-d] [-b BAUDRATE] [-r RETRIES] device
    let cmd = LIBMBUS_PATH.to_owned() + &LIBMBUS_SCAN;
    let dev = DEV_PREFIX.to_owned() + device;
    info!("Executing: {} -b {} {}", cmd, baudrate, dev);
    // XXX Todo - execute as a future
    let rsp = match Command::new(cmd)
        .arg("-b")
        .arg(baudrate.to_string())
        .arg(dev)
        .output()
    {
        // XXX Don't duplicate following code
        Ok(o) => {
            if o.status.success() {
                match str::from_utf8(&o.stdout) {
                    Ok(s) => ScanResponse::OK(s.to_string()), // XXX Actually, need to convert to XML
                    // Somehow failed to convert stdout to a string!
                    Err(e) => ScanResponse::NotFound(format!("Failed to query M-Bus: {:?}", e)),
                }
            } else {
                // Process returned an error code
                let code = match o.status.code() {
                    Some(c) => c.to_string(),
                    None => "None".to_string(),
                };
                // Get stderr (not stdout)
                let stderr = match str::from_utf8(&o.stderr) {
                    Ok(s) => s.to_string(),
                    Err(e) => format!("Failed to parse stderr {:?}", e),
                };
                ScanResponse::NotFound(format!(
                    "Failed to query M-Bus: Internal error, return code {}, stderr {}",
                    code, stderr
                ))
            }
        }
        // Actually executing the process failed - couldn't find the process?
        Err(e) => ScanResponse::NotFound(format!("Failed to query M-Bus: Internal error {:?}", e)),
    };

    info!("API {} -> {:?}", "scan", rsp);
    rsp
}
