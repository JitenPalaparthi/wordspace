#[derive(Debug)]
pub struct CustomError{
   pub message:String,
}

impl std::fmt::Display for CustomError {
    fn fmt(&self, f: &mut std::fmt::Formatter) -> std::fmt::Result {
        write!(f, "{} ", self.message)
    }
}

impl std::error::Error for CustomError {}