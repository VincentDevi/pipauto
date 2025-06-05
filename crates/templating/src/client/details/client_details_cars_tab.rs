use askama::Template;
use common::{car::Car, intervention::Intervention};

#[derive(Template)]
#[template(path = "client_tab_cars.html")]
pub struct ClientCar {
    pub brand: String,
    pub model: String,
    pub cc: String,
    pub fuel: String,
    pub year: String,
    pub oil_quantity: String,
    pub oil_type: String,
    pub interventions: Vec<ClientTabCarIntervention>,
}

impl From<Car> for ClientCar {
    fn from(value: Car) -> Self {
        Self {
            brand: value.brand().to_string(),
            model: value.model().to_string(),
            cc: value.cc().to_string(),
            fuel: value.fuel().to_string(),
            year: value.year().to_string(),
            oil_quantity: value.oil_quantity().to_string(),
            oil_type: value.oil_type().to_string(),
            interventions: value
                .intervention()
                .into_iter()
                .map(|x| x.clone().into())
                .collect(),
        }
    }
}

pub struct ClientTabCarIntervention {
    pub intervention_type: String,
    pub amount: String,
    pub milage: String,
    pub intervention_date: String,
}

impl From<Intervention> for ClientTabCarIntervention {
    fn from(value: Intervention) -> Self {
        Self {
            intervention_type: value.intervention_type().to_string(),
            amount: value.price().to_string(),
            milage: value.mileage().to_string(),
            intervention_date: value.intervention_date().to_string(),
        }
    }
}
