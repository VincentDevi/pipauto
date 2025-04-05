use strum_macros::Display;

#[derive(Debug, Clone, Copy, Display)]
pub enum Fuel {
    Gasoline,
    Diesel,
    Other,
}
