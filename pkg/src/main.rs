use crate::user::User;

mod user;

fn main() {
    let user = User::def_new();
    println!("Hello, world! {:?}",user);
    println!("Hello, world! {:?}",user.pwd);
    println!("Hello, world! {:?}",user.name);

    let mut v = Vec::new();
    v.push(1);
    v.push(2);
    v.push(3);

    println!("pop  {:?}",v.pop());
    println!("pop  {:?}",v.pop());
    println!("pop  {:?}",v.pop());
    let c = match v.pop() {
        Some(val) => val,
        None => 666,
    };
    println!("pop  {:?}",c);

}
