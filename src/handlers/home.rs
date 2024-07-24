use crate::{
    request::HttpRequest,
    response::{HttpResponse, HttpResponseBuilder},
    router::{HttpError, HttpRequestParams},
};

pub fn home(
    mut _req: HttpRequest,
    mut _params: HttpRequestParams,
) -> Result<HttpResponse, HttpError> {
    let res = HttpResponseBuilder::default()
        .status(200, Some("OK"))
        .build();

    Ok(res)
}
