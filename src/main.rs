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

//! Main binary entry point for openapi_client implementation.

#![allow(missing_docs)]

// Imports required by this file.
extern crate httpd_util;
extern crate hyper;
extern crate mbus_httpd;
extern crate native_tls;
extern crate openapi_client;
extern crate swagger;
extern crate tokio_proto;
extern crate tokio_tls;
#[macro_use]
extern crate log;

use httpd_util::{get_server_addr, https, init_app, ssl};
use hyper::server::Http;
use swagger::auth::AllowAllAuthenticator;
use swagger::EmptyContext;
use tokio_proto::TcpServer;

/// Create custom server, wire it to the autogenerated router,
/// and pass it to the web server.
fn main() {
    init_app(
        "mbus-httpd",
        "packom.net, mbus@packom.net",
        "An HTTP(S) microservice exposing M-Bus functionality\n(C) Copyright 2019  packom.net",
        vec![
            "[LIBMBUS_PATH] - Path to libmbus binaries",
            "[LIBMBUS_GET] - libmbus get binary",
            "[LIBMBUS_SCAN] - libmbus scan binary",
            "[LD_LIBRARY_PATH] - Path containing libmbus.so, used by libmbus binaries",
        ],
        mbus_httpd::get_env(),
    );

    let service_fn = openapi_client::server::context::NewAddContext::<_, EmptyContext>::new(
        AllowAllAuthenticator::new(mbus_httpd::NewService::new(), "cosmo"),
    );

    let addr = get_server_addr();
    // Start the server
    if https() {
        info!("Running server at https://{}", addr);
        let ssl = ssl().expect("Failed to load SSL keys");
        let builder: native_tls::TlsAcceptorBuilder =
            native_tls::backend::openssl::TlsAcceptorBuilderExt::from_openssl(ssl);
        let tls_acceptor = builder.build().expect("Failed to build TLS acceptor");
        TcpServer::new(
            tokio_tls::proto::Server::new(Http::new(), tls_acceptor),
            addr,
        )
        .serve(service_fn);
    } else {
        // Using HTTP
        info!("Running server at http://{}", addr);
        TcpServer::new(Http::new(), addr).serve(service_fn);
    }
}
