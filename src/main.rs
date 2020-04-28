#![deny(warnings)]

use std::task::{Context, Poll};

use futures_util::future;
use hyper::service::Service;
use hyper::{Body, Request, Response};
use std::sync::Arc;
use fortuna::create_server;


// const ROOT: &str = "/";

#[derive(Debug, Clone)]
pub struct Svc {
    val: Arc<u32>
}

impl Svc {
    async fn test(&self) -> Result<Response<Body>, hyper::Error> {
        Ok(Response::new(Body::from("hello")))
    }
}

impl Service<Request<Body>> for Svc {
    type Response = Response<Body>;
    type Error = hyper::Error;
    type Future = future::BoxFuture<'static, Result<Self::Response, Self::Error>>;

    fn poll_ready(&mut self, _cx: &mut Context<'_>) -> Poll<Result<(), Self::Error>> {
        Ok(()).into()
    }

    fn call(&mut self, _req: Request<Body>) -> Self::Future {
        let me = self.clone();
        let fut = async move {
             let resp = me.test().await;
            resp
        };
        Box::pin(fut)
    }
}

pub struct MakeSvc;

impl<T> Service<T> for MakeSvc {
    type Response = Svc;
    type Error = std::io::Error;
    type Future = future::Ready<Result<Self::Response, Self::Error>>;

    fn poll_ready(&mut self, _cx: &mut Context<'_>) -> Poll<Result<(), Self::Error>> {
        Ok(()).into()
    }

    fn call(&mut self, _: T) -> Self::Future {
        future::ok(Svc{val: Arc::new(1)})
    }
}

#[tokio::main]
async fn main() -> Result<(), Box<dyn std::error::Error>> {
    fortuna::init_v8();
    // pretty_env_logger::init();

    let addr = "127.0.0.1:1337".parse().unwrap();

    // let server = Server::bind(&addr).serve(MakeSvc);
    let server = create_server(&addr);

    println!("Listening on http://{}", addr);

    server.await?;

    Ok(())
}
