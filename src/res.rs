use actix_web::{http::StatusCode, Error, HttpRequest, HttpResponse, Responder};
use futures::future::{ready, Ready};
use serde::Serialize;
use std::convert::TryInto;

/**
 * General response format for other than GQL responses.
 * Supports a single generic data type, which must implement `serde::Serialize`.
 * Since associated functions "consume" the instance,
 * this is best used at the return line.
 */
pub struct Res<D>
where
    D: Serialize,
{
    status_code: StatusCode,
    body: Body<D>,
}

impl<D> Res<D>
where
    D: Serialize,
{
    /**
     * New response, no data, `State::Ok` and status code 200.
     */
    pub fn new() -> Res<D> {
        Res {
            status_code: StatusCode::OK,
            body: Body {
                state: State::Ok,
                message: "No message.".to_string(),
                data: None,
            },
        }
    }

    /**
     * Response with an Ok -state and 200 HTTP status.
     * Data is expected to be included.
     * If no data it required, set it to and empty tuple: `()`.
     */
    pub fn ok<S: Into<String>>(msg: S, data: D) -> Res<D> {
        Res {
            status_code: StatusCode::OK,
            body: Body {
                state: State::Ok,
                message: msg.into(),
                data: Some(data),
            },
        }
    }

    /**
     * Response where data might be included,
     */
    pub fn info<S: Into<String>>(msg: S, data: Option<D>) -> Res<D> {
        Res {
            status_code: StatusCode::OK,
            body: Body {
                state: State::Info,
                message: msg.into(),
                data,
            },
        }
    }

    /**
     * An error reponse with no data. Defaults to HTTP status 400, and should be changed if needed.
     */
    pub fn error<S: Into<String>>(msg: S) -> Res<()> {
        Res {
            status_code: StatusCode::BAD_REQUEST,
            body: Body {
                state: State::Error,
                message: msg.into(),
                data: None::<()>,
            },
        }
    }

    /**
     * Set the data again. Must be of the same type.
     */
    pub fn data(mut self, data: Option<D>) -> Self {
        self.body.data = data;
        self
    }

    /**
     * Set the status code. If the status code is not valid, does not change it.
     */
    pub fn status<C: TryInto<StatusCode>>(mut self, status: C) -> Res<D> {
        if let Ok(value) = status.try_into() {
            self.status_code = value;
        }

        self
    }

    /**
     * Convert self to a `HttpResponse`.
     */
    pub fn to_response(self) -> HttpResponse {
        let body = serde_json::to_string(&self.body).unwrap();

        HttpResponse::Ok()
            .content_type("application/json")
            .status(self.status_code)
            .body(body)
    }
}

#[derive(Serialize)]
struct Body<D>
where
    D: Serialize,
{
    state: State,
    message: String,
    data: Option<D>,
}

#[derive(Serialize, PartialEq, Debug)]
enum State {
    Ok,
    Info,
    Error,
}

/**
 * Implement actix-web's Responder -trait, so Res can be directly returned from handlers.
 */
impl<D> Responder for Res<D>
where
    D: Serialize,
{
    type Error = Error;
    type Future = Ready<Result<HttpResponse, Error>>;

    fn respond_to(self, _req: &HttpRequest) -> Self::Future {
        ready(Ok(self.to_response()))
    }
}

#[cfg(test)]
mod tests {
    use super::*;

    #[test]
    fn res_ok() {
        let r = Res::ok("", ());

        assert_eq!(r.body.state, State::Ok);

        assert_eq!(StatusCode::OK, r.to_response().status());
    }

    #[test]
    fn res_info() {
        let r = Res::info("", None::<()>);

        assert_eq!(r.body.state, State::Info);

        assert_eq!(StatusCode::OK, r.to_response().status());
    }

    #[test]
    fn res_error() {
        let r = Res::<()>::error("");

        assert_eq!(r.body.state, State::Error);

        assert_eq!(StatusCode::BAD_REQUEST, r.to_response().status());
    }
}
