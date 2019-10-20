//#[derive(Debug)]
use super::document::Document;
pub trait Storage {
    fn read4mongo(&mut self, file_name: &str) -> Document;
}
pub struct Util;

impl Util {
    pub fn sss(){

    }
}
impl Storage for Util {
    fn read4mongo(&mut self, file_name: &str) -> Document {
        unimplemented!()
    }
}