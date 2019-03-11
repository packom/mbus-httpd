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

//! Server implementation of openapi_client.

#![allow(unused_imports)]

use futures::{self, Future};
use chrono;
use std::collections::HashMap;
use std::marker::PhantomData;

use swagger;
use swagger::{Has, XSpanIdString};

use crate::http;

use openapi_client::{Api, ApiError,
                      ApiResponse,
                      GetResponse,
                      HatResponse,
                      HatOffResponse,
                      HatOnResponse,
                      ScanResponse
};
use openapi_client::models;

#[derive(Copy, Clone)]
pub struct Server<C> {
    marker: PhantomData<C>,
}

impl<C> Server<C> {
    pub fn new() -> Self {
        Server{marker: PhantomData}
    }
}

impl<C> Api<C> for Server<C> where C: Has<XSpanIdString>{


    fn api(&self, _context: &C) -> Box<Future<Item=ApiResponse, Error=ApiError>> {
        Box::new(futures::future::ok(http::api()))
    }


    fn get(&self, device: String, baudrate: models::Baudrate, address: i32, _context: &C) -> Box<Future<Item=GetResponse, Error=ApiError>> {
        Box::new(futures::future::ok(http::get(&device, &baudrate, &address)))
    }


    fn hat(&self, _context: &C) -> Box<Future<Item=HatResponse, Error=ApiError>> {
        Box::new(futures::future::ok(http::hat()))
    }


    fn hat_off(&self, _context: &C) -> Box<Future<Item=HatOffResponse, Error=ApiError>> {
        Box::new(futures::future::ok(http::hat_off()))
    }


    fn hat_on(&self, _context: &C) -> Box<Future<Item=HatOnResponse, Error=ApiError>> {
        Box::new(futures::future::ok(http::hat_on()))
    }


    fn scan(&self, device: String, baudrate: models::Baudrate, _context: &C) -> Box<Future<Item=ScanResponse, Error=ApiError>> {
        Box::new(futures::future::ok(http::scan(&device, &baudrate)))
    }

}
