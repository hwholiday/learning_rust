pub mod modone;
pub use modone::one::fn_one;
pub mod modtwo;
pub use modtwo::two::{fn_two_v2,fn_two};
pub use modtwo::twotwo::fn_twotwo;

pub fn mod_test(){
    fn_one();
    fn_two();
    fn_two_v2();
    fn_twotwo();
}