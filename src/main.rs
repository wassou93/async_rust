enum Message {
    Tick(i32),
}

async fn sender1(tx: tokio::sync::mpsc::Sender<Message>) {
    loop {
        tx.send(Message::Tick(1)).await.unwrap();
        tokio::time::sleep(tokio::time::Duration::from_secs(1)).await;
    }
}

async fn sender2(tx: tokio::sync::mpsc::Sender<Message>) {
    loop {
        tx.send(Message::Tick(2)).await.unwrap();
        tokio::time::sleep(tokio::time::Duration::from_secs(1)).await;
    }
}

async fn receiver(mut rx: tokio::sync::mpsc::Receiver<Message>) {
    while let Some(message) = rx.recv().await {
        match message {
            Message::Tick(id) => println!("Tick({id})"),
        }
    }
}

#[tokio::main]
async fn main() {
    let (tx, rx) = tokio::sync::mpsc::channel::<Message>(100);
    tokio::spawn(sender1(tx.clone()));
    tokio::spawn(sender2(tx));
    receiver(rx).await;
}
