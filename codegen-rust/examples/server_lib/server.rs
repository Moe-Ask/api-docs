//! Server implementation of openapi_client.

#![allow(unused_imports)]

use futures::{self, Future};
use chrono;
use std::collections::HashMap;
use std::marker::PhantomData;
use swagger;
use swagger::{Has, XSpanIdString};

use openapi_client::{Api, ApiError,
                      UserGetResponse,
                      UserLoginPostResponse
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

    /// 获取用户信息
    fn user_get(&self, context: &C) -> Box<dyn Future<Item=UserGetResponse, Error=ApiError>> {
        let context = context.clone();
        println!("user_get() - X-Span-ID: {:?}", context.get().0.clone());
        Box::new(futures::failed("Generic failure".into()))
    }


    fn user_login_post(&self, context: &C) -> Box<dyn Future<Item=UserLoginPostResponse, Error=ApiError>> {
        let context = context.clone();
        println!("user_login_post() - X-Span-ID: {:?}", context.get().0.clone());
        Box::new(futures::failed("Generic failure".into()))
    }

}
