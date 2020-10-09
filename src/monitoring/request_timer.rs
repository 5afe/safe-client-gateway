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
        request.local_cache(|| Utc::now().timestamp_millis());
    }

    fn on_response(&self, request: &Request, _response: &mut Response) {
        let path_data = request.route().map(|route| route.uri.to_string()).unwrap_or(String::from(request.uri().path()));
        let cached = request.local_cache(|| Utc::now().timestamp_millis()).to_owned();
        let delta = Utc::now().timestamp_millis() - cached;
        log::info!("response_time_ms::{}::{}",path_data , delta)
    }
}
