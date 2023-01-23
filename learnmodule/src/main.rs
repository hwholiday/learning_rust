use learnmodule::mod_test;
use rand::Rng;
fn main() {
    println!("learn module");
    mod_test();
    test_rand();
}


fn test_rand() {
    let secret_number = rand::thread_rng().gen_range(1..=100);
    println!("test_rand() got {}",secret_number)
}