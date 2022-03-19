pub mod tiait;
pub mod references_and_borrowing {
    pub fn fn_borrowing() {
        super::borrowing()
    }
}
fn borrowing() {
    println!("Hello, world!");
    let mut a = String::from("bbbbbbbbb");
    len(&mut a);
    println!("car {}", a)
}

fn len(car: &mut String) {
    car.push_str("ccccccc");
    println!("car {}", car)
}