use std::fs::File;
use std::io::ErrorKind;

fn main(){

    let file_name = String::from("some_file.txt");
    let f = File::open(&file_name);

    let f = match f{
        Ok(file) => file,
        Err(error) => match error.kind(){
            ErrorKind::NotFound => match File::create(file_name){
                Ok(fc) => fc,
                Err(e) => panic!("Problem creating the file: {:?}", e),
            },
            other_error =>{
                panic!("Problem opening the file: {:?}", other_error)
            }
        },
    };
}

//more advanced way -> gets rid of matches using closures
// fn main() {
//     let f = File::open("hello.txt").unwrap_or_else(|error| {
//         if error.kind() == ErrorKind::NotFound {
//             File::create("hello.txt").unwrap_or_else(|error| {
//                 panic!("Problem creating the file: {:?}", error);
//             })
//         } else {
//             panic!("Problem opening the file: {:?}", error);
//         }
//     });
// }


//simple but less useful
// fn main(){
//     let file = File::open("../some_file.txt");
//     let file = match file{
//         Ok(f) => f,
//         Err(error) => panic!("Problem opening the file: {:?}", error) 
//     };

// }