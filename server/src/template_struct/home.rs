use askama::Template;

#[derive(Template)]
#[template(path = "home.j2")]
pub struct HomeTemplate;
