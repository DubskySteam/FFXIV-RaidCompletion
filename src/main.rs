#![allow(dead_code)]
#![allow(unused_imports)]

mod player;
mod fetch;

#[tokio::main]
async fn main() {
    fetch::fetch_data().await;
}
