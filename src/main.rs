// main.rs
mod domain;
mod application;
mod infrastructure;
mod presentation;

use infrastructure::kernel::Kernel; 
use tracing::error;

#[tokio::main]
async fn main() {
    // 1. Bootstrap Framework
    if let Err(e) = Kernel::bootstrap().await {
        eprintln!("Framework boot failed: {:?}", e);
        std::process::exit(1);
    }

    // 2. Start Server
    if let Err(e) = Kernel::start().await {
        error!("Server crashed: {:?}", e);
        std::process::exit(1);
    }
}