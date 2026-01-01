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
}
