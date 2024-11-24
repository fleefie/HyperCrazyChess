use crate::movement::moves::Movement;

pub struct Piece<'a>
{
    pub name: String,
    pub icon: char,
    pub moves: Vec<&'a dyn Movement>,
    pub position: (i32, i32)
}