use http::StatusCode;
use serde::{Deserialize, Serialize};
use std::{fmt::Debug, string::ToString};
use warp::{http::Response, Reply};

/**
 * Descripbes the respose state, might be removed since HTTP codes exist.
 */
#[derive(Serialize, Deserialize, Debug)]
pub enum State {
    Ok,
    Info,
    Error,
}

/**
 * Sets the reponse format for other than GraphQL responses.
 */
#[derive(Serialize, Deserialize, Debug)]
pub struct Res<T>
where
    T: Serialize + Debug,
{
    #[serde(skip)]
    code: StatusCode,
    state: State,
    /**
     * Short description of the action taken, or why it failed.
     */
    message: String,
    data: Option<T>,
}

impl<T> Res<T>
where
    T: Serialize + Debug,
{
    /**
     * Template for an OK response. Data is expected to be included.
     */
    pub fn ok<S: ToString>(msg: S, data: T) -> Res<T> {
        Res {
            code: StatusCode::OK,
            state: State::Ok,
            message: msg.to_string(),
            data: Some(data),
        }
    }

    /**
     * Template for an info response. Data might be included.
     * Set the appropriate HTTP status code and state.
     */
    pub fn info<S: ToString>(msg: S, data: Option<T>) -> Res<T> {
        Res {
            code: StatusCode::OK,
            state: State::Info,
            message: msg.to_string(),
            data,
        }
    }

    /**
     * Template for an error response. Data is never included.S
     * By default uses BAD_REQUEST (400) status code and an `Error` -state.
     */
    pub fn error<S: ToString>(msg: S) -> Res<()> {
        Res {
            code: StatusCode::BAD_REQUEST,
            state: State::Error,
            message: msg.to_string(),
            data: None,
        }
    }

    /**
     * Set the status code safely.
     */
    pub fn set_status(mut self, status_code: StatusCode) {
        self.code = status_code;
    }

    /**
     * If the status code is invalid, it will not be changed.
     */
    pub fn status_from_u16(mut self, status_code: u16) -> Self {
        if let Ok(s) = StatusCode::from_u16(status_code) {
            self.code = s;
        }

        self
    }

    /**
     * Converts itself to a HTTP response.
     */
    pub fn done(self) -> Response<warp::hyper::Body> {
        Response::builder()
            .status(self.code)
            .header("Content-Type", "application/json")
            .body(warp::hyper::Body::from(
                serde_json::to_string(&self).unwrap(),
            ))
            .unwrap()
    }
}

/**
 * `Reply` -implementation for easy returning from a handler, without calling `.done()`.
 */
impl<T> Reply for Res<T>
where
    T: Serialize + Debug + Send,
{
    fn into_response(self) -> Response<warp::hyper::Body> {
        self.done()
    }
}

#[cfg(test)]
mod tests {
    use super::*;

    #[test]
    fn res_content_type() {
        let res = Res::<()>::error("").done();

        // Body is always JSON so content type should match
        assert_eq!(
            res.headers().get("Content-Type").unwrap(),
            &"application/json"
        );
    }

    #[test]
    fn res_ok() {
        let res = Res::ok("", 1).done();

        // Should be the default status
        assert_eq!(res.status(), http::StatusCode::OK);
    }

    #[test]
    fn res_error() {
        let res = Res::<()>::error("").done();

        // Should be the default status
        assert_eq!(res.status(), http::StatusCode::BAD_REQUEST);
    }

    #[test]
    fn res_set_status_invalid() {
        let mut res_builder = Res::<()>::error("");

        // Set status to something invalid
        res_builder.status_from_u16(1);

        let res = res_builder.done();

        // The status code should not change
        assert_eq!(res.status(), StatusCode::BAD_REQUEST);
    }
}
