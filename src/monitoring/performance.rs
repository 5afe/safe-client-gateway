use chrono::Utc;
use rocket::fairing::{Fairing, Info, Kind};
use rocket::{Data, Request, Response};

pub struct PerformanceMonitor();

impl Fairing for PerformanceMonitor {
    fn info(&self) -> Info {
        Info {
            name: "PerformanceMonitor",
            kind: Kind::Request | Kind::Response,
        }
    }

    fn on_request(&self, request: &mut Request, _data: &Data) {
        request.local_cache(|| Utc::now().timestamp_millis());
    }

    fn on_response(&self, request: &Request, response: &mut Response) {
        let path_data = request
            .route()
            .map(|route| route.uri.to_string())
            .unwrap_or(String::from(request.uri().path()));
        let cached = request
            .local_cache(|| Utc::now().timestamp_millis())
            .to_owned();
        let method = request.method().as_str();
        let status_code = response.status().code;
        let delta = Utc::now().timestamp_millis() - cached;
        log::info!("MT::{}::{}::{}::{}", method, path_data, delta, status_code)
    }
}
