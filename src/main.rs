use fortuna::{create_server, init_v8};

#[tokio::main]
async fn main() -> Result<(), Box<dyn std::error::Error>> {
    init_v8();
    // pretty_env_logger::init();

    let addr = "127.0.0.1:8444".parse().unwrap();
    let server = create_server(&addr);

    println!("Listening on http://{}", addr);

    server.await?;

    Ok(())
}
