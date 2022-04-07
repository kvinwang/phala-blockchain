use log::info;

use pink_sidevm as sidevm;
use sidevm::{logger::Logger, net};

#[sidevm::main]
async fn main() {
    Logger::with_max_level(log::Level::Debug).init();
    // ocall::enable_ocall_trace(true).unwrap();

    let address = "127.0.0.1:9999";

    info!("Listening on {}", address);
    let listener = net::TcpListener::listen(address).await.unwrap();
    loop {
        info!("Waiting next connection...");
        let _stream = listener.accept().await.unwrap();
    }
}
