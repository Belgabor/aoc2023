use glam::IVec2;
use crate::Direction::{Down, Left, Right, Up};
use crate::NodeType::{BottomLeft, BottomRight, Horizontal, TopLeft, TopRight, Vertical};

#[derive(Debug, PartialEq, Eq, Clone, Copy, Hash)]
pub enum Direction {
    Up,
    Right,
    Down,
    Left,
}

impl Direction {
    pub fn opposite(&self) -> Direction {
        match self {
            Up => Down,
            Right => Left,
            Down => Up,
            Left => Right,
        }
    }

    pub fn delta(&self) -> IVec2 {
        match self {
            Up => IVec2::new(0, -1),
            Right => IVec2::new(1, 0),
            Down => IVec2::new(0, 1),
            Left => IVec2::new(-1, 0),
        }
    }
}

#[derive(Debug, PartialEq, Eq, Clone)]
pub enum NodeType {
    Horizontal,
    Vertical,
    BottomLeft,
    BottomRight,
    TopLeft,
    TopRight,
}

impl NodeType {
    pub fn new(from: &Direction, to: &Direction) -> Self {
        match from {
            Up => match to {
                Up => unreachable!(),
                Right => TopRight,
                Down => Vertical,
                Left => TopLeft,
            }
            Right => match to {
                Up => TopRight,
                Right => unreachable!(),
                Down => BottomRight,
                Left => Horizontal,
            }
            Down => match to {
                Up => Vertical,
                Right => BottomRight,
                Down => unreachable!(),
                Left => BottomLeft,
            }
            Left => match to {
                Up => TopLeft,
                Right => Horizontal,
                Down => BottomLeft,
                Left => unreachable!(),
            }
        }
    }

    pub fn route(direction: &Direction) -> Self {
        match direction {
            Up => Vertical,
            Right => Horizontal,
            Down => Vertical,
            Left => Horizontal,
        }
    }

    pub fn symbol(&self) -> char {
        match self {
            Vertical => '┃',
            Horizontal => '━',
            BottomLeft => '┑',
            BottomRight => '┍',
            TopLeft => '┙',
            TopRight => '┕',
        }
    }
}

#[cfg(test)]
mod tests {
    use super::*;

    #[test]
    fn it_works() {
        assert_eq!(Up.opposite(), Down);
        assert_eq!(Down.opposite(), Up);
        assert_eq!(Left.opposite(), Right);
        assert_eq!(Right.opposite(), Left);
    }
}
