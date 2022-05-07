
use rocket::http::{Method, Status};
use rocket::fairing::{Fairing, Info, Kind};
use rocket::{Request, Response};
use rocket::http::hyper::header::{ACCESS_CONTROL_ALLOW_HEADERS, ACCESS_CONTROL_ALLOW_ORIGIN, CONTENT_TYPE};

pub struct Cors;

#[rocket::async_trait]
impl Fairing for Cors {
    fn info(&self) -> Info {
        Info {
            name: "Cors handler",
            kind: Kind::Response
        }
    }

    async fn on_response<'r>(&self, _req: &'r Request<'_>, _res: &mut Response<'r>) {
        _res.set_raw_header(ACCESS_CONTROL_ALLOW_ORIGIN.as_str(), "*");
        let allowed_headers= format!("{}, {}", ACCESS_CONTROL_ALLOW_ORIGIN.as_str(), CONTENT_TYPE.as_str());
        _res.set_raw_header(ACCESS_CONTROL_ALLOW_HEADERS.as_str(), allowed_headers);
        if _res.status() != Status::NotFound {
            return
        }

        if _res.status() == Status::NotFound && _req.method() == Method::Options {
            _res.set_status(Status::Ok);
        }
    }
}