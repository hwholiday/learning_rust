#[derive(Debug)]
pub struct User{
     pub name:String,
     pub pwd:String
}
 impl User{
     pub fn def_new()->User{
        User{
            name:String::from("test"),
            pwd:String::from("test_pwd")
        }
    }
}