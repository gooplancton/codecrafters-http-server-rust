use std::collections::HashMap;

use crate::{
    request::HttpRequest,
    response::{HttpResponse, HttpResponseBuilder},
    router::HttpError,
};

pub fn echo(
    mut _req: HttpRequest,
    mut params: HashMap<String, String>,
) -> Result<HttpResponse, HttpError> {
    let val = params
        .remove("message")
        .ok_or(HttpError::new(400, Some("Missing message")))?;

    let builder = HttpResponseBuilder::default()
        .status(200, Some("OK"))
        .body(val);

    Ok(builder.build())
}
