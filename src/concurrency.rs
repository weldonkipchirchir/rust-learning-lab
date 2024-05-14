// Fearless Concurrency
//Using Threads to Run Code Simultaneously
// Creating a New Thread with spawn

// To create a new thread, we call the thread::spawn function and pass it a closure containing the code we want to run in the new thread.

//example
use std::thread;
use std::time::Duration;
pub fn concurrency(){
    create_thread();
    create_thread_move();
    channels_example();
    // send_multiple_messages();
    multiple_producers();
    mutex();
    shared_mutex();
}

/*
Note that when the main thread of a Rust program completes, all spawned threads are shut down, whether or not they have finished running.
The calls to thread::sleep force a thread to stop its execution for a short duration, allowing a different thread to run. The threads will probably take turns, but that isn’t guaranteed: it depends on how your operating system schedules the threads.
*/
/*
fn create_thread(){
    thread::spawn(|| {
        for i in 1..10 {
            println!("hi number {} from the spawned thread!", i);
            thread::sleep(Duration::from_millis(1));
        }
    });
    for i in 1..5 {
        println!("hi number {} from the main thread!", i);
        thread::sleep(Duration::from_millis(1));
    }
}
 */

// Waiting for All Threads to Finish Using join Handles
/*
We can fix the problem of the spawned thread not running or ending prematurely by saving the return value of thread::spawn in a variable. The return type of thread::spawn is JoinHandle. A JoinHandle is an owned value that, when we call the join method on it, will wait for its thread to finish.
Calling join on the handle blocks the thread currently running until the thread represented by the handle terminates. Blocking a thread means that thread is prevented from performing work or exiting. 
*/

fn create_thread(){
    let handle = thread::spawn(|| {
        for i in 1..10 {
            println!("hi number {} from the spawned thread!", i);
            thread::sleep(Duration::from_millis(1));
        }
    });
    handle.join().unwrap();
    for i in 1..5 {
        println!("hi number {} from the main thread!", i);
        thread::sleep(Duration::from_millis(1));
    }

}

// Using move Closures with Threads
//We'll often use the move keyword with closures passed to thread::spawn because the closure will then take ownership of the values it uses from the environment, thus transferring ownership of those values from one thread to another. 

fn create_thread_move(){
    let v = vec![1, 2, 3];
    let handle = thread::spawn(move || {
        println!("Here's a vector: {:?}", v);
    });
    handle.join().unwrap();
}

//Using Message Passing to Transfer Data Between Threads
// channel example     let (tx, rx) = mpsc::channel();
// We create a new channel using the mpsc::channel function; mpsc stands for multiple producer, single consumer. In short, the way Rust’s standard library implements channels means a channel can have multiple sending ends that produce values but only one receiving end that consumes those values. 


use std::sync::mpsc;

fn channels_example(){
    let (tx, rx) = mpsc::channel();

    thread::spawn(move || {
        let val = String::from("hello");
        tx.send(val).expect("error in sending val in thread");
    });
    let received = rx.recv().expect("error in receiving val in main");
    println!("Got: {}", received);
}


// channels and ownership transference
//  The send function takes ownership of its parameter, and when the value is moved, the receiver takes ownership of it. This stops us from accidentally using the value again after sending it; the ownership system checks that everything is okay.

// Sending Multiple Values and Seeing the Receiver Waiting

fn send_multiple_messages(){
    let (tx, rx) = mpsc::channel();

    thread::spawn(move || {
        let vals = vec![
            String::from("hi"),
            String::from("from"),
            String::from("the"),
            String::from("thread"),
        ];

        for val in vals {
            tx.send(val).unwrap();
            thread::sleep(Duration::from_secs(1));
        }
    });

    for received in rx {
        println!("Got: {}", received);
    }
}


// creating multiple producers by clonning the transmitter
// create multiple threads that all send values to the same receiver. 
fn multiple_producers(){
    let (tx, rx) = mpsc::channel();

    let tx1 = tx.clone(); // we call clone on the transmitter. This will give us a new transmitter we can pass to the first spawned thread. We pass the original transmitter to a second spawned thread. This gives us two threads, each sending different messages to the one receiver.
    thread::spawn(move || {
        let vals = vec![
            String::from("hi"),
            String::from("from"),
            String::from("the"),
            String::from("thread"),
        ];

        for val in vals {
            tx1.send(val).unwrap();
            thread::sleep(Duration::from_secs(1));
        }
    });

    thread::spawn(move || {
        let vals = vec![
            String::from("more"),
            String::from("messages"),
            String::from("for"),
            String::from("you"),
        ];

        for val in vals {
            tx.send(val).unwrap();
            thread::sleep(Duration::from_secs(1));
        }
    });

    for received in rx {
        println!("Got: {}", received);
    }

}


// shared-state concurrency
// Using Mutexes to Allow Access to Data from One Thread at a Time
// Mutex is an abbreviation for mutual exclusion, as in, a mutex allows only one thread to access some data at any given time. To access the data in a mutex, a thread must first signal that it wants access by asking to acquire the mutex’s lock. The lock is a data structure that is part of the mutex that keeps track of who currently has exclusive access to the data. Therefore, the mutex is described as guarding the data it holds via the locking system.
// Mutexes have a reputation for being difficult to use because you have to remember two rules:

// You must attempt to acquire the lock before using the data.
// When you’re done with the data that the mutex guards, you must unlock the data so other threads can acquire the lock.


// The API of Mutex<T>

 use std::sync::Mutex;

 fn mutex(){
    let m = Mutex::new(5);

    {   // To access the data inside the mutex, we use the lock method to acquire the lock. This call will block the current thread so it can’t do any work until it’s our turn to have the lock.
        let mut num = m.lock().unwrap();
        *num = 6;
    }

    println!("m = {:?}", m);
 }
 //call to lock returns a smart pointer called MutexGuard, wrapped in a LockResult that we handled with the call to unwrap. The MutexGuard smart pointer implements Deref to point at our inner data; the smart pointer also has a Drop implementation that releases the lock automatically when a MutexGuard goes out of scope, which happens at the end of the inner scope. As a result, we don’t risk forgetting to release the lock and blocking the mutex from being used by other threads, because the lock release happens automatically.


 //Sharing a Mutex<T> Between Multiple Threads
 use std::sync::{Arc};
 //Atomic Reference Counting with Arc<T>
// Fortunately, Arc<T> is a type like Rc<T> that is safe to use in concurrent situations.
fn shared_mutex() {
    let counter = Arc::new(Mutex::new(0));
    let mut handles = vec![];

    for _ in 0..10 {
        let counter = Arc::clone(&counter);
        let handle = thread::spawn(move || {
            let mut num = counter.lock().unwrap();
            *num += 1;
        });
        handles.push(handle);
    }

    for handle in handles {
        handle.join().unwrap();
    }

    println!("Result: {}", *counter.lock().unwrap());
}

//Similarities Between RefCell<T>/Rc<T> and Mutex<T>/Arc<T>
// notice that counter is immutable but we could get a mutable reference to the value inside it; this means Mutex<T> provides interior mutability, as the Cell family does.


//extensible concurrency with the sync and send traits