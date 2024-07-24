use std::collections::HashMap;

use crate::{request::HttpRequest, response::{HttpResponse, HttpResponseBuilder}};

pub fn home(_req: HttpRequest, _params: HashMap<String, String>) -> HttpResponse {
    HttpResponseBuilder::default()
        .status(200, Some("OK"))
        .build()
}

