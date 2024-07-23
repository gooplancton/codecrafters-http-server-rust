use crate::{request::HttpRequest, response::{HttpResponse, HttpResponseBuilder}};

pub fn home(_req: HttpRequest) -> HttpResponse {
    HttpResponseBuilder::default()
        .status(200, Some("OK"))
        .build()
}

