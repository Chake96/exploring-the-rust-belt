

fn main(){
    let mut count = 0;
    'counting_up: loop{
        if count == 3{break;}
        if count <= 2{
            break 'counting_up;
        }
        count += 1;
    }
    count += 1;
    println!("end count was: {}", count);

    let mut num = 5;
    while num != 0{
        num -= 1;
    }

    let a = [1,2,3];
    for el in a{
        println!("{}",el);
    }
    println!();

    for number in (1..4).rev(){
        println!("{}", number);
    }
    println!("finished!");
}