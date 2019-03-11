//! Server implementation of openapi_client.

#![allow(unused_imports)]

use futures::{self, Future};
use chrono;
use std::collections::HashMap;
use std::marker::PhantomData;

use swagger;
use swagger::{Has, XSpanIdString};

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


    fn api(&self, context: &C) -> Box<Future<Item=ApiResponse, Error=ApiError>> {
        let context = context.clone();
        println!("api() - X-Span-ID: {:?}", context.get().0.clone());
        Box::new(futures::failed("Generic failure".into()))
    }


    fn get(&self, device: String, baudrate: models::Baudrate, address: i32, context: &C) -> Box<Future<Item=GetResponse, Error=ApiError>> {
        let context = context.clone();
        println!("get(\"{}\", {:?}, {}) - X-Span-ID: {:?}", device, baudrate, address, context.get().0.clone());
        Box::new(futures::failed("Generic failure".into()))
    }


    fn hat(&self, context: &C) -> Box<Future<Item=HatResponse, Error=ApiError>> {
        let context = context.clone();
        println!("hat() - X-Span-ID: {:?}", context.get().0.clone());
        Box::new(futures::failed("Generic failure".into()))
    }


    fn hat_off(&self, context: &C) -> Box<Future<Item=HatOffResponse, Error=ApiError>> {
        let context = context.clone();
        println!("hat_off() - X-Span-ID: {:?}", context.get().0.clone());
        Box::new(futures::failed("Generic failure".into()))
    }


    fn hat_on(&self, context: &C) -> Box<Future<Item=HatOnResponse, Error=ApiError>> {
        let context = context.clone();
        println!("hat_on() - X-Span-ID: {:?}", context.get().0.clone());
        Box::new(futures::failed("Generic failure".into()))
    }


    fn scan(&self, device: String, baudrate: models::Baudrate, context: &C) -> Box<Future<Item=ScanResponse, Error=ApiError>> {
        let context = context.clone();
        println!("scan(\"{}\", {:?}) - X-Span-ID: {:?}", device, baudrate, context.get().0.clone());
        Box::new(futures::failed("Generic failure".into()))
    }

}
