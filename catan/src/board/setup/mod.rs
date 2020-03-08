mod random_default;

use super::Coord;
pub use random_default::random_default_setup_existing_state;
pub use random_default::random_default_setup as random_default;

const fn c(y:i8, x:i8) -> Coord {
    Coord::new(x,y)
}
