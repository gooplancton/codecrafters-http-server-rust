use crate::{handlers::{echo, home, user_agent}, request::HttpRequest, response::HttpResponse};

pub trait HttpRouter {
    fn get_handler(self: &Self, target: &impl AsRef<str>) -> Option<fn (request: HttpRequest) -> HttpResponse>;
}

pub struct SimpleRouter;

impl HttpRouter for SimpleRouter {
    fn get_handler(self: &Self, target: &impl AsRef<str>) -> Option<fn (request: HttpRequest) -> HttpResponse> {
        match target.as_ref() {
            target if target == "/" => Some(home),
            target if target.starts_with("/echo") => Some(echo),
            target if target == "/user-agent" => Some(user_agent),
            _ => None
        }
    }
}

