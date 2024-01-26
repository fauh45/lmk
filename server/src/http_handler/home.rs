use actix_web::{get, HttpRequest};

use crate::template_struct::home::HomeTemplate;

#[get("/")]
async fn home(_: HttpRequest) -> HomeTemplate {
    HomeTemplate {}
}
