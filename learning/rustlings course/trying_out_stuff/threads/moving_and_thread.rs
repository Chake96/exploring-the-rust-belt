use std::thread;

fn main() {
    let v = vec![1, 2, 3];

    let handle = thread::spawn(move || { //move here forces the thread/closure to take ownership of v so it will not go out of scope
        println!("Here's a vector: {:?}", v);
        // drop(v); okay!
    });
    
    // drop(v); error!!!
    
    handle.join().unwrap();
}