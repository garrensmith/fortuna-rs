use fortuna::{create_server, init_v8};

async fn shutdown_signal() {
    // Wait for the CTRL+C signal
    tokio::signal::ctrl_c()
        .await
        .expect("failed to install CTRL+C signal handler");
}

#[tokio::main(core_threads = 6)]
async fn main() -> Result<(), Box<dyn std::error::Error>> {
    init_v8();
    let addr = "127.0.0.1:8444".parse().unwrap();
    let server = create_server(&addr);

    println!("Listening on http://{}", addr);

    // server.await?;

    let graceful = server.with_graceful_shutdown(shutdown_signal());

    // Run this server for... forever!
    if let Err(e) = graceful.await {
        eprintln!("server error: {}", e);
    }

    Ok(())
}
