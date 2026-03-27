use std::default;

use lsp::lsp;
use serde::{Deserialize, Serialize};

pub mod lsp;

#[derive(Deserialize, Serialize, Default)]
pub struct Config {
    lazy: Option<bool>,
    public: Option<bool>,
    start_port: Option<u16>,
}

#[tokio::main]
async fn main() {
    lsp().await;
    println!("Hello, Doniaa");
}
