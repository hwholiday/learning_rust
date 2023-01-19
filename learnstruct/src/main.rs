fn main() {
    let mut user = User::new("name".to_string(), "pwd".to_string());
    let login_succeed = user.login_succeed();
    println!("{}", login_succeed);
    user.set_name("h".to_string());
    user.set_pwd("w".to_string());
    let login_succeed = user.login_succeed();
    println!("{}", login_succeed);
    user.set_name2("a".to_string());
    println!("{:?}", user);
}
#[derive(Debug)]
struct User {
    name: String,
    pwd: String,
}

impl User {
    fn new(name: String, pwd: String) -> User {
        User {
            name: String::from(name),
            pwd: String::from(pwd),
        }
    }
    fn login_succeed(&self) -> bool {
        if self.name == "h" && self.pwd == "w" {
            true
        } else {
            false
        }
    }
    fn set_name(&mut self, name: String) {
        self.name = name;
    }
    fn set_pwd(&mut self, pwd: String) {
        self.pwd = pwd;
    }
}

impl User {
    fn set_name2(&mut self, name: String) {
        self.name = name;
    }
}
