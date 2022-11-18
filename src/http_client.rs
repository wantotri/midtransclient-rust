//! Http Client

use std::collections::HashMap;
use reqwest::{
    self,
    header::{self, HeaderMap},
    Proxy,
    Method
};
use serde_json::Value;
use crate::{MidtransError, error_midtrans::ApiError};

const CONTENT_TYPE: &'static str = "application/json";
const ACCEPT: &'static str = "application/json";
const USER_AGENT: &'static str = "midtransclient-rust";
const VERSION: &'static str = env!("CARGO_PKG_VERSION");

type MidtransResult = Result<HashMap<String, Value>, MidtransError>;

pub trait MidtransClient {
    fn request(
        &self,
        method: Method,
        server_key: &str,
        api_url: &str,
        parameters: &str,
        custom_headers: Option<HeaderMap>,
        proxies: Option<Proxy>
    ) -> MidtransResult {
        let parameters: HashMap<String, Value> = match parameters {
            "" => HashMap::new(),
            params => serde_json::from_str(params)?
        };

        let mut client = HttpClient::new();
        if let Some(headers) = custom_headers {
            client.custom_headers(headers);
        }
        if let Some(proxies) = proxies {
            client.proxies(proxies);
        }

        let request_builder = client.build()?
            .request(method.clone(), api_url)
            .basic_auth(server_key, Some(""));

        let request_builder = match method {
            Method::GET => request_builder.query(&parameters),
            _ => request_builder.json(&parameters)
        };

        let response = Box::new(request_builder.send()?);
        let response_header = response.headers().clone();
        let response_status_code = response.status().as_u16();
        let response_text = response.text()?;
        let mut response_hashmap: HashMap<String, Value> = serde_json::from_str(&response_text)?;

        if !response_hashmap.contains_key("status_code") {
            response_hashmap.insert("status_code".to_string(), Value::String(response_status_code.to_string()));
        }

        let status_code = match &response_hashmap["status_code"] {
            Value::String(code) => code.parse::<u16>()?,
            _ => 0
        };

        if status_code >= 400 {
            return Err(MidtransError::ApiError(
                ApiError::new(
                    status_code,
                    response_hashmap.clone(),
                    format!(
                        "Midtrans API is returning API error. \nHTTP status code: {}. \nAPI Response: \nHeader {:#?} \nBody {:#?}",
                        status_code,
                        response_header,
                        response_hashmap
                    )
                )
            ));
        }

        Ok(response_hashmap)
    }
}

/// Http Client Struct is wrapper to Rust's `reqwest` crate.
/// Used to do API call to Midtrans API urls.
pub struct HttpClient;

impl HttpClient {
    pub fn new() -> HttpClientBuilder {
        HttpClientBuilder {
            custom_headers: None,
            proxies: None
        }
    }
}

/// Builder for HttpClient
pub struct HttpClientBuilder {
    custom_headers: Option<header::HeaderMap>,
    proxies: Option<reqwest::Proxy>
}

impl HttpClientBuilder {
    pub fn custom_headers(&mut self, headers: header::HeaderMap) -> &mut Self {
        self.custom_headers = Some(headers);
        self
    }

    pub fn proxies(&mut self, proxies: reqwest::Proxy) -> &mut Self {
        self.proxies = Some(proxies);
        self
    }

    pub fn build(&self) -> reqwest::Result<reqwest::blocking::Client> {
        let user_agent = format!("{}/{}", USER_AGENT, VERSION);
        let mut headers = header::HeaderMap::new();
        headers.insert("content-type", header::HeaderValue::from_static(CONTENT_TYPE));
        headers.insert("accept", header::HeaderValue::from_static(ACCEPT));
        headers.insert("user-agent", user_agent.parse().unwrap());

        if let Some(custom_headers) = &self.custom_headers {
            for (key, val) in custom_headers.iter() {
                headers.insert(key, val.clone());
            }
        }

        let http_client = reqwest::blocking::Client::builder().default_headers(headers);
        let http_client = match &self.proxies {
            Some(proxies) => http_client.proxy(proxies.clone()).build()?,
            None => http_client.build()?
        };

        Ok(http_client)
    }
}

#[cfg(test)]
mod test {
    use super::*;

    mod http_client {
        use crate::http_client::HttpClient;

        #[test]
        fn new() {
            let http_client = HttpClient::new();
            assert!(http_client.custom_headers.is_none());
            assert!(http_client.proxies.is_none());
        }
    }

    mod http_client_builder {
        use reqwest::header;
        use crate::http_client::HttpClient;

        #[test]
        fn custom_headers() {
            let mut http_client = HttpClient::new();
            let mut headers = header::HeaderMap::new();
            headers.insert("X-Custom-Header", header::HeaderValue::from_static("Some Value"));
            http_client.custom_headers(headers.clone());
            assert_eq!(http_client.custom_headers, Some(headers));
        }

        #[test]
        fn proxies() {
            let mut http_client = HttpClient::new();
            let proxies = reqwest::Proxy::http("https://secure.example").unwrap();
            http_client.proxies(proxies);
        }

        #[test]
        fn build() {
            let _ = HttpClient::new().build().unwrap();
        }
    }

    mod request {
        use serde_json::json;
        use super::*;
        use std::env;

        struct TestClient;

        impl MidtransClient for TestClient {}

        fn server_key() -> String {
            env::var("MIDTRANS_SERVER_KEY").expect("SERVER_KEY NOT FOUND")
        }

        fn generate_order_id(test_number: u8) -> String {
            let now = chrono::offset::Local::now().format("%Y%m%d%H%M%S").to_string();
            format!("rust-midtransclient-test{}-{}", test_number, now)
        }

        fn generate_param_min(order_id: &str) -> String {
            json!({
                "transaction_details": {
                    "order_id": order_id,
                    "gross_amount": 200000
                }, "credit_card":{
                    "secure" : true
                }
            }).to_string()
        }

        #[test]
        fn can_raw_request_to_snap() -> Result<(), MidtransError> {
            let http_client = TestClient{};
            let order_id = generate_order_id(1);
            let parameters = generate_param_min(&order_id);
            let response = http_client.request(
                Method::POST,
                &server_key(),
                "https://app.sandbox.midtrans.com/snap/v1/transactions",
                &parameters,
                None,
                None
            )?;
            assert!(response.contains_key("token"));
            assert!(response.contains_key("redirect_url"));
            Ok(())
        }

        #[test]
        fn fail_request_401_to_snap() -> Result<(), MidtransError> {
            let http_client = TestClient{};
            let order_id = generate_order_id(1);
            let parameters = generate_param_min(&order_id);
            let response = http_client.request(
                Method::POST,
                "wrong-server-key",
                "https://app.sandbox.midtrans.com/snap/v1/transactions",
                &parameters,
                None,
                None
            );
            assert!(response.is_err());
            if let Err(MidtransError::ApiError(e)) = response {
                assert_eq!(e.status_code, 401);
            }
            Ok(())
        }

        #[test]
        fn response_not_json_exception() -> Result<(), MidtransError> {
            let http_client = TestClient{};
            let response = http_client.request(
                Method::GET,
                "",
                "https://midtrans.com/",
                "",
                None,
                None
            );
            assert!(response.is_err());
            if let Err(e) = response {
                assert_eq!(e.to_string(), "Fail to decode JSON string");
            }
            Ok(())
        }

        #[test]
        #[ignore = "To Be Implemented"]
        fn is_custom_headers_applied() -> Result<(), MidtransError> {
            todo!() // TODO request header test
        }
    }
}
