use tokio::sync::oneshot;
use tokio::sync::mpsc;
use tokio::net::TcpStream;
use tokio::io::{self, AsyncWriteExt};
use Command::Increment;

enum Command {
    Increment,
}


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
    // let mut socket = TcpStream::connect("www.example.com:1234").await?;
    // let(tx, mut rx) = mpsc::channel(100);

    // for _ in 0..10 {
    //     let tx = tx.clone();

    //     tokio::spawn(async move {
    //         tx.send(&b"data to write"[..]).await.unwrap();
    //     });
    // }

    // drop(tx);

    // while let Some(res) = rx.recv().await {
    //     socket.write_all(res).await?;
    // }

    // Ok(())


    // let (cmd_tx, mut cmd_rx) = mpsc::channel::<(Command, oneshot::Sender<u64>)>(100);
    // tokio::spawn(async move {
    //     let mut counter: u64 = 0;

    //     while let Some((cmd, response)) = cmd_rx.recv().await {
    //         match cmd {
    //             Increment => {
    //                 let prev = counter;
    //                 counter += 1;
    //                 response.send(prev).unwrap();
    //             }
    //         }
    //     }
    // });

    // let mut join_handles = vec![];


    // for _ in 0..10 {
    //     let cmd_tx = cmd_tx.clone();

    //     join_handles.push(tokio::spawn(async move {
    //         let (resp_tx, resp_rx) = oneshot::channel();

    //         cmd_tx.send((Increment, resp_tx)).await.ok().unwrap();
    //         let res = resp_rx.await.unwrap();

    //         println!("previous value ={}", res);
    //     }))
    // }

    // for join_handle in join_handles.drain(..) {
    //     join_handle.await.unwrap();
    // }

    // Ok(())

    let (tx,rx) = oneshot::channel::<u32>();

    tokio::spawn(async move {
        drop(tx)
    });

    match rx.await {
        Ok(_) => panic!("this does not happen"),
        Err(_) => println!("the sender dropped")
    }

    Ok(())
}


// async fn some_computation2(i: u32) -> String {
//     format!("the result of computation {}", i)
// }

// async fn some_computation() -> String {
//     "represents the result of the computation".to_string()
// }
