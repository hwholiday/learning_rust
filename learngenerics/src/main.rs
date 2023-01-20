fn main() {
    println!("learn generics");
    println!(" add(1,2) got :{} want :{}", add(1, 2), 3);
    println!(" add(1,2) got :{} want :{}", add(1i32, 2i32), 3);
    println!(" add(1.2,2.1) got :{} want :{}", add(1.2, 2.1), 3.3);
    test_in_array();
    test_trait();
    test_login_client();
}

fn add<T: std::ops::Add<Output = T>>(a: T, b: T) -> T {
    a + b
}

fn test_in_array() {
    let key = 1;
    let arr = [1, 2, 3];
    let exist = in_array(&key, &arr);
    println!("exist: {}", exist);

    let key = "1";
    let arr = ["11", "2", "3"];
    let exist = in_array(&key, &arr);
    println!("exist: {}", exist);

    let key = "1";
    let arr = vec!["1", "2", "3"];
    let exist = in_array(&key, &arr);
    println!("exist: {}", exist);
}

fn in_array<T: std::cmp::PartialEq<U>, U>(key: &T, list: &[U]) -> bool {
    let mut result = false;
    for val in list {
        if key == val {
            result = true
        }
    }
    result
}

#[derive(Debug)]
struct User {
    name: String,
    pwd: String,
}

trait Manager {
    fn login(&self) -> bool;
}

#[derive(Debug)]
struct Admin {
    name: String,
    pwd: String,
}

impl Manager for User {
    fn login(&self) -> bool {
        if self.name != "h" || self.pwd != "w" {
            false
        } else {
            true
        }
    }
}

impl Manager for Admin {
    fn login(&self) -> bool {
        if self.name != "h" || self.pwd != "w" {
            false
        } else {
            true
        }
    }
}

fn test_login_client(){
    let cli=login_client(true);
    println!("login_client(true) is {}",cli.login());

    let cli=login_client(false);
    println!("login_client(false) is {}",cli.login());
} 

fn login_client(is_admin:bool)-> Box<dyn Manager +  'static>{
    if is_admin{
        let admin = Admin {
            name: "a".to_string(),
            pwd: "b".to_string(),
        };
        Box::new(admin)
    }else {
        let user = User {
            name: "h".to_string(),
            pwd: "w".to_string(),
        };
        Box::new(user)
    }
}

fn test_trait() {
    let user = User {
        name: "a".to_string(),
        pwd: "b".to_string(),
    };
    println!("user.login() is {}", user.login());
    let user = User {
        name: "h".to_string(),
        pwd: "w".to_string(),
    };
    println!("user.login() is {}", login(&user));

    let admin = Admin {
        name: "a".to_string(),
        pwd: "b".to_string(),
    };
    println!("admin.login() is {}", admin.login());
    println!("admin.login() is {}", login(&admin));
    println!("admin.login_t() is {}", login_t(&admin));
    println!("admin.login_w() is {}", login_w(&admin));
}


fn login(input: &impl Manager)->bool{
    input.login()
}

fn login_t<T:Manager>(input:&T)->bool{
    input.login()
}

fn login_w<T>(input:&T)->bool
    where T:Manager,
{
    input.login()
}



