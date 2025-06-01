use askama::Template;
use common::client::Client;

#[derive(Template)]
#[template(path = "clients.html")]
pub struct ClientsTemplate {
    pub clients: Vec<ClientTemp>,
    pub paging: Paging,
}

#[derive(Debug, Clone, Copy)]
pub struct Paging {
    pub start: u32,
    pub count: u32,
}

#[derive(Debug, Clone)]
pub struct ClientTemp {
    pub name: String,
    pub address: String,
    pub phone: Option<String>,
    pub email: Option<String>,
}

impl From<Client> for ClientTemp {
    fn from(value: Client) -> Self {
        Self {
            name: value.full_name(),
            address: value.address().to_string(),
            phone: value.phone().map(|x| x.to_string()),
            email: value.email().map(|x| x.to_string()),
        }
    }
}
