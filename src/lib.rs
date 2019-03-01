mod color;
pub mod random;

pub struct Provider {
    pub color: color::Provider,
}

impl Provider {
    pub fn new() -> Self {
        // 暂时使用37
        let selector = random::Random::new(37);

        Provider {
            color: color::Provider::new(selector.clone()),
        }
    }
}

impl Default for Provider {
    fn default() -> Self {
        Self::new()
    }
}
