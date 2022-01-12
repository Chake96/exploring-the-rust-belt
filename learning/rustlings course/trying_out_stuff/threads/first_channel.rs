use std::sync::mpsc;
use std::thread;
use std::time::Duration;

//multiproducer single consumer
fn main() {
    // --snip--

    let (tx, rx) = mpsc::channel();

    let tx1 = tx.clone();
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

    // --snip--
}



//sending multiple things to another thread
// fn main(){
//     let (tx, rx) = mpsc::channel();

//     thread::spawn(move ||{
//         let values = vec![
//             String::from("I"),
//             String::from("LOVE"),
//             String::from("BIG"),
//             String::from("BOOTIES"),
//         ];

//         for val in values{
//             tx.send(val).unwrap();
//             thread::sleep(Duration::from_secs(1));
//         }
//     });

//     for recvd in rx{
//         println!("{}", recvd);
//     }


// }


//simple example
// fn main(){
//     //tx sender, rx receiver
//     let(tx, rx) = mpsc::channel(); //multiple producer, single consumer

//     //sending thread
//     let sender = std::thread::spawn(move ||{
//         //the spawned thread must own the tx to move messages into the channel
//         const value:&str = "titties!\n";
//         tx.send(value).unwrap();
//         // println!("value"); //error! cannot use value after sending it through the channel
//     });

//     let received = rx.recv().unwrap();

//     println!("Got: {}", received);
// }