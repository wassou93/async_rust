
#[tokio::main]
async fn main() {
    println!("Hello sync world!");
    hello().await;
}

async fn hello() {
    println!("Hello, async world!");
}
