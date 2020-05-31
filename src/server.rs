//
//  mbus-httpd - An HTTP microservice exposing M-Bus Functionality
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

//! Main library entry point for mbus_api implementation.

#![allow(unused_imports)]

mod errors {
    error_chain::error_chain!{}
}

pub use self::errors::*;

use chrono;
use futures::{future, Future, Stream};
use hyper::server::conn::Http;
use hyper::service::MakeService as _;
use log::info;
use openssl::ssl::SslAcceptorBuilder;
use std::marker::PhantomData;
use std::net::SocketAddr;
use std::sync::{Arc, Mutex};
use swagger;
use swagger::{Has, XSpanIdString};
use swagger::auth::MakeAllowAllAuthenticator;
use swagger::EmptyContext;
use tokio::net::TcpListener;


#[cfg(not(any(target_os = "macos", target_os = "windows", target_os = "ios")))]
use tokio_openssl::SslAcceptorExt;
#[cfg(not(any(target_os = "macos", target_os = "windows", target_os = "ios")))]
use openssl::ssl::{SslAcceptor, SslFiletype, SslMethod};

use mbus_api::models;

#[cfg(not(any(target_os = "macos", target_os = "windows", target_os = "ios")))]
/// Builds an SSL implementation for Simple HTTPS from some hard-coded file names
pub fn create(addr: &str, ssl: Option<SslAcceptorBuilder>) -> Box<dyn Future<Item = (), Error = ()> + Send> {
    let addr = addr.parse().expect("Failed to parse bind address");

    let server = Server::new();

    let service_fn = MakeService::new(server);

    let service_fn = MakeAllowAllAuthenticator::new(service_fn, "cosmo");

    let service_fn =
        mbus_api::server::context::MakeAddContext::<_, EmptyContext>::new(
            service_fn
        );

    match ssl {
        Some(ssl) => {
            let tls_acceptor = ssl.build();
            let service_fn = Arc::new(Mutex::new(service_fn));
            let tls_listener = TcpListener::bind(&addr).unwrap().incoming().for_each(move |tcp| {
                let addr = tcp.peer_addr().expect("Unable to get remote address");

                let service_fn = service_fn.clone();

                hyper::rt::spawn(tls_acceptor.accept_async(tcp).map_err(|_| ()).and_then(move |tls| {
                    let ms = {
                        let mut service_fn = service_fn.lock().unwrap();
                        service_fn.make_service(&addr)
                    };

                    ms.and_then(move |service| {
                        Http::new().serve_connection(tls, service)
                    }).map_err(|_| ())
                }));

                Ok(())
            }).map_err(|_| ());

            Box::new(tls_listener)
        },
        None => Box::new(hyper::server::Server::bind(&addr).serve(service_fn).map_err(|e| panic!("{:?}", e))),
    }
}

#[derive(Copy, Clone)]
pub struct Server<C> {
    marker: PhantomData<C>,
}

impl<C> Server<C> {
    pub fn new() -> Self {
        Server{marker: PhantomData}
    }
}


use mbus_api::{
    Api,
    ApiError,
    GetResponse,
    HatResponse,
    HatOffResponse,
    HatOnResponse,
    MbusApiResponse,
    ScanResponse,
};
use mbus_api::server::MakeService;

mod http;

impl<C> Api<C> for Server<C> where C: Has<XSpanIdString>{
    fn get(
        &self,
        device: String,
        baudrate: models::Baudrate,
        address: i32,
        _context: &C) -> Box<dyn Future<Item=GetResponse, Error=ApiError> + Send>
    {
        Box::new(futures::future::ok(http::get(&device, &baudrate, &address)))
    }

    fn hat(
        &self,
        _context: &C) -> Box<dyn Future<Item=HatResponse, Error=ApiError> + Send>
    {
        Box::new(futures::future::ok(http::hat()))
    }

    fn hat_off(
        &self,
        _context: &C) -> Box<dyn Future<Item=HatOffResponse, Error=ApiError> + Send>
    {
        Box::new(futures::future::ok(http::hat_off()))
    }

    fn hat_on(
        &self,
        _context: &C) -> Box<dyn Future<Item=HatOnResponse, Error=ApiError> + Send>
    {
        Box::new(futures::future::ok(http::hat_on()))
    }

    fn mbus_api(
        &self,
        _context: &C) -> Box<dyn Future<Item=MbusApiResponse, Error=ApiError> + Send>
    {
        Box::new(futures::future::ok(http::api()))
    }

    fn scan(
        &self,
        device: String,
        baudrate: models::Baudrate,
        _context: &C) -> Box<dyn Future<Item=ScanResponse, Error=ApiError> + Send>
    {
        Box::new(futures::future::ok(http::scan(&device, &baudrate)))
    }

}
