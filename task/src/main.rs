use tokio::time;

use tokio::task;

#[tokio::main(flavor = "current_thread")]
async fn main() {
    // spwan and spawn_blocking return JoinHandler
    let handler = task::spawn(async {
        // this wont print because our main fn returns before this spawn  async task run
        println!("Hello from task");
        return 5
    });

    let join = task::spawn(async {
        panic!("task panic here")
    });

    // handler return Result 
    let result = handler.await; 
    println!("Result: {}", result.unwrap()); // now print works

    assert!(join.await.is_err());

    let mut handles: Vec<_> = Vec::new();

    handles.push(tokio::spawn(async {
        time::sleep(time::Duration::from_secs(10)).await;
        true;
    }));

    handles.push(tokio::spawn(async {
        time::sleep(time::Duration::from_secs(10)).await
    }));

    for handle in &handles {
        handle.abort();
    }

    for handle in handles {
        assert!(handle.await.unwrap_err().is_cancelled()) // is cancelled return true if aborting is succeed  
    }
}
