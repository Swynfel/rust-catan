use super::{Layout, c};
use once_cell::sync::Lazy;

fn default_layout() -> Layout {
    let hexes = vec![
                  c(-4, -4), c(-4, 0), c(-4, 4),
             c(-2, -6), c(-2,-2), c(-2, 2), c(-2, 6),
        c( 0, -8), c( 0,-4), c( 0, 0), c( 0, 4), c( 0, 8),
             c( 2, -6), c( 2,-2), c( 2, 2), c( 2, 6),
                  c( 4, -4), c( 4, 0), c( 4, 4),
    ];

    Layout::new(2, hexes)
}

pub static DEFAULT: Lazy<Layout> = Lazy::new(||
    default_layout()
);
