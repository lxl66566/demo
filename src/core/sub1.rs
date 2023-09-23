use super::*;
pub struct T1;

impl Process for T1 {
    fn f(&self) {
        println!("1");
    }
}

// impl Takable for T1 {}
