use tokio::sync::oneshot;

#[tokio::main(flavor = "current_thread")]
async fn main() {
    let (tx, rx) = oneshot::channel();

    tokio::spawn(async move {
        let res = some_computation().await;
        println!("computation completed");
        tx.send(res).unwrap();
    });

    println!("awaiting res");
    let res = rx.await.unwrap();
    println!("result is: {}", res);

    // using join handler

    let join_handle = tokio::spawn(async move { some_computation().await });

    // Wait for the computation result
    let res = join_handle.await.unwrap();
}

async fn some_computation() -> String {
    "represents the result of the computation".to_string()
}
