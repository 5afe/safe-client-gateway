use rocket::fairing::{Fairing, Info, Kind};
use chrono::Utc;
use rocket::{Request, Response, Data};
use rocket::http::Header;

pub struct RequestTimer();

impl Fairing for RequestTimer {
    fn info(&self) -> Info {
        Info {
            name: "RequestTimer",
            kind: Kind::Request | Kind::Response,
        }
    }

    fn on_request(&self, request: &mut Request, _data: &Data) {
        let instant = Utc::now().timestamp_millis();
        log::debug!("on_request: {}", &instant);
        let h = Header::new("requested_timestamp", instant.to_string());
        request.add_header(h)
    }

    fn on_response(&self, request: &Request, _response: &mut Response) {
        if let Some(requested_timestamp) = request.headers().get("requested_timestamp").next().map(|it| it.parse::<i64>().ok()).flatten() {
            let instant = Utc::now().timestamp_millis();
            let delta = instant - requested_timestamp;
            log::debug!("For endpoint {} the request processing duration was {} [ms]", request.uri().path(), delta)
        }
    }
}
