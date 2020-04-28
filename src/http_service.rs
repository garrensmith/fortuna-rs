use std::task::{Context, Poll};

use hyper::service::Service;
use std::future::Future;
use std::pin::Pin;
use std::sync::Arc;

use hyper::{Body, Request, Response, Server, Method, StatusCode};

use futures_util::future;

use crate::{JSEnv, FortunaIsolate};

const ROOT: &str = "/";

use ateles::{JsRequest, JsResponse};
use prost::Message;
use std::net::SocketAddr;
use hyper::server::conn::AddrIncoming;

pub mod ateles {
    tonic::include_proto!("ateles"); // The string specified here must match the proto package name
}

#[derive(Clone)]
pub struct Svc {
    isolate: Arc<FortunaIsolate>
}

impl Svc  {
    pub fn new() -> Svc {
        Svc {
            isolate: Arc::new(FortunaIsolate::new_clean())
        }
    }

    pub async fn handle_resp(&self, req: &Request<Body>) -> Result<Response<Body>, hyper::Error> {
        Ok(Response::new(Body::from("HELLO")))
    }
}

impl Service<Request<Body>> for Svc {
    type Response = Response<Body>;
    type Error = hyper::Error;
    type Future = future::BoxFuture<'static, Result<Self::Response, Self::Error>>;

    fn poll_ready(&mut self, _cx: &mut Context<'_>) -> Poll<Result<(), Self::Error>> {
        Ok(()).into()
    }

    fn call(&mut self, req: Request<Body>) -> Self::Future {
        let me = self.clone();
        let fut = async move {
            let resp = me.handle_resp(&req).await;
            resp
        };
        Box::pin(fut)
    }
}


pub struct MakeService {
    // js_env: JSEnv
}

impl MakeService {
   pub fn new () -> MakeService {
       // let js_env = JSEnv::new();
       MakeService {
           // js_env
       }
   }
}

impl<T> Service<T> for MakeService {
    type Response = Svc;
    type Error = std::io::Error;
    type Future = future::Ready<Result<Self::Response, Self::Error>>;

    fn poll_ready(&mut self, _cx: &mut Context<'_>) -> Poll<Result<(), Self::Error>> {
        Ok(()).into()
    }

    fn call(&mut self, _: T) -> Self::Future {
        // let svc = Svc::new(&self.js_env);
        let svc = Svc::new();
        future::ok(svc)
    }
}

pub fn create_server(addr: &SocketAddr) -> Server<AddrIncoming, MakeService> {
    Server::bind(&addr).serve(MakeService::new())
}
