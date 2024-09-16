use tokio::task::JoinSet;


#[tokio::main]
async fn main() {
    println!("Hello from async!");

    // Using tokio macro
    let (double_two, double_five) = tokio::join!(double(2), double(5));
    println!("Double of 2: {}", double_two);
    println!("Double of 5: {}", double_five);

    // Using futures 
    let futures = vec![double(2), double(5)];
    let result = futures::future::join_all(futures).await;
    println!("{:?}", result);

    // Using tokio JoinSet - unordered result and faster
    let mut set = JoinSet::new();
    for i in 0..10 {
        set.spawn(double(i));
    }

    while let Some(res) = set.join_next().await {
        println!("{res:?}");
    }

}

async fn double(number: i32) -> i32 {
    number * 2
}