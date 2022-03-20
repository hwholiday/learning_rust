pub trait UserLoginInterFace {
    fn verify(&self) -> bool;
    fn verifyv2(&self)->String{
        return "verifyv2".to_string()
    }
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
