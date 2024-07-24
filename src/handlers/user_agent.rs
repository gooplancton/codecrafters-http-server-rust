use crate::{
    request::HttpRequest,
    response::{HttpResponse, HttpResponseBuilder},
    router::{HttpError, HttpRequestParams},
};

pub fn user_agent(
    mut req: HttpRequest,
    mut _params: HttpRequestParams,
) -> Result<HttpResponse, HttpError> {
    let user_agent = req
        .headers
        .remove("user-agent")
        .ok_or(HttpError::new(400, Some("Missing user-agent header")))?;

    let res = HttpResponseBuilder::default()
        .status(200, Some("OK"))
        .body(user_agent)
        .build();

    Ok(res)
}
