// ---------------------------------------
// Threads
// ---------------------------------------

use std::rc::Rc;
use std::sync::mpsc;
use std::sync::Arc;
use std::sync::Barrier;
use std::sync::Mutex;
use std::thread;
use std::time::Duration;

fn main() {
    println!("Hello, this will be printed!");
    println!("Hello, this will be printed too!");
    println!("Concurrency starts after this line!");
    let x = "HELLO";
    let t = thread::spawn(|| {
        println!("Hello, 1 from the thread!");
        println!("Hello, 2 from the thread!");
        println!("Hello, 3 from the thread!");
        println!("Hello, 4 from the thread!");
        println!("Hello, 5 from the thread!");
        println!("Hello, 6 from the thread!");
        println!("Hello, 7 from the thread!");
    });
    // thread::sleep(Duration::from_millis(1));
    println!("Hello, 1 from the main!");
    println!("Hello, 2 from the main!");

    t.join();

    let (tx, rx) = mpsc::channel();

    //rx cant be cloned

    for i in 0..10 {
        let tx_clone = tx.clone();
        thread::spawn(move || {
            //let mut i = "5".to_string();
            println!("Sending value {i}");
            tx_clone.send(i).unwrap();
            // println!("Val is {i}"); // This will not work as the value has been sent and no longer is owned
        });
    }

    drop(tx);
    /*
    // this is only good to recieve 1 item at a time
    let recieved_val = rx.recv().unwrap();
    println!("Recieved {recieved_val}");
    let recieved_val = rx.recv().unwrap();
    println!("Recieved {recieved_val}");

    //for all....
    */
    for message in rx {
        println!("Reveived {message}");
    }

    //pt 2
    let (tx, rx) = mpsc::channel();

    thread::spawn(move || {
        let x = "some_value".to_string();
        println!("sending value {x}");
        thread::sleep(Duration::from_secs(3));
        tx.send(x).unwrap();
    });
    //let recv_val = rx.recv().unwrap();
    //println!("I am blocked!");

    let mut recieved_status = false;
    while recieved_status != true {
        match rx.try_recv() {
            Ok(recieved_value) => {
                println!("recieved value is: {recieved_value}");
                recieved_status = true;
            }
            Err(_) => println!("I am doing some other stuff"),
        }
    }

    // ------------------------------------------------
    // sharing states
    // ------------------------------------------------

    let m = Mutex::new(5);
    {
        let mut num = m.lock().unwrap();
        *num = 10;
    }
    let lock_m = m.lock().unwrap();
    println!("m is: {:?}", *lock_m);

    //let lock_m1 = m.lock().unwrap();
    //println!("This code would be blocked since lock_m isnt dropped");
    drop(lock_m);

    let lock_m1 = m.lock().unwrap();
    println!("This code now can run");

    // ------------------------------------------------
    // states pt2
    // ------------------------------------------------

    let file = Arc::new(Mutex::new(File { text: vec![] }));
    let mut thread_vec = vec![];

    for i in 0..10 {
        let file = Arc::clone(&file);
        let handle = thread::spawn(move || {
            let mut file_lock = file.lock().unwrap();
            file_lock.text.push(format!("Hello from Thread {i}"));
        });
        thread_vec.push(handle);
    }
    for handle in thread_vec {
        handle.join().unwrap();
    }
    let file_lock = file.lock().unwrap();
    for t in &file_lock.text {
        println!("{t}");
    }

    // ------------------------------------------------
    //Barriers
    // ------------------------------------------------

    let mut thread_vec = Vec::new();
    let tasks = Arc::new(Mutex::new(vec![]));
    let barrier = Arc::new(Barrier::new(5));

    for i in 0..5 {
        let tasks = tasks.clone();
        let barrier = barrier.clone();
        let handle = thread::spawn(move || {
            //task 1
            tasks
                .lock()
                .unwrap()
                .push(format!("Thread {i}, completed its part on task 1"));
            barrier.wait();

            //task 2
            tasks
                .lock()
                .unwrap()
                .push(format!("Thread {i}, completed its part on task 2"));
        });
        thread_vec.push(handle);
    }

    for handle in thread_vec {
        handle.join().unwrap();
    }

    let task_lock = &*tasks.lock().unwrap();
    for contents in task_lock {
        println!("{contents}");
    }

    // ------------------------------------------------
    //Scoped Threads
    // ------------------------------------------------

    let mut vec = vec![1, 2, 3];
    // thread::spawn(move || {
    //     println!("{:?}", vec);
    // });
    thread::scope(|some_scope| {
        some_scope.spawn(|| {
            println!("Thread inside scope");
            println!("{:?}", vec);
        });

        some_scope.spawn(|| {
            println!("Antoher Thread inside of scope");
            //vec.push(4);
            println!("{:?}", vec);
        });
    });
    println!("The Scope has finished");
    vec.push(5);
    println!("{:?}", vec);
    // ------------------------------------------------
    // Thread Park
    // ------------------------------------------------

    let data = Arc::new(Mutex::new(5));
    let data_clone = data.clone();
    let thread_1 = thread::spawn(move || {
        println!("Thread 1: I am doing some work");
        println!("Thread 1: I am doing some more work");
        //thread::sleep(Duration::from_secs(2));
        println!("Thread 1: Parked");
        //thread::park();
        thread::park_timeout(Duration::from_secs(4));

        println!("Thread 1: Printing the updated data");
        println!("Thread 1: Data {:?}", *data.lock().unwrap());
    });

    let thread_2 = thread::spawn(move || {
        println!("Thread 2: I updating the data");
        thread::sleep(Duration::from_secs(1));
        *data_clone.lock().unwrap() = 10;
        println!("Thread 2: Data updated completed.");
    });

    thread_2.join();
    // thread_1.thread().unpark();
    thread_1.join();
}

struct File {
    text: Vec<String>,
}
