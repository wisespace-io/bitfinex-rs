use errors::*;
use reqwest;
use reqwest::{StatusCode, Response};
use reqwest::header::{HeaderMap, HeaderName, HeaderValue, USER_AGENT, CONTENT_TYPE};
use std::io::Read;
use ring::{digest, hmac};
use hex::encode as hex_encode;
use std::time::{SystemTime, UNIX_EPOCH};

static API1_HOST : &'static str = "https://api.bitfinex.com/v2/";
static API_SIGNATURE_PATH : &'static str = "/api/v2/auth/r/";

#[derive(Clone)]
pub struct Client {
    api_key: String,
    secret_key: String
}

impl Client {
    pub fn new(api_key: Option<String>, secret_key: Option<String>) -> Self {
        Client {
            api_key : api_key.unwrap_or("".into()),
            secret_key : secret_key.unwrap_or("".into())
        }
    }

    pub fn get(&self, endpoint: String, request: String) -> Result<(String)> {
        let mut url: String = format!("{}{}", API1_HOST, endpoint);
        if !request.is_empty() {
            url.push_str(format!("?{}", request).as_str());
        }

        let response = reqwest::get(url.as_str())?;

        self.handler(response)
    }

    pub fn post_signed(&self, request: String, payload: String) -> Result<(String)> {
        let url: String = format!("{}auth/r/{}", API1_HOST, request);

        let client = reqwest::Client::new();
        let response = client.post(url.as_str())
            .headers(self.build_headers(request, payload.clone())?)
            .body(payload)
            .send()?;

        self.handler(response)
    }

    fn build_headers(&self, request: String, payload: String) -> Result<HeaderMap> {
        let nonce: String = self.generate_nonce()?;
        let signature_path: String = format!("{}{}{}{}", API_SIGNATURE_PATH, request, nonce, payload);

        let signed_key = hmac::SigningKey::new(&digest::SHA384, self.secret_key.as_bytes());
        let signature = hex_encode(hmac::sign(&signed_key, signature_path.as_bytes()).as_ref());

        let mut headers = HeaderMap::new();
        headers.insert(USER_AGENT, HeaderValue::from_static("bitfinex-rs"));
        headers.insert(HeaderName::from_static("bfx-nonce"), HeaderValue::from_str(nonce.as_str())?);
        headers.insert(HeaderName::from_static("bfx-apikey"), HeaderValue::from_str(self.api_key.as_str())?);
        headers.insert(HeaderName::from_static("bfx-signature"), HeaderValue::from_str(signature.as_str())?);
        headers.insert(CONTENT_TYPE, HeaderValue::from_static("application/json"));

        Ok(headers)
    }

    fn generate_nonce(&self) -> Result<String> {
        let start = SystemTime::now();
        let since_epoch = start.duration_since(UNIX_EPOCH)?;

        let timestamp = since_epoch.as_secs() * 1000 + since_epoch.subsec_nanos() as u64 / 1_000_000;

        Ok((timestamp + 1).to_string())
    }

    fn handler(&self, mut response: Response) -> Result<(String)> {
        match response.status() {
            StatusCode::OK => {
                let mut body = String::new();
                response.read_to_string(&mut body)?;
                return Ok(body);
            },
            StatusCode::INTERNAL_SERVER_ERROR => {
                bail!("Internal Server Error");
            }
            StatusCode::SERVICE_UNAVAILABLE => {
                bail!("Service Unavailable");
            }
            StatusCode::UNAUTHORIZED => {
                bail!("Unauthorized");
            }
            StatusCode::BAD_REQUEST => {
                bail!(format!("Bad Request: {:?}", response));
            }
            s => {
                bail!(format!("Received response: {:?}", s));
            }
        };
    }

}
