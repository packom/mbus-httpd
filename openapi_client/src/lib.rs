#![allow(missing_docs, trivial_casts, unused_variables, unused_mut, unused_imports, unused_extern_crates, non_camel_case_types)]
extern crate serde;
#[macro_use]
extern crate serde_derive;
extern crate serde_json;


extern crate futures;
extern crate chrono;
#[macro_use]
extern crate lazy_static;
#[macro_use]
extern crate log;

// Logically this should be in the client and server modules, but rust doesn't allow `macro_use` from a module.
#[cfg(any(feature = "client", feature = "server"))]
#[macro_use]
extern crate hyper;

extern crate swagger;

#[macro_use]
extern crate url;

use futures::Stream;
use std::io::Error;

#[allow(unused_imports)]
use std::collections::HashMap;

pub use futures::Future;

#[cfg(any(feature = "client", feature = "server"))]
mod mimetypes;

pub use swagger::{ApiError, ContextWrapper};

pub const BASE_PATH: &'static str = "";
pub const API_VERSION: &'static str = "0.1.0";


#[derive(Debug, PartialEq)]
pub enum ApiResponse {
    /// OK
    OK ( String ) ,
    /// Not found
    NotFound ( String ) ,
}

#[derive(Debug, PartialEq)]
pub enum GetResponse {
    /// OK
    OK ( String ) ,
    /// Bad request
    BadRequest ( String ) ,
    /// Not found (or M-Bus HTTPD is unauthorized to access it, or to change baud rate to that specified, etc)
    NotFound ( String ) ,
}

#[derive(Debug, PartialEq)]
pub enum HatResponse {
    /// OK
    OK ( models::Hat ) ,
    /// Not found
    NotFound ( String ) ,
}

#[derive(Debug, PartialEq)]
pub enum HatOffResponse {
    /// OK
    OK ,
    /// Not found
    NotFound ( String ) ,
}

#[derive(Debug, PartialEq)]
pub enum HatOnResponse {
    /// OK
    OK ,
    /// Not found
    NotFound ( String ) ,
}

#[derive(Debug, PartialEq)]
pub enum ScanResponse {
    /// OK
    OK ( String ) ,
    /// Bad request
    BadRequest ( String ) ,
    /// Not found (e.g. device not found, or M-Bus HTTPD is unauthorized to access it, or to change baud rate to that specified, device not responding etc)
    NotFound ( String ) ,
}


/// API
pub trait Api<C> {


    fn api(&self, context: &C) -> Box<Future<Item=ApiResponse, Error=ApiError>>;


    fn get(&self, device: String, baudrate: models::Baudrate, address: i32, context: &C) -> Box<Future<Item=GetResponse, Error=ApiError>>;


    fn hat(&self, context: &C) -> Box<Future<Item=HatResponse, Error=ApiError>>;


    fn hat_off(&self, context: &C) -> Box<Future<Item=HatOffResponse, Error=ApiError>>;


    fn hat_on(&self, context: &C) -> Box<Future<Item=HatOnResponse, Error=ApiError>>;


    fn scan(&self, device: String, baudrate: models::Baudrate, context: &C) -> Box<Future<Item=ScanResponse, Error=ApiError>>;

}

/// API without a `Context`
pub trait ApiNoContext {


    fn api(&self) -> Box<Future<Item=ApiResponse, Error=ApiError>>;


    fn get(&self, device: String, baudrate: models::Baudrate, address: i32) -> Box<Future<Item=GetResponse, Error=ApiError>>;


    fn hat(&self) -> Box<Future<Item=HatResponse, Error=ApiError>>;


    fn hat_off(&self) -> Box<Future<Item=HatOffResponse, Error=ApiError>>;


    fn hat_on(&self) -> Box<Future<Item=HatOnResponse, Error=ApiError>>;


    fn scan(&self, device: String, baudrate: models::Baudrate) -> Box<Future<Item=ScanResponse, Error=ApiError>>;

}

/// Trait to extend an API to make it easy to bind it to a context.
pub trait ContextWrapperExt<'a, C> where Self: Sized {
    /// Binds this API to a context.
    fn with_context(self: &'a Self, context: C) -> ContextWrapper<'a, Self, C>;
}

impl<'a, T: Api<C> + Sized, C> ContextWrapperExt<'a, C> for T {
    fn with_context(self: &'a T, context: C) -> ContextWrapper<'a, T, C> {
         ContextWrapper::<T, C>::new(self, context)
    }
}

impl<'a, T: Api<C>, C> ApiNoContext for ContextWrapper<'a, T, C> {


    fn api(&self) -> Box<Future<Item=ApiResponse, Error=ApiError>> {
        self.api().api(&self.context())
    }


    fn get(&self, device: String, baudrate: models::Baudrate, address: i32) -> Box<Future<Item=GetResponse, Error=ApiError>> {
        self.api().get(device, baudrate, address, &self.context())
    }


    fn hat(&self) -> Box<Future<Item=HatResponse, Error=ApiError>> {
        self.api().hat(&self.context())
    }


    fn hat_off(&self) -> Box<Future<Item=HatOffResponse, Error=ApiError>> {
        self.api().hat_off(&self.context())
    }


    fn hat_on(&self) -> Box<Future<Item=HatOnResponse, Error=ApiError>> {
        self.api().hat_on(&self.context())
    }


    fn scan(&self, device: String, baudrate: models::Baudrate) -> Box<Future<Item=ScanResponse, Error=ApiError>> {
        self.api().scan(device, baudrate, &self.context())
    }

}

#[cfg(feature = "client")]
pub mod client;

// Re-export Client as a top-level name
#[cfg(feature = "client")]
pub use self::client::Client;

#[cfg(feature = "server")]
pub mod server;

// Re-export router() as a top-level name
#[cfg(feature = "server")]
pub use self::server::Service;

pub mod models;
