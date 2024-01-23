fn main() {
    let mut user = User::new(String::from("h"), String::from("w"));
    println!("{:?}", user.active);
    println!("{:?}", user.verify_password("w"));
    user.disable();
    println!("{:?}", user.verify_password("w"))
}

#[derive(Debug)]
struct User {
    name: String,
    pwd: String,
    age: u32,
    active: bool,
}

impl User {
    fn new(name: String, pwd: String) -> User {
        let mut u = User {
            name,
            pwd,
            age: 12,
            active: false,
        };
        u.active = true;
        u
    }
    fn verify_password(&self, pwd: &str) -> bool {
        if !self.active() {
            return false;
        }
        self.pwd == pwd
    }

    fn active(&self) -> bool {
        self.active
    }
    fn disable(&mut self) {
        self.active = false
    }
}
