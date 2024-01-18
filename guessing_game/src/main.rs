use std::cmp::Ordering;
use std::io;
use rand::Rng;

fn main() {
    let mut num = rand::thread_rng().gen_range(1 ..=100);
    println!("The secret number is: {num}");
    loop {
        let mut input = String::new();
        io::stdin()
            .read_line(&mut input)
            .expect("read failed");
        let input_int:i32 =match input.trim().parse(){
            Ok(v)=>v,
            Err(_)=> {
                println!("num not is int!!!");
                continue
            },
        };
        match input_int.cmp(&mut num) {
            Ordering::Less=> println!("too less!!!"),
            Ordering::Greater=> println!("too big!!!"),
            Ordering::Equal=> {println!("you win!!!!");break},
        }
    }

}
