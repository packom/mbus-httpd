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

use async_trait::async_trait;
use futures::{future, Stream, StreamExt, TryFutureExt, TryStreamExt};
use hyper::server::conn::Http;
use hyper::service::Service;
use log::info;
use openssl::ssl::SslAcceptorBuilder;
use std::future::Future;
use std::marker::PhantomData;
use std::net::SocketAddr;
use std::sync::{Arc, Mutex};
use std::task::{Context, Poll};
use swagger::{Has, XSpanIdString};
use swagger::auth::MakeAllowAllAuthenticator;
use swagger::EmptyContext;
use tokio::net::TcpListener;


#[cfg(not(any(target_os = "macos", target_os = "windows", target_os = "ios")))]
use openssl::ssl::{SslAcceptor, SslFiletype, SslMethod};

use mbus_api::models;

#[cfg(not(any(target_os = "macos", target_os = "windows", target_os = "ios")))]
/// Builds an SSL implementation for Simple HTTPS from some hard-coded file names
pub async fn create(addr: &str, ssl: Option<SslAcceptorBuilder>) {
    let addr = addr.parse().expect("Failed to parse bind address");

    let server = Server::new();

    let service = MakeService::new(server);

    let service = MakeAllowAllAuthenticator::new(service, "cosmo");

    let mut service =
        mbus_api::server::context::MakeAddContext::<_, EmptyContext>::new(
            service
        );

    match ssl {
        Some(ssl) => {
            let tls_acceptor = Arc::new(ssl.build());
            let mut tcp_listener = TcpListener::bind(&addr).await.unwrap();
            let mut incoming = tcp_listener.incoming();

            while let (Some(tcp), rest) = incoming.into_future().await {
                if let Ok(tcp) = tcp {
                    let addr = tcp.peer_addr().expect("Unable to get remote address");
                    let service = service.call(addr);
                    let tls_acceptor = Arc::clone(&tls_acceptor);

                    tokio::spawn(async move {
                        let tls = tokio_openssl::accept(&*tls_acceptor, tcp).await.map_err(|_| ())?;

                        let service = service.await.map_err(|_| ())?;

                        Http::new().serve_connection(tls, service).await.map_err(|_| ())
                    });
                }

                incoming = rest;
            }
        },
        None => hyper::server::Server::bind(&addr).serve(service).await.unwrap(),
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
    GetResponse,
    GetMultiResponse,
    HatResponse,
    HatOffResponse,
    HatOnResponse,
    MbusApiResponse,
    ScanResponse,
};
use mbus_api::server::MakeService;
use std::error::Error;
use swagger::ApiError;

mod http;

#[async_trait]
impl<C> Api<C> for Server<C> where C: Has<XSpanIdString> + Send + Sync
{
    async fn get(
        &self,
        device: String,
        baudrate: models::Baudrate,
        address: i32,
        _context: &C) -> Result<GetResponse, ApiError>
    {
        Ok(http::get(&device, &baudrate, &address))
    }

    async fn get_multi(
        &self,
        device: String,
        baudrate: models::Baudrate,
        address: i32,
        maxframes: i32,
        _context: &C) -> Result<GetMultiResponse, ApiError>
    {
        Ok(http::get_multi(&device, &baudrate, &address, &maxframes))
    }

    async fn hat(
        &self,
        _context: &C) -> Result<HatResponse, ApiError>
    {
        Ok(http::hat())
    }

    async fn hat_off(
        &self,
        _context: &C) -> Result<HatOffResponse, ApiError>
    {
        Ok(http::hat_off())
    }

    async fn hat_on(
        &self,
        _context: &C) -> Result<HatOnResponse, ApiError>
    {
        Ok(http::hat_on())
    }

    async fn mbus_api(
        &self,
        _context: &C) -> Result<MbusApiResponse, ApiError>
    {
        Ok(http::api())
    }

    async fn scan(
        &self,
        device: String,
        baudrate: models::Baudrate,
        _context: &C) -> Result<ScanResponse, ApiError>
    {
        Ok(http::scan(&device, &baudrate))
    }

}
