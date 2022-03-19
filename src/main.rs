use std::collections::HashMap;
use learning_rust::references_and_borrowing::fn_borrowing;

fn main() {
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

fn hashmap(){
    let mut conn:HashMap<String,u32>=HashMap::new();
    conn.insert("a".to_string(),1);
    conn.insert("b".to_string(),2);

    println!("{:?}",conn);
    conn.entry("c".to_string()).or_insert(2);
    let conn_a=conn.entry("a".to_string()).or_insert(4);
    *conn_a+=3;
    println!("{:?}",conn);
}

fn string() {
    let a = "a".to_string();
    let b = String::from("b22222222");
    let d =&b[0..1];
    println!("{}",d);
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
