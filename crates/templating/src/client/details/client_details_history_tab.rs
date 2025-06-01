use askama::Template;
use common::intervention::SpecificInterventionWithCar;

#[derive(Template)]
#[template(path = "client_tab_history.html")]
pub struct ClientTabHistoryTemplate {
    pub interventions: Vec<InterventionHistoryTemplate>,
}

impl From<SpecificInterventionWithCar> for InterventionHistoryTemplate {
    fn from(value: SpecificInterventionWithCar) -> Self {
        Self {
            intervention_type: value.intervention_type().to_string(),
            intervention_date: value.intervention_date().to_string(),
            car: InterventionCarHistory {
                brand: value.car().brand.clone(),
                model: value.car().model.clone(),
            },
        }
    }
}

pub struct InterventionHistoryTemplate {
    pub intervention_type: String,
    pub intervention_date: String,
    pub car: InterventionCarHistory,
}

pub struct InterventionCarHistory {
    pub brand: String,
    pub model: String,
}
