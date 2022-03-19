use learning_rust::references_and_borrowing::fn_borrowing;
use std::collections::HashMap;
use std::fs::File;
use std::io::Read;

fn main() {
    read_file();
    hashmap();
    string();
    fn_borrowing();
    let v = vec![1, 2, 3, 4, 5];

    let third = &v[2];
    println!("The third element is {}", third);
    let user = User {
        name: String::from("1"),
        pass: String::from("2"),
    };

    println!("user {:?}", user);
    println!("pwd {}", user.pwd())
}
fn read_file() {
    let mut f = File::open("src/hello.txt").unwrap_or_else(|err|{
        panic!("{:?}", err)
    });
    let mut s = String::new();
    match f.read_to_string(&mut s) {
        Ok(s) => println!("len {}", s),
        Err(e) => println!("{:?}", e),
    }
    println!("{}",s)
}

fn hashmap() {
    let mut conn: HashMap<String, u32> = HashMap::new();
    conn.insert("a".to_string(), 1);
    conn.insert("b".to_string(), 2);

    println!("{:?}", conn);
    conn.entry("c".to_string()).or_insert(2);
    let conn_a = conn.entry("a".to_string()).or_insert(4);
    *conn_a += 3;
    println!("{:?}", conn);
}

fn string() {
    let a = "a".to_string();
    let b = String::from("b22222222");
    let d = &b[0..1];
    println!("{}", d);
    let c = b + &a;
    println!("{} {}", a, c)
}

#[derive(Debug)]
struct User {
    name: String,
    pass: String,
}

impl User {
    fn pwd(&self) -> bool {
        if self.pass == "2" && self.name == "1" {
            return false;
        }
        return true;
    }
}
