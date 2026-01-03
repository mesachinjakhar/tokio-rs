use tokio::signal;
use tokio::signal::unix::{signal, SignalKind};

#[tokio::main]
async fn main() -> Result<(), Box<dyn std::error::Error>>{
    // signal::ctrl_c().await;
    // println!("ctrl-c recieved");
    // Ok(())

    let mut stream = signal(SignalKind::hangup())?;

    loop {
        stream.recv().await;
        println!("got signal HUP");
    }
}

