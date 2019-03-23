#[macro_use]
pub mod macros;

mod color;
pub mod random;

// ------------------------

pub struct Provider {
    pub color: color::Provider,
}

impl Provider {
    pub fn new() -> Self {
        Provider {
            color: color::Provider::new(),
        }
    }
}

impl Default for Provider {
    fn default() -> Self {
        Self::new()
    }
}
