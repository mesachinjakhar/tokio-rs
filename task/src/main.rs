use tokio::time;

use tokio::task;

#[tokio::main(flavor = "current_thread")]
async fn main() {
    // spwan and spawn_blocking return JoinHandler
    let handler = task::spawn(async {
        // this wont print because our main fn returns before this spawn  async task run
        println!("Hello from task");
        return 5;
    });

    let join = task::spawn(async { panic!("task panic here") });

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

    let handle1 = tokio::spawn(async {
        // do some stuff here
    });
    let handle2 = tokio::spawn(async {
        // do some other stuff here
        time::sleep(time::Duration::from_secs(10)).await;
    });

    // Wait for the task to finish
    handle2.abort();
    time::sleep(time::Duration::from_secs(1)).await;
    assert!(handle1.is_finished());
    assert!(handle2.is_finished());

    let handle3 = tokio::spawn(async {
        // do some other stuff here
        time::sleep(time::Duration::from_secs(10)).await;
    });

    let id = handle3.id();
    println!("id is: {}", id);

    // this can be used give permission to other task to abort this task
    // we cant awwit on abort_handler
    let abort_handler = handle3.abort_handle();

    // spawn_blocking create a blocking task on dedicated thread
    task::spawn_blocking(|| {
        // do some compute-heavy work or call synchronous code
    });


    // to run block_in_place, we have to use rt_multi_thread runtime instead of single thread
    // let result = task::block_in_place(|| {
    //     // do some compute-heavy work or call synchronous code
    //     "blocking completed"
    // });


}
