use std::fmt;
#[derive(Debug, Clone)]
pub enum MovementErrorType
{
    OutOfBoard (i32, i32),
    PieceBlocking (i32, i32),
    DoesNotAllow (i32, i32),
    NeedsCapture
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