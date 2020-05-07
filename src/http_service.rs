use std::task::{Context, Poll};

use hyper::service::Service;

use hyper::{Body, Method, Request, Response, Server, StatusCode};

use futures_util::future;

use ateles::{JsRequest, JsResponse};
use hyper::server::conn::AddrIncoming;
use prost::Message;
use std::net::SocketAddr;

use crate::js_server::{create_js_env, Command, JSClient, Ops};
use crate::JSEnv;
use std::time::Instant;

pub mod ateles {
    tonic::include_proto!("ateles"); // The string specified here must match the proto package name
}

impl From<ateles::JsRequest> for Command {
    fn from(js_request: JsRequest) -> Self {
        let op = match js_request.action {
            0 => Ops::REWRITE,
            1 => Ops::EVAL,
            2 => Ops::CALL,
            _ => Ops::EXIT,
        };
        Command {
            operation: op,
            payload: js_request.script,
            args: js_request.args,
        }
    }
}

#[derive(Clone)]
pub struct Svc {
    js_client: JSClient,
}

impl Svc {
    pub async fn handle_resp(
        &mut self,
        req: Request<Body>,
    ) -> Result<Response<Body>, hyper::Error> {
        // println!("req {:?}", req.uri().path());
        match (req.method(), req.uri().path()) {
            (&Method::GET, "/") => {
                // println!("hello");
                Ok(Response::new(Body::from(
                "HELLO Ateles on Rust with V8!!!!",
            )))},
            (&Method::GET, "/Health") => Ok(Response::new(Body::from("OK"))),
            (&Method::POST, "/Ateles/Execute") => {
                let start = Instant::now();

                let full_body = hyper::body::to_bytes(req.into_body()).await?;
                // println!("body {:?}", full_body);
                let js_request = JsRequest::decode(full_body).unwrap();
                let cmd: Command = js_request.clone().into();
                let resp = self.js_client.run(js_request.into());
                let js_resp = JsResponse {
                    status: 0,
                    result: resp,
                };

                let mut resp: Vec<u8> = Vec::new();
                js_resp.encode(&mut resp).unwrap();
                println!("request {:?} took {:?}", cmd.operation, start.elapsed());
                Ok(Response::new(Body::from(resp)))
            }
            _ => {
                let mut not_found = Response::default();
                *not_found.status_mut() = StatusCode::NOT_FOUND;
                Ok(not_found)
            }
        }
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
        let mut me = self.clone();
        let fut = async move {
            me.handle_resp(req).await
        };
        Box::pin(fut)
    }
}

pub struct MakeService {
    js_env: JSEnv
}

impl MakeService {
    pub fn new() -> MakeService {

        MakeService {
            js_env: JSEnv::new()
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
        let svc = Svc {
            js_client: create_js_env(&self.js_env),
        };
        future::ok(svc)
    }
}

pub fn create_server(addr: &SocketAddr) -> Server<AddrIncoming, MakeService> {
    Server::bind(&addr).serve(MakeService::new())
}
