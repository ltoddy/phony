mod color;
pub mod random;

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
