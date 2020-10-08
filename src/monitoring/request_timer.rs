use rocket::fairing::{Fairing, Info, Kind};
use chrono::Utc;
use rocket::{Request, Response, Data};

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
        log::info!("on_request: {}", &instant);
        request.local_cache(|| instant);
    }

    fn on_response(&self, request: &Request, _response: &mut Response) {
        let instant = Utc::now().timestamp_millis();
        log::info!("on_response: {}", &instant);
        let cached = request.local_cache(|| instant).to_owned();
        let delta = Utc::now().timestamp_millis() - cached;
        log::info!("For endpoint {} the request processing duration was {} [ms]", request.uri().path(), delta)
    }
}
