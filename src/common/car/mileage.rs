use derive_more::{Constructor, Display};

#[derive(Debug, Clone, Copy, Constructor, Display)]
pub struct Milage(f64);
