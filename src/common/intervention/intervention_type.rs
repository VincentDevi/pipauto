use super::*;

#[derive(Debug, Clone, Copy)]
pub enum InterventionType {
    Repair,
    Maintenance(Maintenance),
}
