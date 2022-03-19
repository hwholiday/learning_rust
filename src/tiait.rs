pub trait UserLoginInterFace {
    fn verify(&self) -> bool;
}

pub struct UserLogin {
    pub name: String,
    pub pwd: String,
}

impl UserLoginInterFace for UserLogin {
    fn verify(&self) -> bool {
        if &self.name == "1" && &self.pwd == "2" {
            return true;
        }
        return false;
    }
}
