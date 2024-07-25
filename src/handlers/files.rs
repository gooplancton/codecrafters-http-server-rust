use crate::{
    request::HttpRequest,
    response::{HttpResponse, HttpResponseBuilder},
    router::{HttpError, HttpRequestParams},
};
use std::{
    env, fs,
    path::{Path, PathBuf},
};

pub fn get_file(
    mut _req: HttpRequest,
    mut params: HttpRequestParams,
) -> Result<HttpResponse, HttpError> {
    let filename = params
        .remove("filename")
        .ok_or(HttpError::new(400, Some("Missing filename")))?;

    let data_dir = env::var("DATA_DIR").unwrap();
    let file_path = Path::join(&PathBuf::from(data_dir), filename);

    let contents = fs::read_to_string(file_path)
        .map_err(|_err| HttpError::new(404, Some("Not Found")))
        .map(|contents| contents.bytes().collect::<Vec<_>>())?;

    let res = HttpResponseBuilder::default()
        .status(200, Some("OK"))
        .header("Content-Type", "application/octet-stream")
        .body(contents)
        .build();

    Ok(res)
}

pub fn create_file(
    req: HttpRequest,
    mut params: HttpRequestParams,
) -> Result<HttpResponse, HttpError> {
    let filename = params
        .remove("filename")
        .ok_or(HttpError::new(400, Some("Missing filename")))?;

    let data_dir = env::var("DATA_DIR").unwrap();
    let file_path = Path::join(&PathBuf::from(data_dir), filename);

    if req.body.is_none() {
        return Err(HttpError::new(422, Some("No body")));
    }

    fs::write(file_path, req.body.unwrap()).map_err(|_err| HttpError::new(404, Some("Not Found")))?;

    let res = HttpResponseBuilder::default()
        .status(201, Some("Created"))
        .header("Content-Type", "application/octet-stream")
        .build();

    Ok(res)
}
