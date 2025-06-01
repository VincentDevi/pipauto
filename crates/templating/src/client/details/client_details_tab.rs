use askama::Template;

#[derive(Template)]
#[template(path = "client_tab_details.html")]
pub struct ClientTabDetailTemplate {
    pub first_name: String,
    pub last_name: String,
    pub country: String,
    pub street: String,
    pub number: String,
    pub postal: String,
    pub phone: Option<String>,
    pub email: Option<String>,
}
