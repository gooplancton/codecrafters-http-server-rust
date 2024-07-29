use crate::{
    request::HttpRequest,
    response::{HttpResponse, HttpResponseBuilder},
    router::{HttpError, HttpRequestParams},
};

pub fn query(
    req: HttpRequest,
    mut params: HttpRequestParams,
) -> Result<HttpResponse, HttpError> {
    let query_param = params.remove("query_param").unwrap();
    let query_value = req.query.unwrap().remove(&query_param).unwrap_or("Missing".into());

    let res = HttpResponseBuilder::default()
        .status(200, Some("OK"))
        .body(query_value)
        .build();

    Ok(res)
}
