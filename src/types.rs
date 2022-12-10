use clap::ValueEnum;
use std::ops::Add;
use std::ops::AddAssign;
use std::ops::Sub;

#[derive(Debug, Copy, Clone, PartialEq, Eq, PartialOrd, Ord, ValueEnum)]
pub enum RunPart {
    Pt1,
    Pt2,
    Both,
}

#[derive(PartialEq, Eq, Debug, Clone, Copy, Hash)]
pub struct Coords<NumericT> {
    pub x: NumericT,
    pub y: NumericT,
}

impl<HasZero: From<bool> + Eq> Coords<HasZero> {
    pub fn is_origin(&self) -> bool {
        return self.x == HasZero::from(false) && self.y == HasZero::from(false);
    }
    pub fn origin() -> Coords<HasZero> {
        Coords {
            x: HasZero::from(false),
            y: HasZero::from(false),
        }
    }
}

impl<NumericT: Add<Output = NumericT>> Add for Coords<NumericT> {
    type Output = Coords<NumericT>;

    fn add(self, rhs: Self) -> Coords<NumericT> {
        Coords {
            x: self.x + rhs.x,
            y: self.y + rhs.y,
        }
    }
}
impl<NumericT: Sub<Output = NumericT>> Sub for Coords<NumericT> {
    type Output = Coords<NumericT>;

    fn sub(self, rhs: Self) -> Coords<NumericT> {
        Coords {
            x: self.x - rhs.x,
            y: self.y - rhs.y,
        }
    }
}
impl<NumericT: AddAssign> AddAssign for Coords<NumericT> {
    fn add_assign(&mut self, rhs: Self) {
        self.x += rhs.x;
        self.y += rhs.y;
    }
}
