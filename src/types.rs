use clap::ValueEnum;
use std::cmp::{max, min};
use std::ops::Add;
use std::ops::AddAssign;
use std::ops::RangeInclusive;
use std::ops::Sub;
use std::str::FromStr;

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

#[derive(Debug)]
pub struct CoordsParsingError;

impl<ParsableT: FromStr> FromStr for Coords<ParsableT> {
    type Err = CoordsParsingError;

    fn from_str(s: &str) -> Result<Self, Self::Err> {
        let mut parts = s.split(",");
        let x = parts
            .next()
            .ok_or(CoordsParsingError)?
            .parse::<ParsableT>()
            .map_err(|_| CoordsParsingError)?;
        let y = parts
            .next()
            .ok_or(CoordsParsingError)?
            .parse::<ParsableT>()
            .map_err(|_| CoordsParsingError)?;
        Ok(Coords { x, y })
    }
}

impl<NumericT: From<bool> + Eq + Ord + Copy> Coords<NumericT>
where
    RangeInclusive<NumericT>: Iterator<Item = NumericT>,
{
    pub fn is_origin(&self) -> bool {
        return self.x == NumericT::from(false) && self.y == NumericT::from(false);
    }

    pub fn origin() -> Coords<NumericT> {
        Coords {
            x: NumericT::from(false),
            y: NumericT::from(false),
        }
    }

    pub fn line_between(&self, other: &Coords<NumericT>) -> Vec<Coords<NumericT>> {
        if self.x == other.x {
            let start_y = min(self.y, other.y);
            let end_y = max(self.y, other.y);
            (start_y..=end_y).map(|y| Coords { x: self.x, y }).collect()
        } else if self.y == other.y {
            let start_x = min(self.x, other.x);
            let end_x = max(self.x, other.x);
            (start_x..=end_x).map(|x| Coords { y: self.y, x }).collect()
        } else {
            Vec::new()
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
