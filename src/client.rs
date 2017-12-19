use hex::ToHex;
use errors::*;
use reqwest;
use reqwest::{StatusCode, Response};
use reqwest::header::{Headers, UserAgent, ContentType};
use std::io::Read;
use ring::{digest, hmac};

static API1_HOST : &'static str = "https://api.bitfinex.com/v2/";

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

    fn handler(&self, mut response: Response) -> Result<(String)> {
        match response.status() {
            StatusCode::Ok => {
                let mut body = String::new();
                response.read_to_string(&mut body).unwrap();
                return Ok(body);
            },
            StatusCode::InternalServerError => {
                bail!("Internal Server Error");
            }
            StatusCode::ServiceUnavailable => {
                bail!("Service Unavailable");
            }
            StatusCode::Unauthorized => {
                bail!("Unauthorized");
            }            
            StatusCode::BadRequest => {
                bail!(format!("Bad Request: {:?}", response));
            }                        
            s => {
                bail!(format!("Received response: {:?}", s));
            }
        };
    }

}