#[derive(Clone, PartialEq, Debug)]
pub enum Orientation {
    Up,
    Right,
    Down,
    Left,
}

#[derive(Clone, PartialEq, Debug, Copy)]
#[repr(u8)]
pub enum PieceColor {
    Wall,
    Empty,
    Red,
    Green,
    Blue,
    Yellow,
    Cyan,
    Magenta,
    Orange,
    Tracer,
}

#[derive(Clone, PartialEq, Debug)]

///
///  Grid encoding the offsets from the center piece, with only those locations
///  that are relevant to the pieces we encode.
///
///    NW  | N  | NE |
///    -----------------
///    W   | C  | E  | EE
///    -----------------
///    SW  | S  | SE | ESE
///    -----------------
///    WW  | SS | SBE|
///
///
pub enum Offsets {
    Center,
    North,
    NorthWest,
    West,
    SouthWest,
    South,
    SouthEast,
    East,
    NorthEast,
    EastEast,
    SouthSouth,
    SouthBySouthEast,
    EastBySouthEast,
}

///
/// Mapping of the grid to relative x, y coordinates.
/// North is up, so smaller y, East is right, so larger x.
///
pub fn xy(offset: &Offsets) -> (i8, i8) {
    match offset {
        Center => (0, 0),
        North => (0, -1),
        NorthWest => (-1, -1),
        West => (-1, 0),
        SouthWest => (-1, 1),
        South => (0, 1),
        SouthEast => (1, 1),
        East => (1, 0),
        NorthEast => (1, -1),
        EastEast => (2, 0),
        SouthSouth => (0, 2),
        SouthBySouthEast => (1, 2),
        EastBySouthEast => (2, 1),
    }
}
use Offsets::*;

pub type PieceView = [Offsets; 4];

#[derive(Clone, PartialEq, Debug)]
pub struct Piece {
    pub color: PieceColor,
    pub orientation: Orientation,
    up: PieceView,
    right: PieceView,
    down: PieceView,
    left: PieceView,
}

impl Piece {
    pub fn view(&self) -> &PieceView {
        match self.orientation {
            Orientation::Up => &self.up,
            Orientation::Right => &self.right,
            Orientation::Down => &self.down,
            Orientation::Left => &self.left,
        }
    }

    pub fn rotate_left(&mut self) {
        self.orientation = match self.orientation {
            Orientation::Up => Orientation::Left,
            Orientation::Right => Orientation::Up,
            Orientation::Down => Orientation::Right,
            Orientation::Left => Orientation::Down,
        };
    }

    pub fn rotate_right(&mut self) {
        self.orientation = match self.orientation {
            Orientation::Up => Orientation::Right,
            Orientation::Right => Orientation::Down,
            Orientation::Down => Orientation::Left,
            Orientation::Left => Orientation::Up,
        };
    }
}

pub const PIECES: [Piece; 7] = [
    // O piece
    Piece {
        color: PieceColor::Yellow,
        orientation: Orientation::Up,
        up: [North, NorthEast, Center, East],
        right: [North, NorthEast, Center, East],
        down: [North, NorthEast, Center, East],
        left: [North, NorthEast, Center, East],
    },
    // T Piece
    Piece {
        color: PieceColor::Magenta,
        orientation: Orientation::Up,
        up: [Center, West, East, North],
        right: [Center, East, North, South],
        down: [Center, West, East, South],
        left: [Center, West, North, South],
    },
    Piece {
        // I piece
        color: PieceColor::Cyan,
        orientation: Orientation::Up,
        up: [West, Center, East, EastEast],
        right: [NorthEast, East, SouthEast, SouthBySouthEast],
        down: [SouthWest, South, SouthEast, EastBySouthEast],
        left: [North, Center, South, SouthSouth],
    },
    Piece {
        // L piece
        color: PieceColor::Orange,
        orientation: Orientation::Up,
        up: [West, Center, East, NorthEast],
        right: [North, Center, South, SouthEast],
        down: [Center, East, West, SouthWest],
        left: [NorthWest, North, Center, South],
    },
    Piece {
        // J piece
        color: PieceColor::Blue,
        orientation: Orientation::Up,
        up: [Center, East, West, NorthWest],
        right: [NorthEast, North, Center, South],
        down: [West, Center, East, SouthEast],
        left: [North, Center, South, SouthWest],
    },
    Piece {
        // S piece
        color: PieceColor::Green,
        orientation: Orientation::Up,
        up: [West, Center, North, NorthEast],
        right: [North, Center, East, SouthEast],
        down: [SouthWest, South, Center, East],
        left: [NorthWest, West, Center, South],
    },
    Piece {
        // Z piece
        color: PieceColor::Red,
        orientation: Orientation::Up,
        up: [NorthWest, North, Center, East],
        right: [South, Center, East, NorthEast],
        down: [West, Center, South, SouthEast],
        left: [SouthWest, West, Center, North],
    },
];
