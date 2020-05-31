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

impl<C> Api<C> for Server<C> where C: Has<XSpanIdString>{
    fn get(
        &self,
        device: String,
        baudrate: models::Baudrate,
        address: i32,
        context: &C) -> Box<dyn Future<Item=GetResponse, Error=ApiError> + Send>
    {
        let context = context.clone();
        info!("get(\"{}\", {:?}, {}) - X-Span-ID: {:?}", device, baudrate, address, context.get().0.clone());
        Box::new(future::err("Generic failure".into()))
    }

    fn hat(
        &self,
        context: &C) -> Box<dyn Future<Item=HatResponse, Error=ApiError> + Send>
    {
        let context = context.clone();
        info!("hat() - X-Span-ID: {:?}", context.get().0.clone());
        Box::new(future::err("Generic failure".into()))
    }

    fn hat_off(
        &self,
        context: &C) -> Box<dyn Future<Item=HatOffResponse, Error=ApiError> + Send>
    {
        let context = context.clone();
        info!("hat_off() - X-Span-ID: {:?}", context.get().0.clone());
        Box::new(future::err("Generic failure".into()))
    }

    fn hat_on(
        &self,
        context: &C) -> Box<dyn Future<Item=HatOnResponse, Error=ApiError> + Send>
    {
        let context = context.clone();
        info!("hat_on() - X-Span-ID: {:?}", context.get().0.clone());
        Box::new(future::err("Generic failure".into()))
    }

    fn mbus_api(
        &self,
        context: &C) -> Box<dyn Future<Item=MbusApiResponse, Error=ApiError> + Send>
    {
        let context = context.clone();
        info!("mbus_api() - X-Span-ID: {:?}", context.get().0.clone());
        Box::new(future::err("Generic failure".into()))
    }

    fn scan(
        &self,
        device: String,
        baudrate: models::Baudrate,
        context: &C) -> Box<dyn Future<Item=ScanResponse, Error=ApiError> + Send>
    {
        let context = context.clone();
        info!("scan(\"{}\", {:?}) - X-Span-ID: {:?}", device, baudrate, context.get().0.clone());
        Box::new(future::err("Generic failure".into()))
    }

}
