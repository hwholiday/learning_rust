use std::collections::HashMap;

fn main() {
    println!("learn iterate");
    f_in();
    f_iter();
    f_struct();
    f_map();
}

fn f_in() {
    let arr = [11, 22, 33];
    for v in arr {
        println!("{}", v);
    }
}

fn f_iter() {
    let arr = [11, 22, 33];
    for v in arr.iter() {
        println!("{}", v);
    }
}

struct User {
    name: String,
}

fn f_struct(){
   let  mut user_ver = Vec::new();
   user_ver.push(User{
    name:String::from("value")
   });
   for v in &user_ver{
    println!("{:?}",v.name)
   }

}

fn f_map(){
    //let mut hash_map=HashMap::new();
    let mut hash_map=HashMap::with_capacity(3);
    hash_map.insert("1", "333");
    hash_map.insert("2", "333");
    hash_map.insert("3", "333");
    for (key,val) in &hash_map{
        println!("key is {} val is {}",key,val);
    }

}
