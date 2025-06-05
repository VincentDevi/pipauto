use super::*;
use askama::Template;

#[derive(Template)]
#[template(path = "clients_table.html")]
pub struct ClientsTableTemplate {
    pub clients: Vec<ClientTemp>,
    pub paging: Paging,
}
