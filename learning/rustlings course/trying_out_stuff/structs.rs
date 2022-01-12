

struct Test{
}

impl Test{
    fn test(&self){
        println!("testing!\n");
    }
}

fn main(){
    
    let t = Test{};
    t.test();
}