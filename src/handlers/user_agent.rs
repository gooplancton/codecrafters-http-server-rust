use std::collections::HashMap;

use crate::{
    request::HttpRequest,
    response::{HttpResponse, HttpResponseBuilder},
};

pub fn user_agent(req: HttpRequest, _params: HashMap<String, String>) -> HttpResponse {
    HttpResponseBuilder::default()
        .status(200, Some("OK"))
        .body(
            req.headers
                .get("user-agent")
                .map(|s| s.as_ref())
                .unwrap_or(""),
        )
        .build()
}
