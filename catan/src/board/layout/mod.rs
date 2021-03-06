mod layout;
mod default;

pub use layout::{Layout, print_layout};
pub use default::DEFAULT;

use super::{Coord, Error};

const fn c(y:i8, x:i8) -> Coord {
    Coord::new(x,y)
}
