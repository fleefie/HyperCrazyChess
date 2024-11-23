use super::err::MovementError;

pub trait Movement
{
    /// Checks if movement is valid for position (x, y)
    /// Returns True if the move is valid for that position,
    /// False otherwise.
    fn is_valid(&self, x: i32, y: i32) -> Result<bool, MovementError>;
}

pub struct DiscreteMovement
{
    x_offset: i32,
    y_offset: i32
}