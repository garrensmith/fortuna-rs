use tonic::{transport::Server, Request, Response, Status};
use ateles::{JSRequest, JSResponse};

pub mod ateles {
    tonic::include_proto!("ateles"); // The string specified here must match the proto package name
}