use askama_actix::Template;

#[derive(Template)]
#[template(path = "home.html")]
pub struct HomeTemplate;
