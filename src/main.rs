use std::net::SocketAddr;
use hyper::{Body, Request, Response, Server, Method, StatusCode};
use hyper::service::{make_service_fn, service_fn};

use ateles::{JsRequest, JsResponse};
use prost::Message;
use fortuna::{FortunaIsolate, init, JSEnv};

pub mod ateles {
    tonic::include_proto!("ateles"); // The string specified here must match the proto package name
}

use std::{io, fs};
use std::fs::{read_dir, write, File};
use std::path::Path;

async fn routes(req: Request<Body>) -> Result<Response<Body>, hyper::Error>  {

    match (req.method(), req.uri().path()) {
        (&Method::GET, "/") => {
            Ok(Response::new(Body::from("HELLO")))
        },
        (&Method::GET, "/Health") => {
            Ok(Response::new(Body::from("OK")))
        },
        (&Method::POST, "/Ateles/Execute") => {
            let full_body = hyper::body::to_bytes(req.into_body()).await?;
            let out = JsRequest::decode(full_body.clone()).unwrap();
            println!("RECEIVED {:?} {:?}", full_body, out.action);
            let js_resp = JsResponse{
                status: 0,
                result: "2".to_string()
            };
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


#[tokio::main]
async fn main() {
    // println!("hh {:?}", JS_CODE);
    // read_file();
    // println!("HEE {:?}", hello::rewrite_anon_fun_code);
    fortuna::init_v8();
    let js_env = JSEnv::new();
    let isolate = js_env.create_isolate();
    // let port = 8444;
    // println!("Starting on {}", port);
    //
    // let addr = SocketAddr::from(([127, 0, 0, 1], port));
    //
    // let service = make_service_fn(|_| async { Ok::<_, hyper::Error>(service_fn(routes)) });
    //
    // let server = Server::bind(&addr).serve(service);
    //
    // // Run this server for... forever!
    // if let Err(e) = server.await {
    //     eprintln!("server error: {}", e);
    // }

}
