

#[derive(Debug)]
pub struct Incept {
   pub pc: u8,
   pub steps:u8,

}

impl Incept {
    pub fn new() -> Incept {
        Incept {
            pc: 0,
            steps: u8::MAX,
        }
    }

}