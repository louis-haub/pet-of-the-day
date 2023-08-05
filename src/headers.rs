use rocket::http::Status;
use rocket::Request;
use rocket::request::{FromRequest, Outcome};

pub struct InteractionHandshake {
    ed25519: String,
    timestamp: String,
}

#[async_trait]
impl<'r> FromRequest<'r> for InteractionHandshake {
    type Error = String;

    async fn from_request(request: &'r Request<'_>) -> Outcome<Self, Self::Error> {
        let result_fn: fn(&'r Request<'_>) -> Option<Self> = |request| {
            let ed25519 = request.headers().get_one("X-Signature-Ed25519")?.to_owned();
            let timestamp = request.headers().get_one("X-Signature-Timestamp")?.to_owned();

            Some(InteractionHandshake{
                ed25519,
                timestamp
            })
        };

        return match result_fn(request){
            None => {Outcome::Failure((Status::Unauthorized, "Failed to validate Request".to_owned()))}
            Some(res) => {Outcome::Success(res)}
        }
    }
}