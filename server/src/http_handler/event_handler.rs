use actix_web::{get, web, HttpRequest};

use crate::template_struct::events::EventsTemplate;

#[get("/events/{identifier}")]
pub async fn events(_: HttpRequest, path: web::Path<String>) -> EventsTemplate {
    let identifier = path.into_inner();

    EventsTemplate { identifier }
}
