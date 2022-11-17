//! Api Config

use std::fmt::Display;
use reqwest::header;

const CORE_SANDBOX_BASE_URL: &'static str = "https://api.sandbox.midtrans.com";
const CORE_PRODUCTION_BASE_URL:  &'static str = "https://api.midtrans.com";
const SNAP_SANDBOX_BASE_URL: &'static str = "https://app.sandbox.midtrans.com";
const SNAP_PRODUCTION_BASE_URL: &'static str = "https://app.midtrans.com";

/// Config Object that used to store is_production, server_key, client_key.
/// And also API base urls.
///
/// note: client_key is not necessarily required for API call.
pub struct ApiConfig {
    pub is_production: bool,
    pub server_key: String,
    pub client_key: String,
    pub custom_headers: Option<header::HeaderMap>,
    pub proxies: Option<reqwest::Proxy>
}

impl Display for ApiConfig {
    fn fmt(&self, f: &mut std::fmt::Formatter) -> std::fmt::Result {
        write!(f, "<ApiConfig({},{},{},{:?},{:?})>",
            self.is_production,
            self.server_key,
            self.client_key,
            self.custom_headers,
            self.proxies
        )?;
        Ok(())
    }
}

impl ApiConfig {
    pub fn new(is_production: bool, server_key: String) -> ApiConfigBuilder {
        ApiConfigBuilder {
            is_production,
            server_key,
            client_key: None,
            custom_header: None,
            proxies: None
        }
    }

    pub fn get_core_api_base_url(&self) -> &'static str {
        match self.is_production {
            true => CORE_PRODUCTION_BASE_URL,
            false => CORE_SANDBOX_BASE_URL
        }
    }

    pub fn get_snap_base_url(&self) -> &'static str {
        match self.is_production {
            true => SNAP_PRODUCTION_BASE_URL,
            false => SNAP_SANDBOX_BASE_URL
        }
    }

    pub fn get_is_production(&self) -> bool {
        self.is_production
    }

    pub fn set_is_production(&mut self, value: bool) {
        self.is_production = value;
    }

    pub fn get_server_key(&self) -> &str {
        &self.server_key
    }

    pub fn set_server_key(&mut self, value: String) {
        self.server_key = value;
    }

    pub fn get_client_key(&self) -> &str {
        &self.client_key
    }

    pub fn set_client_key(&mut self, value: String) {
        self.client_key = value;
    }

    pub fn get_custom_headers(&self) -> &Option<header::HeaderMap> {
        &self.custom_headers
    }

    pub fn set_custom_headers(&mut self, headers: header::HeaderMap) {
        self.custom_headers = Some(headers);
    }

    pub fn get_proxies(&self) -> &Option<reqwest::Proxy> {
        &self.proxies
    }

    pub fn set_proxies(&mut self, proxies: reqwest::Proxy) {
        self.proxies = Some(proxies);
    }
}

pub struct ApiConfigBuilder {
    is_production: bool,
    server_key: String,
    client_key: Option<String>,
    custom_header: Option<header::HeaderMap>,
    proxies: Option<reqwest::Proxy>
}

impl ApiConfigBuilder {
    pub fn client_key(&mut self, client_key: String) -> &mut Self {
        self.client_key = Some(client_key);
        self
    }

    pub fn custom_header(&mut self, custom_header: header::HeaderMap) -> &mut Self {
        self.custom_header = Some(custom_header);
        self
    }

    pub fn proxies(&mut self, proxies: reqwest::Proxy) -> &mut Self {
        self.proxies = Some(proxies);
        self
    }

    pub fn build(&mut self) -> ApiConfig {
        ApiConfig {
            is_production: self.is_production,
            server_key: self.server_key.clone(),
            client_key: self.client_key.clone().unwrap_or(String::new()),
            custom_headers: self.custom_header.clone(),
            proxies: self.proxies.clone()
        }
    }
}

