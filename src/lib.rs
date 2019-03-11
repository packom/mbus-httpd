//! Main library entry point for openapi_client implementation.

// Imports required by server library.
extern crate openapi_client;
extern crate swagger;
extern crate futures;
extern crate chrono;
#[macro_use]
extern crate error_chain;
#[macro_use]
extern crate log;
#[macro_use]
extern crate lazy_static;
extern crate sysfs_gpio;

mod server;

mod http;

mod errors {
    error_chain!{}
}

pub use crate::http::get_env;
pub use self::errors::*;
use std::io;
use std::clone::Clone;
use std::marker::PhantomData;
use hyper;
//use openapi_client;
use swagger::{Has, XSpanIdString};
//use swagger::auth::Authorization;

pub struct NewService<C>{
    marker: PhantomData<C>
}

impl<C> NewService<C>{
    pub fn new() -> Self {
        NewService{marker:PhantomData}
    }
}

impl<C> hyper::server::NewService for NewService<C> where C: Has<XSpanIdString>  + Clone + 'static {
    type Request = (hyper::Request, C);
    type Response = hyper::Response;
    type Error = hyper::Error;
    type Instance = openapi_client::server::Service<server::Server<C>, C>;

    /// Instantiate a new server.
    fn new_service(&self) -> io::Result<Self::Instance> {
        Ok(openapi_client::server::Service::new(server::Server::new()))
    }
}
