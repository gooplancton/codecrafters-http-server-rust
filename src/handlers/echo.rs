use crate::{request::HttpRequest, response::{HttpResponse, HttpResponseBuilder}};

pub fn echo(req: HttpRequest) -> HttpResponse {
    let (_, val) = req.target.split_once("/echo").unwrap();

    HttpResponseBuilder::default()
        .status(200, Some("OK"))
        .body(val)
        .build()
}

