use std::collections::HashMap;

use crate::{request::HttpRequest, response::{HttpResponse, HttpResponseBuilder}};

pub fn echo(_req: HttpRequest, params: HashMap<String, String>) -> HttpResponse {
    let val = params.get("message");

    let mut builder = HttpResponseBuilder::default().status(200, Some("OK"));
    if val.is_some() {
        builder = builder.body(val.unwrap());
    };

    builder.build()
}

