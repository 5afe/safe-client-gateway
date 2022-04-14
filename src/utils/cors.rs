use rocket::fairing::{Fairing, Info, Kind};
use rocket::http::{ContentType, Header, Method, Status};
use rocket::{Request, Response};

pub struct CORS();

#[rocket::async_trait]
impl Fairing for CORS {
    fn info(&self) -> Info {
        Info {
            name: "Add CORS headers to requests",
            kind: Kind::Response,
        }
    }

    async fn on_response<'r>(&self, request: &'r Request<'_>, response: &mut Response<'r>) {
        // https://github.com/lawliet89/rocket_cors/blob/master/examples/fairing.rs
        if request.method() == Method::Options || response.content_type() == Some(ContentType::JSON)
        {
            response.set_header(Header::new("Access-Control-Allow-Origin", "*"));
            response.set_header(Header::new(
                "Access-Control-Allow-Methods",
                "POST, GET, OPTIONS",
            ));
            response.set_header(Header::new(
                "Access-Control-Allow-Headers",
                "X-Requested-With, Content-Type, Authorization",
            ));
            response.set_header(Header::new("Access-Control-Allow-Credentials", "true"));
        }
        if request.method() == Method::Options && request.route().is_none() {
            response.set_header(ContentType::Plain);
            response.set_status(Status::NoContent);
            let _ = response.body_mut().take();
        }
    }
}
