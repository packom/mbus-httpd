//! Main library entry point for mbus_api implementation.

// Imports required by server library.
extern crate chrono;
extern crate futures;
extern crate mbus_api;
extern crate swagger;
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
    error_chain! {}
}

pub use self::errors::*;
pub use crate::http::get_env;
use hyper;
use std::clone::Clone;
use std::io;
use std::marker::PhantomData;
//use mbus_api;
use swagger::{Has, XSpanIdString};
//use swagger::auth::Authorization;

pub struct NewService<C> {
    marker: PhantomData<C>,
}

impl<C> NewService<C> {
    pub fn new() -> Self {
        NewService {
            marker: PhantomData,
        }
    }
}

impl<C> hyper::server::NewService for NewService<C>
where
    C: Has<XSpanIdString> + Clone + 'static,
{
    type Request = (hyper::Request, C);
    type Response = hyper::Response;
    type Error = hyper::Error;
    type Instance = mbus_api::server::Service<server::Server<C>, C>;

    /// Instantiate a new server.
    fn new_service(&self) -> io::Result<Self::Instance> {
        Ok(mbus_api::server::Service::new(server::Server::new()))
    }
}
