use learning_rust::references_and_borrowing::fn_borrowing;

fn main() {
    fn_borrowing();
    let vec:Vec<String> =Vec::new();
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
