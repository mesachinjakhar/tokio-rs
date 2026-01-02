use tokio::sync::oneshot;

#[tokio::main(flavor = "current_thread")]
async fn main() {
    let (tx, rx) = oneshot::channel();

    tokio::spawn(async move {
        let res = some_computation().await;
        tx.send(res).unwrap();

    });


    let res = rx.await.unwrap();
    println!("result is: {}", res);
    
}

async fn some_computation() -> String {
    "represents the result of the computation".to_string()
}

