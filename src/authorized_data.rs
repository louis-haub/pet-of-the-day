use ed25519_dalek::{Verifier, VerifyingKey};
use rocket::data::{Capped, FromData, Outcome, ToByteUnit};
use rocket::{Data, Request};
use crate::config::Config;

pub struct AuthorizedRequest {
    pub data: Capped<Vec<u8>>
}

#[async_trait]
impl<'a> FromData<'a> for AuthorizedRequest {
    type Error = ();

    async fn from_data(req: &'a Request<'_>, data: Data<'a>) -> Outcome<'a, Self> {
        match verify(req, data).await {
            Some(data) => Outcome::Success(AuthorizedRequest { data }),
            None => Outcome::Failure((rocket::http::Status::Unauthorized, ()))
        }
    }
}

async fn verify(req: &Request<'_>, data: Data<'_>) -> Option<Capped<Vec<u8>>> {
    let config = Config::create_and_validate();
    let public_key_string = config.public_key;
    let public_key = VerifyingKey::from_bytes(&hex::decode(public_key_string).ok()?.as_slice().try_into().ok()?).unwrap();
    let ed25519 = req.headers().get_one("X-Signature-Ed25519")?;
    let timestamp = req.headers().get_one("X-Signature-Timestamp")?;
    let body: Capped<Vec<u8>> = data.open(2.mebibytes()).into_bytes().await.ok()?.to_owned();
    let mut msg = Vec::new();
    msg.extend(timestamp.as_bytes());
    msg.extend(body.iter());

    match public_key.verify(msg.as_slice(), &hex::decode(ed25519).ok()?.as_slice().try_into().ok()?){
        Ok(_) => Some(body),
        Err(_) => None
    }
}