#[cfg(test)]
mod test {
    use super::*;

    #[test]
    fn build() {
        let server_key = String::from("secret_key");
        let config = ApiConfig::new(false, server_key).build();
        assert_eq!(config.is_production, false);
        assert_eq!(config.server_key, "secret_key");
        assert_eq!(config.client_key, "");
        assert!(config.custom_headers.is_none());
        assert!(config.proxies.is_none());
    }

    #[test]
    fn display() {
        let server_key = String::from("secret_key");
        let config = ApiConfig::new(false, server_key).build();
        assert_eq!(config.to_string(), "<ApiConfig(false,secret_key,,None,None)>");
    }

    #[test]
    fn get_core_api_base_url() {
        let server_key = String::from("secret_key");
        let mut config = ApiConfig::new(false, server_key).build();
        assert_eq!(config.get_core_api_base_url(), CORE_SANDBOX_BASE_URL);
        config.set_is_production(true);
        assert_eq!(config.get_core_api_base_url(), CORE_PRODUCTION_BASE_URL);
    }

    #[test]
    fn get_snap_base_url() {
        let server_key = String::from("secret_key");
        let mut config = ApiConfig::new(false, server_key).build();
        assert_eq!(config.get_snap_base_url(), SNAP_SANDBOX_BASE_URL);
        config.set_is_production(true);
        assert_eq!(config.get_snap_base_url(), SNAP_PRODUCTION_BASE_URL);
    }

    #[test]
    fn get_is_production() {
        let server_key = String::from("secret_key");
        let config = ApiConfig::new(false, server_key).build();
        assert_eq!(config.get_is_production(), false);
    }

    #[test]
    fn set_is_production() {
        let server_key = String::from("secret_key");
        let mut config = ApiConfig::new(false, server_key).build();
        config.set_is_production(true);
        assert_eq!(config.get_is_production(), true);
    }

    #[test]
    fn get_server_key() {
        let server_key = String::from("secret_key");
        let config = ApiConfig::new(false, server_key).build();
        assert_eq!(config.get_server_key(), "secret_key");
    }

    #[test]
    fn set_server_key() {
        let server_key = String::from("secret_key");
        let mut config = ApiConfig::new(false, server_key).build();
        config.set_server_key("key_secret".to_string());
        assert_eq!(config.get_server_key(), "key_secret");
    }

    #[test]
    fn get_client_key() {
        let server_key = String::from("secret_key");
        let config = ApiConfig::new(false, server_key).build();
        assert_eq!(config.get_client_key(), "");
    }

    #[test]
    fn set_client_key() {
        let server_key = String::from("secret_key");
        let mut config = ApiConfig::new(false, server_key).build();
        config.set_client_key("client_key".to_string());
        assert_eq!(config.get_client_key(), "client_key");
    }

    #[test]
    fn get_custom_headers() {
        let server_key = String::from("secret_key");
        let config = ApiConfig::new(false, server_key).build();
        assert!(config.get_custom_headers().is_none());
    }

    #[test]
    fn set_custom_headers() {
        let server_key = String::from("secret_key");
        let mut config = ApiConfig::new(false, server_key).build();
        let mut headers = header::HeaderMap::new();
        headers.insert("X-Custom-Header", header::HeaderValue::from_static("Some Value"));
        config.set_custom_headers(headers.clone());
        assert_eq!(config.get_custom_headers().clone().unwrap(), headers);
    }

    #[test]
    fn get_proxies() {
        let server_key = String::from("secret_key");
        let config = ApiConfig::new(false, server_key).build();
        assert!(config.get_proxies().is_none());
    }

    #[test]
    fn set_proxies() {
        let server_key = String::from("secret_key");
        let mut config = ApiConfig::new(false, server_key).build();
        let proxies = reqwest::Proxy::http("https://secure.example").unwrap();
        config.set_proxies(proxies.clone());
    }
}
