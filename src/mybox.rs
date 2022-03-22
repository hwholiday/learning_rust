use std::ops::Deref;

#[cfg(test)]
mod tests {
    use super::*;
    #[test]
    fn test_box() {
        let a = MyBox::new(1);
        println!("{}", *a)
    }
}
struct MyBox<T>(T);

impl<T> MyBox<T> {
    fn new(t: T) -> MyBox<T> {
        MyBox(t)
    }
}

impl<T> Deref for MyBox<T> {
    type Target = T;

    fn deref(&self) -> &Self::Target {
        &self.0
    }
}

impl<T> Drop for MyBox<T> {
    fn drop(&mut self) {
        println!("call drop fn ",)
    }
}
