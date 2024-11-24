use crate::movement::err::*;
use crate::board::*;

pub trait Movement
{
    /// Checks if movement is valid for position (x, y)
    /// Returns () if the move is valid for that position,
    /// Returns the index of the to-be-captured piece if possible,
    /// False otherwise.
    fn is_valid(&self, board: &Board, source: (i32, i32), target: (i32, i32)) -> Result<Option<usize>, MovementError>;
}

pub enum CaptureType
{
    Move,
    Capture,
    MoveCapture
}

pub struct DiscreteMovement
{
    pub offset: (i32, i32),
    pub capture_type: CaptureType
}

/// Takes in the board, the position of the executing piece, and the target.
/// Returns a MovementError if the check fails,
/// If the movement is valid, returns either nothing or the index of the
/// piece at the movement target in the board's vector of pieces.
impl Movement for DiscreteMovement
{
    fn is_valid(&self, board: &Board, source: (i32, i32), target: (i32, i32)) -> Result<Option<usize>, MovementError>
    {
        if target.0 - source.0 != self.offset.0 || target.1 - source.1 != self.offset.1
        {
            return Err(MovementError {
                message: "movement does not allow this offset".to_string(),
                error_type: MovementErrorType::DoesNotAllow(target.0, target.1)
            })
        }

        if target.0 > board.size.0 || target.1 > board.size.1
        {
            return Err(MovementError {
                message: "target out of board".to_string(),
                error_type: MovementErrorType::OutOfBoard(target.0, target.1)
            })
        }

        // I feel like this is more efficient than a hashmap...
        match (board.pieces.iter().position(|piece| piece.position == target), &self.capture_type)
        {
            (Some(_index), CaptureType::Move)
                => Err(MovementError {
                    message: "target has a piece blocking movement".to_string(),
                    error_type: MovementErrorType::PieceBlocking(target.0, target.1)
                }),
            (Some(index), CaptureType::Capture)
                => Ok(Some(index)),
            (Some(index), CaptureType::MoveCapture)
                => Ok(Some(index)),
            (None, CaptureType::Move)
                => Ok(None),
            (None, CaptureType::Capture)
                => Err(MovementError {
                    message: "target does not have a piece to be captured".to_string(),
                    error_type: MovementErrorType::NeedsCapture
                }),
            (None, CaptureType::MoveCapture)
                => Ok(None)
        }

    }
}