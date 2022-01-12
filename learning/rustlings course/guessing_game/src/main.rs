use std::io;
use rand::Rng;
use std::cmp::Ordering;

fn main(){
    let secret_number = rand::thread_rng().gen_range(1..101); //1-100, inclusive on lower bound, exclusive on upper
    
    println!("Guess the number!");
    
    loop{
        println!("Please input your guess.");
        let mut guess = String::new();
        io::stdin().read_line(&mut guess).expect("Failed to read line!");
        let guess: u32 = match guess.trim().parse(){ //shadow the guess variable and use a lambda for custom error handling
            Ok(num) => num,
            Err(_) => continue,
        }; 

        match guess.cmp(&secret_number){
            Ordering::Less => println!("too small!"),
            Ordering::Greater => println!("too big!"),
            Ordering::Equal => {
                println!("you win!!");
                break;
            }
        }
        println!("You guessed: {}", guess);
    }
}