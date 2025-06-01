use askama::Template;

#[derive(Template)]
#[template(path = "home.html")]
pub struct HomePage {
    pub name: String,
}
