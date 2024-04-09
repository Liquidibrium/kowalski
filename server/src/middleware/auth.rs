use axum::body::Body;
use axum::extract::Request;
use axum::response::Response;
use http::StatusCode;
use std::convert::Infallible;

fn auth_middleware(
) -> impl tower::Service<Request<Body>, Response = Response<Body>, Error = Infallible> + Clone {
    tower::service_fn(|req: Request<Body>| async move {
        let token = req
            .headers()
            .get("Authorization")
            .and_then(|value| value.to_str().ok());
        if token.is_none() {
            return Ok(Response::builder()
                .status(StatusCode::UNAUTHORIZED)
                .body(Body::empty())
                .unwrap());
        }
        let token = token.unwrap();
        if token != "Bearer token" {
            return Ok(Response::builder()
                .status(StatusCode::UNAUTHORIZED)
                .body(Body::empty())
                .unwrap());
        }
        Ok(Response::builder()
            .status(StatusCode::OK)
            .body(Body::empty())
            .unwrap())
    })
}
