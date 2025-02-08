// ------------------------------------------------
// Async Await
// ------------------------------------------------
use std::time::Duration;
use tokio::time::sleep;

async fn printing(i: i32) {
    sleep(Duration::from_secs(1)).await;
    println!("Task {i}");
}

//#[tokio::main]
#[tokio::main(flavor = "current_thread")] // current_thread causes it to run in a single thread
async fn main() {
    let mut handles = vec![];
    for i in 0..3 {
        let handle = tokio::spawn(async move {
            println!("Task {i}, printing first time");
            printing(i).await;
            println!("Task {i}, printing second time");
            printing(i).await;
            println!("Task {i}, completed");
        });
        handles.push(handle);
    }
    for handle in handles {
        handle.await.unwrap();
    }
    //let x = printing();
    //println!("The future has not been polled yet");
    //drop(x); //can drop if not needed
    //x.await; //or run if needed
}
