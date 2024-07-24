use crate::{
    request::HttpRequest,
    response::{HttpResponse, HttpResponseBuilder},
    router::{HttpError, HttpRequestParams},
};
use std::fs;

pub fn files(
    mut _req: HttpRequest,
    mut params: HttpRequestParams,
) -> Result<HttpResponse, HttpError> {
    let filename = params
        .remove("filename")
        .ok_or(HttpError::new(400, Some("Missing filename")))?;

    let contents = fs::read_to_string(filename)
        .map_err(|err| HttpError::new(500, Some(err.to_string())))
        .map(|contents| contents.bytes().collect::<Vec<_>>())?;

    let res = HttpResponseBuilder::default()
        .header("Content-Type", "application/octet-stream")
        .body(contents)
        .build();

    Ok(res)
}
