#[derive(Debug, Clone, Copy)]
pub struct Maintenance {
    filter_air: bool,
    filter_cabin: bool,
    filter_oil: bool,
    type_specific_maintenance: Option<MaintenanceType>,
}

#[derive(Debug, Clone, Copy)]
pub enum MaintenanceType {
    FilterGasoil,
    SparkPlug,
}
