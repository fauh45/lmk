use askama_actix::Template;

#[derive(Template)]
#[template(path = "events.html")]
pub struct EventsTemplate {
    pub identifier: String,
}
