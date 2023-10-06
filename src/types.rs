use std::error::Error;

pub type CustomResault<T> = Result<T, Box<dyn Error>>;
