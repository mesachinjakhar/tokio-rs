use tokio::task;

#[tokio::main(flavor = "current_thread")]
async fn main() {
    // spwan and spawn_blocking return JoinHandler
    let handler = task::spawn(async {
        // this wont print because our main fn returns before this spawn  async task run
        println!("Hello from task");
    });

    handler.await; // now print works
}
