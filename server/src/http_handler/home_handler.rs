use actix_web::{get, HttpRequest};

use crate::template_struct::home::HomeTemplate;

#[get("/")]
async fn index(_: HttpRequest) -> HomeTemplate {
    HomeTemplate {}
}
