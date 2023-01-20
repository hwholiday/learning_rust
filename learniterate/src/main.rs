use std::collections::{BTreeMap, HashMap};

fn main() {
    println!("learn iterate");
    f_in();
    f_iter();
    f_struct();
    f_map();
    f_btree_map();
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

fn f_struct() {
    let mut user_ver = Vec::new();
    user_ver.push(User {
        name: String::from("value"),
    });
    for v in &user_ver {
        println!("{:?}", v.name)
    }
}
//无序
fn f_map() {
    //let mut hash_map=HashMap::new();
    let mut hash_map = HashMap::with_capacity(3);
    hash_map.insert("1", "1");
    hash_map.insert("2", "2");
    hash_map.insert("3", "3");
    for (key, val) in &hash_map {
        println!("key is {} val is {}", key, val);
    }
}

//有序
fn f_btree_map() {
    //let mut hash_map=HashMap::new();
    let mut hash_map = BTreeMap::new();
    hash_map.insert("1", "1");
    hash_map.insert("2", "2");
    hash_map.insert("3", "3");
    for (key, val) in &hash_map {
        println!("key is {} val is {}", key, val);
    }
}
