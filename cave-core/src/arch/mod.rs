pub struct RegisterSpace {}

pub struct RegisterFile {}

impl RegisterFile {
    pub fn get_register_spaces() -> Vec<RegisterSpace> {
        todo!()
    }
}

pub trait Architecture {
    fn get_name() -> &'static str;
    fn get_register_file(&self) -> RegisterFile;
}
