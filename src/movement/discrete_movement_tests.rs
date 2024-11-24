#[cfg(test)]
mod tests
{
    use crate::movement::moves::{CaptureType, Movement, DiscreteMovement};
    use crate::movement::err::MovementErrorType;
    use crate::board::*;
    use crate::piece::*;

    #[test]
    fn discrete_movements_test ()
    {
        // Will test bodyblocking
        let mov_two_forward_nocap = DiscreteMovement
       {
           offset: (0, 2),
           capture_type: CaptureType::Move
       };

        // Will test not being able to move without a capture
        let mov_two_right_cap = DiscreteMovement
       {
           offset: (2, 0),
           capture_type: CaptureType::Capture
       };

        // Will test capturing
        let mov_two_up_two_right_movcap = DiscreteMovement
       {
           offset: (2, 2),
           capture_type: CaptureType::MoveCapture
       };

        // THE TESTER
        // TODO: MAKE HIM DO MORE TESTS.
        // Actual TODO: Try to make the move tests use the tester from within the board directly.
        let tester = Piece
       {
           name: "THE TESTER".to_string(),
           icon: 'T',
           moves: Vec::new(),
           position: (10, 10)
       };

        // Should PREVENT movement
        let dummy_one = Piece
       {
           name: "Dummy".to_string(),
           icon: 'd',
           moves: vec![],
           position: (10, 12)
       };

        // Should CREATE A CAPTURE
        let dummy_two = Piece
        {
            name: "Dummy".to_string(),
            icon: 'd',
            moves: vec![],
            position: (12, 12)
        };

        let mut board = Board {pieces: Vec::new(), size: (20, 20)};
        board.pieces.push(dummy_one);
        board.pieces.push(dummy_two);

        // Wants to move, but can't capture
        let result = mov_two_forward_nocap.is_valid(&board, tester.position, (10, 12));
        match result
        {
            Ok(_) => panic!("Expected Err PieceBlocking (10, 12), got Ok"),
            Err(err) => assert!(matches!(err.error_type, MovementErrorType::PieceBlocking(10, 12)), "Expected PieceBlocking (10, 12), got {:?}", err.error_type),
        }

        // Wants to move, but needs a capture
        let result = mov_two_right_cap.is_valid(&board, tester.position, (12, 10));
        match result
        {
            Ok(_) => panic!("Expected Err NeedsCapture, got Ok"),
            Err(err) => assert!(matches!(err.error_type, MovementErrorType::NeedsCapture), "Expected NeedsCapture, got {:?}", err.error_type),
        }

        // Can move, but will do a capture
        let result = mov_two_up_two_right_movcap.is_valid(&board, tester.position, (12, 12));
        match result
        {
            Ok(Some(index)) => assert_eq!(index, 1, "Expected index 1, got index {}", index),
            Ok(None) => panic!("Expected Some(index), got None"),
            Err(err) => panic!("Expected Ok(Some(index)), got Err: {:?}", err),
        }

        // Movement does not allow this move
        let result = mov_two_up_two_right_movcap.is_valid(&board, tester.position, (15, 15));
        match result
        {
            Ok(_) => panic!("Expected Err DoesNotAllow (15, 15), got Ok"),
            Err(err) => assert!(matches!(err.error_type, MovementErrorType::DoesNotAllow(15, 15)), "Expected DoesNotAllow, got {:?}", err.error_type)
        }

        // This should cause a does not allow error FIRST Instead of an out of board error!!!
        let result = mov_two_up_two_right_movcap.is_valid(&board, tester.position, (102, 102));
        match result
        {
            Ok(_) => panic!("Expected Err DoesNotAllow (102, 102), got Ok"),
            Err(err) => assert!(matches!(err.error_type, MovementErrorType::DoesNotAllow(102, 102)), "Expected DoesNotAllow (102, 102), got {:?}", err.error_type)
        }

        // THIS should return OutOfBoard.
        let result = mov_two_up_two_right_movcap.is_valid(&board, (100, 100), (102, 102));
        match result
        {
            Ok(_) => panic!("Expected Err OutOfBoard (102, 102), got Ok"),
            Err(err) => assert!(matches!(err.error_type, MovementErrorType::OutOfBoard(102, 102)), "Expected OutOfBoard, got {:?}", err.error_type)
        }
    }
}