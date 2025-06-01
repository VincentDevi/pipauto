use askama::Template;
use common::car::Car;

#[derive(Template)]
#[template(path = "cars.html")]
pub struct CarsTemplate {
    pub cars: Vec<CarsTemp>,
}

#[derive(Debug, Clone)]
pub struct CarsTemp {
    pub cc: String,
    pub brand: String,
    pub oil_type: String,
    pub oil_quantity: String,
    pub year: String,
}

impl From<Car> for CarsTemp {
    fn from(value: Car) -> Self {
        Self {
            cc: value.cc().to_string(),
            brand: value.brand().to_string(),
            oil_type: value.oil_type().to_string(),
            oil_quantity: value.oil_quantity().to_string(),
            year: value.year().to_string(),
        }
    }
}
