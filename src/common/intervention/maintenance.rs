#[derive(Debug, Clone, Copy)]
pub enum Maintenance {
    Gas(MaintenanceGas),
    Diesel(MaintenanceDiesel),
}

#[derive(Debug, Clone, Copy)]
pub struct MaintenanceGas {
    filter_air: bool,
    filter_cabin: bool,
    filter_oil: bool,
    spark_plug: bool,
}

#[derive(Debug, Clone, Copy)]
pub struct MaintenanceDiesel {
    filter_air: bool,
    filter_cabin: bool,
    filter_oil: bool,
    filter_gasoil: bool,
}
