use askama::Template;

#[derive(Template)]
#[template(path = "home.html")]
pub struct HomeTemplate<'a> {
    id: &'a str,
}
