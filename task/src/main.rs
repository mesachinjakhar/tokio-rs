use tokio::task;

#[tokio::main(flavor = "current_thread")]
async fn main() {
    task::spawn(async {
    // this wont print
    println!("Hello from task");
});


}