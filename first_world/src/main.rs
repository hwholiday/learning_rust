fn main() {
    let world="Hello, world!";
    let world=first_world(world);
    println!("{}",world);
    let world_str =String::from("Hello, world!");
    let world=first_world(&world_str[..]);
    println!("{}",world);
}

fn first_world(str: &str) -> &str {
    let bytes =str.as_bytes();
    for (i,&val) in bytes.iter().enumerate(){
        if val == b' '{
            return &str[..i]
        }
    }
    return &str[..]
}
