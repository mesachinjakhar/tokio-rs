use tokio::sync::oneshot;
use tokio::sync::mpsc;
use tokio::net::TcpStream;
use tokio::io::{self, AsyncWriteExt};


#[tokio::main]
async fn main() -> io::Result<()> {
    // let (tx, rx) = oneshot::channel();

    // tokio::spawn(async move {
    //     let res = some_computation().await;
    //     println!("computation completed");
    //     tx.send(res).unwrap();
    // });

    // println!("awaiting res");
    // let res = rx.await.unwrap();
    // println!("result is: {}", res);

    // // using join handler

    // let join_handle = tokio::spawn(async move { some_computation().await });

    // // Wait for the computation result
    // let res = join_handle.await.unwrap();

    // //mpsc
    // // used for multiper producer single consumer
    // // we can produce message from either 1 task or many other different tasks

    // let (tx, mut rx) = mpsc::channel(100);

    // tokio::spawn(async move {
    //     for i in 0..10 {
    //         let res = some_computation2(i).await;
    //         tx.send(res).await.unwrap();
    //     }
    // });

    // while let Some(res) = rx.recv().await {
    //     println!("got = {}", res);
    // }




    // message passing 
    let mut socket = TcpStream::connect("www.example.com:1234").await?;
    let(tx, mut rx) = mpsc::channel(100);

    for _ in 0..10 {
        let tx = tx.clone();

        tokio::spawn(async move {
            tx.send(&b"data to write"[..]).await.unwrap();
        });
    }

    drop(tx);

    while let Some(res) = rx.recv().await {
        socket.write_all(res).await?;
    }

    Ok(())
}


async fn some_computation2(i: u32) -> String {
    format!("the result of computation {}", i)
}

async fn some_computation() -> String {
    "represents the result of the computation".to_string()
}
