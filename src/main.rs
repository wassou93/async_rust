use futures::executor::block_on;

fn main() {
    println!("Hello sync world!");
    block_on(do_it());
}

async fn do_it() {
    println!("Hello, async world!");
}
