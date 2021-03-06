use super::Response;
use crate::CONFIG;
use reqwest::{self, header::CONTENT_TYPE};

fn url_from_config() -> String {
    format!(
        "{}://{}:{}{}",
        &CONFIG.read().k2.scheme,
        &CONFIG.read().k2.host,
        &CONFIG.read().k2.port,
        &CONFIG.read().k2.path
    )
}

pub fn request() -> Response {
    let url = url_from_config();
    match reqwest::get(&url) {
        Ok(ref mut response) => {
            let headers = response.headers().clone();
            if let Some(content_header) = headers.get(CONTENT_TYPE) {
                if content_header == "application/json" {
                    match response.json() {
                        Ok(json) => json,
                        Err(ref err) if err.is_redirect() => {
                            panic!("Redirect loop when attempting to get JSON from K2")
                        }
                        Err(ref err) if err.is_serialization() => {
                            if let Some(serde_error) = err.get_ref() {
                                panic!(
                                    "Unable to deserialize payload to JSON. Status: {:?} - Error: {:?}",
                                    response.status(), serde_error
                                )
                            } else {
                                panic!(
                                    "Unable to deserialize payload for unknown reason. Status: {:?}",
                                    response.status()
                                )
                            }
                        }
                        Err(err) => panic!(
                            "Unable to receive JSON from K2. Status: {:?} - Error: {:?}",
                            response.status(),
                            err
                        ),
                    }
                } else {
                    super::handle_response_failure(response)
                }
            } else {
                super::handle_response_failure(response)
            }
        }
        Err(err) => panic!("Request failed: {:?}", err),
    }
}
