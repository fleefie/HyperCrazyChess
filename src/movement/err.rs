use std::fmt;
#[derive(Debug, Clone)]
pub enum MovementErrorType
{
    OutOfBoard,
    PieceBlocking {x: i32, y: i32}
}

#[derive(Debug, Clone)]
pub struct MovementError
{
    pub message: String,
    pub error_type: MovementErrorType
}

impl fmt::Display for MovementError
{
    fn fmt(&self, f: &mut fmt::Formatter) -> Result<(), fmt::Error>
    {
        write!(f, "{}, {:?}", self.message, self.error_type)
    }
}

impl std::error::Error for MovementError {}