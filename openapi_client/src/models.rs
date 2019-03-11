#![allow(unused_imports, unused_qualifications, unused_extern_crates)]
extern crate chrono;
extern crate uuid;


use serde::ser::Serializer;

use std::collections::HashMap;
use models;
use swagger;


/// Slave address
#[derive(Debug, Clone, PartialEq, PartialOrd, Serialize, Deserialize)]

pub struct Address(i32);

impl ::std::convert::From<i32> for Address {
    fn from(x: i32) -> Self {
        Address(x)
    }
}

impl ::std::convert::From<Address> for i32 {
    fn from(x: Address) -> Self {
        x.0
    }
}

impl ::std::ops::Deref for Address {
    type Target = i32;
    fn deref(&self) -> &i32 {
        &self.0
    }
}

impl ::std::ops::DerefMut for Address {
    fn deref_mut(&mut self) -> &mut i32 {
        &mut self.0
    }
}


/// Baudrate to use for the communication - valid values 300, 600, 1200, 2400, 4800, 9600
/// Enumeration of values.
/// Since this enum's variants do not hold data, we can easily define them them as `#[repr(C)]`
/// which helps with FFI.
#[allow(non_camel_case_types)]
#[repr(C)]
#[derive(Debug, Clone, Copy, PartialEq, PartialOrd, Serialize, Deserialize, Eq, Ord)]
pub enum Baudrate { 
    #[serde(rename = "300")]
    _300,
    #[serde(rename = "600")]
    _600,
    #[serde(rename = "1200")]
    _1200,
    #[serde(rename = "2400")]
    _2400,
    #[serde(rename = "4800")]
    _4800,
    #[serde(rename = "9600")]
    _9600,
}

impl ::std::fmt::Display for Baudrate {
    fn fmt(&self, f: &mut ::std::fmt::Formatter) -> ::std::fmt::Result {
        match *self { 
            Baudrate::_300 => write!(f, "{}", "300"),
            Baudrate::_600 => write!(f, "{}", "600"),
            Baudrate::_1200 => write!(f, "{}", "1200"),
            Baudrate::_2400 => write!(f, "{}", "2400"),
            Baudrate::_4800 => write!(f, "{}", "4800"),
            Baudrate::_9600 => write!(f, "{}", "9600"),
        }
    }
}

impl ::std::str::FromStr for Baudrate {
    type Err = ();
    fn from_str(s: &str) -> Result<Self, Self::Err> {
        match s {
            "300" => Ok(Baudrate::_300),
            "600" => Ok(Baudrate::_600),
            "1200" => Ok(Baudrate::_1200),
            "2400" => Ok(Baudrate::_2400),
            "4800" => Ok(Baudrate::_4800),
            "9600" => Ok(Baudrate::_9600),
            _ => Err(()),
        }
    }
}

/// The device the M-Bus is connected to - /dev/ is prepended to {device} by M-Bus HTTPD
#[derive(Debug, Clone, PartialEq, PartialOrd, Serialize, Deserialize)]

pub struct Device(String);

impl ::std::convert::From<String> for Device {
    fn from(x: String) -> Self {
        Device(x)
    }
}

impl ::std::convert::From<Device> for String {
    fn from(x: Device) -> Self {
        x.0
    }
}

impl ::std::ops::Deref for Device {
    type Target = String;
    fn deref(&self) -> &String {
        &self.0
    }
}

impl ::std::ops::DerefMut for Device {
    fn deref_mut(&mut self) -> &mut String {
        &mut self.0
    }
}


/// Raspberry Pi Hat Information
#[derive(Debug, Clone, PartialEq, Serialize, Deserialize)]
pub struct Hat {
    /// Product
    #[serde(rename = "product")]
    #[serde(skip_serializing_if="Option::is_none")]
    pub product: Option<String>,

    /// Product ID
    #[serde(rename = "productId")]
    #[serde(skip_serializing_if="Option::is_none")]
    pub product_id: Option<String>,

    /// Product Version
    #[serde(rename = "productVer")]
    #[serde(skip_serializing_if="Option::is_none")]
    pub product_ver: Option<String>,

    /// Hat UUID
    #[serde(rename = "uuid")]
    #[serde(skip_serializing_if="Option::is_none")]
    pub uuid: Option<String>,

    /// Hat Vendor
    #[serde(rename = "vendor")]
    #[serde(skip_serializing_if="Option::is_none")]
    pub vendor: Option<String>,

}

impl Hat {
    pub fn new() -> Hat {
        Hat {
            product: None,
            product_id: None,
            product_ver: None,
            uuid: None,
            vendor: None,
        }
    }
}

/// M-Bus device data as an XML document
#[derive(Debug, Clone, PartialEq, PartialOrd, Serialize, Deserialize)]

pub struct MbusData(String);

impl ::std::convert::From<String> for MbusData {
    fn from(x: String) -> Self {
        MbusData(x)
    }
}

impl ::std::convert::From<MbusData> for String {
    fn from(x: MbusData) -> Self {
        x.0
    }
}

impl ::std::ops::Deref for MbusData {
    type Target = String;
    fn deref(&self) -> &String {
        &self.0
    }
}

impl ::std::ops::DerefMut for MbusData {
    fn deref_mut(&mut self) -> &mut String {
        &mut self.0
    }
}


/// An XML list of slaves
#[derive(Debug, Clone, PartialEq, PartialOrd, Serialize, Deserialize)]

pub struct Slaves(String);

impl ::std::convert::From<String> for Slaves {
    fn from(x: String) -> Self {
        Slaves(x)
    }
}

impl ::std::convert::From<Slaves> for String {
    fn from(x: Slaves) -> Self {
        x.0
    }
}

impl ::std::ops::Deref for Slaves {
    type Target = String;
    fn deref(&self) -> &String {
        &self.0
    }
}

impl ::std::ops::DerefMut for Slaves {
    fn deref_mut(&mut self) -> &mut String {
        &mut self.0
    }
}


/// Some error text
#[derive(Debug, Clone, PartialEq, PartialOrd, Serialize, Deserialize)]

pub struct TextError(String);

impl ::std::convert::From<String> for TextError {
    fn from(x: String) -> Self {
        TextError(x)
    }
}

impl ::std::convert::From<TextError> for String {
    fn from(x: TextError) -> Self {
        x.0
    }
}

impl ::std::ops::Deref for TextError {
    type Target = String;
    fn deref(&self) -> &String {
        &self.0
    }
}

impl ::std::ops::DerefMut for TextError {
    fn deref_mut(&mut self) -> &mut String {
        &mut self.0
    }
}


/// A YAML file
#[derive(Debug, Clone, PartialEq, PartialOrd, Serialize, Deserialize)]

pub struct Yaml(String);

impl ::std::convert::From<String> for Yaml {
    fn from(x: String) -> Self {
        Yaml(x)
    }
}

impl ::std::convert::From<Yaml> for String {
    fn from(x: Yaml) -> Self {
        x.0
    }
}

impl ::std::ops::Deref for Yaml {
    type Target = String;
    fn deref(&self) -> &String {
        &self.0
    }
}

impl ::std::ops::DerefMut for Yaml {
    fn deref_mut(&mut self) -> &mut String {
        &mut self.0
    }
}

