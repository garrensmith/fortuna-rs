use std::task::{Context, Poll};

use hyper::service::Service;
use std::future::Future;
use std::pin::Pin;
use std::sync::{Arc, Mutex};

use hyper::{Body, Request, Response, Server, Method, StatusCode};

use futures_util::future;

use crate::{JSEnv, FortunaIsolate};

use ateles::{JsRequest, JsResponse, js_request};
use prost::Message;
use std::net::SocketAddr;
use hyper::server::conn::AddrIncoming;

use crossbeam::crossbeam_channel::{unbounded, Receiver, Sender};
use std::thread;
use crate::js_server::{create_js_env, JSClient, Command, Ops};

pub mod ateles {
    tonic::include_proto!("ateles"); // The string specified here must match the proto package name
}

impl From<ateles::JsRequest> for Command {
    fn from(js_request: JsRequest) -> Self {
        let op = match js_request.action {
            0 => Ops::REWRITE,
            1 => Ops::EVAL,
            2 => Ops::CALL,
            _ => Ops::EXIT
        };
        Command {
            operation: op,
            payload: js_request.script,
            args: js_request.args
        }
    }
}

#[derive(Clone)]
pub struct Svc {
    // isolate: Arc<Mutex<FortunaIsolate>>
    // isolate: FortunaIsolate
    js_client: JSClient
}

impl Svc  {
    // pub fn new(js_client: &JSClient) -> Svc {
    //     Svc {
    //         js_client.clon
    //         // isolate: Arc::new(Mutex::new(FortunaIsolate::new_clean()))
    //     }
    // }

    pub async fn handle_resp(&mut self, req: Request<Body>) -> Result<Response<Body>, hyper::Error> {
        match (req.method(), req.uri().path()) {
            (&Method::GET, "/") => {
                // self.js_client.tx.send("hello".to_string());
                // let resp = self.js_client.rx.recv().unwrap();
                // let fmt = format!("hello with {:}", resp);
                // Ok(Response::new(Body::from(fmt)))
                Ok(Response::new(Body::from("HELLO Ateles on Rust with V8!!!!")))
            },
            (&Method::GET, "/Health") => {
                Ok(Response::new(Body::from("OK")))
            },
            (&Method::POST, "/Ateles/Execute") => {
                let full_body = hyper::body::to_bytes(req.into_body()).await?;
                let js_request = JsRequest::decode(full_body.clone()).unwrap();
                println!("RECEIVED {:?} {:?}", full_body, js_request.action);
                // let mut isolate = self.isolate.lock().unwrap();
                let resp = self.js_client.run(js_request.into());
                println!("eval {:?}", resp);
                let js_resp = JsResponse {
                    status: 0,
                    result: resp
                };
                // let js_resp = match js_request.action {
                //     1 => {
                //         // let result = isolate.eval(&js_request.script.as_str(), &js_request.args.as_slice());
                //     },
                //     _ => {
                //         JsResponse {
                //             status: 1,
                //             result: "Unsupported".to_string()
                //         }
                //     }
                // };
                // let instruction: Instruction = js_request.into();
                // let result = self.isolate.process(&instruction);
                let mut resp: Vec<u8> = Vec::new();
                js_resp.encode(&mut resp).unwrap();
                // JsReponse::encode(&resp);
                Ok(Response::new(Body::from(resp)))
            },
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
            let resp = me.handle_resp(req).await;
            resp
        };
        Box::pin(fut)
    }
}


pub struct MakeService {

}

impl MakeService {
   pub fn new () -> MakeService {
       MakeService {}
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
            js_client: create_js_env()
        };
        future::ok(svc)
    }
}

pub fn create_server(addr: &SocketAddr) -> Server<AddrIncoming, MakeService> {
    Server::bind(&addr).serve(MakeService::new())
}
