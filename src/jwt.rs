use actix_web::dev::Payload;
use actix_web::{Error, FromRequest, HttpRequest};
use futures::future::{Ready, ok};

pub struct JWToken {
    pub message: String
}

impl FromRequest for JWToken {
    type Error = Error;
    type Future = Ready<Result<Self, Error>>;

    fn from_request(req: &HttpRequest, payload: &mut Payload) -> Self::Future {
        match req.headers().get("token") {
            Some(data) => {
                let token = JWToken {
                    message: data.to_str().unwrap().to_string()
                };
                ok(token)
            },
            None => {
                let token = JWToken {
                    message: String::from("nothing found")
                };
                ok(token)
            }
        }

    }
}