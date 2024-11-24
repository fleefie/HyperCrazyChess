use crate::piece::Piece;

pub struct Board<'a>
{
    pub pieces: Vec<Piece<'a>>,
    pub size: (i32, i32)
}