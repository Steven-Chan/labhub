use rocket::http::Status;
use rocket::request::{self, FromRequest, Request};
use rocket::Outcome;

#[derive(Debug)]
pub enum GitLabTokenError {
    InvalidToken,
}

#[derive(Debug)]
pub enum RequestError {
    BadCount,
    Missing,
}

pub struct XGitLabEvent(pub String);

impl<'a, 'r> FromRequest<'a, 'r> for XGitLabEvent {
    type Error = RequestError;

    fn from_request(request: &'a Request<'r>) -> request::Outcome<Self, Self::Error> {
        let events: Vec<_> = request.headers().get("X-GitLab-Event").collect();

        match events.len() {
            0 => Outcome::Failure((Status::BadRequest, RequestError::Missing)),
            1 => Outcome::Success(XGitLabEvent(events[0].to_string())),
            _ => Outcome::Failure((Status::BadRequest, RequestError::BadCount)),
        }
    }
}

pub struct XGitLabToken(pub String);

impl<'a, 'r> FromRequest<'a, 'r> for XGitLabToken {
    type Error = RequestError;

    fn from_request(request: &'a Request<'r>) -> request::Outcome<Self, Self::Error> {
        let signatures: Vec<_> = request.headers().get("X-Gitlab-Token").collect();

        match signatures.len() {
            0 => Outcome::Failure((Status::BadRequest, RequestError::Missing)),
            1 => Outcome::Success(XGitLabToken(signatures[0].to_string())),
            _ => Outcome::Failure((Status::BadRequest, RequestError::BadCount)),
        }
    }
}
