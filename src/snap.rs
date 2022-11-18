//! Snap

use std::collections::HashMap;
use reqwest::{Method, header::HeaderMap, Proxy};
use serde_json::Value;
use crate::{ApiConfig, MidtransError, http_client::MidtransClient, Transactions};

type MidtransResult = Result<HashMap<String, Value>, MidtransError>;

/// Snap struct used to do request to Midtrans Snap API
pub struct Snap {
    pub api_config: ApiConfig,
}

impl MidtransClient for Snap {}

impl Transactions for Snap {
    /// Getter for ApiConfig
    fn get_api_config(&self) -> &ApiConfig {
        &self.api_config
    }

    /// Setter for ApiConfig
    fn set_api_config(&mut self, api_config: ApiConfig) {
        self.api_config = api_config
    }
}

pub struct SnapBuilder {
    is_production: bool,
    server_key: String,
    client_key: Option<String>,
    custom_headers: Option<HeaderMap>,
    proxies: Option<Proxy>
}

impl SnapBuilder {
    pub fn client_key(&mut self, client_key: String) -> &mut Self {
        self.client_key = Some(client_key);
        self
    }

    pub fn custom_headers(&mut self, custom_headers: HeaderMap) -> &mut Self {
        self.custom_headers = Some(custom_headers);
        self
    }

    pub fn proxies(&mut self, proxies: Proxy) -> &mut Self {
        self.proxies = Some(proxies);
        self
    }

    pub fn build(&self) -> Result<Snap, MidtransError> {
        let mut api_config = ApiConfig::new(self.is_production, self.server_key.clone());

        if let Some(key) = &self.client_key {
            api_config.client_key(key.clone());
        }

        if let Some(headers) = &self.custom_headers {
            api_config.custom_header(headers.clone());
        }

        if let Some(proxy) = &self.proxies {
            api_config.proxies(proxy.clone());
        }

        let api_config = api_config.build();

        Ok(Snap { api_config })
    }
}

impl Snap {
    pub fn new(is_production: bool, server_key: String) -> SnapBuilder {
        SnapBuilder {
            is_production,
            server_key,
            client_key: None,
            custom_headers: None,
            proxies: None
        }
    }

    /// Trigger API call to Snap API
    ///
    /// ### Argument
    ///
    /// `parameters` is a `&str` of Core API JSON
    /// (more params detail refer to: <https://snap-docs.midtrans.com>)
    ///
    /// ### Return
    ///
    /// HashMap from JSON decoded response, that contains `token` and `redirect_url`
    ///
    pub fn create_transaction(&self, parameters: &str) -> MidtransResult {
        let api_url = format!(
            "{}/snap/v1/transactions",
            self.api_config.get_snap_base_url()
        );

        let response = self.request(
            Method::POST,
            self.api_config.get_server_key(),
            &api_url,
            parameters,
            self.api_config.get_custom_headers().clone(),
            self.api_config.get_proxies().clone()
        )?;

        Ok(response)
    }

    /// Wrapper method that call `create_transaction` and directly return `token`
    pub fn create_transaction_token(&self, parameters: &str) -> Result<Value, MidtransError> {
        let response = self.create_transaction(parameters)?;
        Ok(response["token"].clone())
    }

    /// Wrapper method that call `create_transaction` and directly return `redirect_url`
    pub fn create_transaction_redirect_url(&self, parameters: &str) -> Result<Value, MidtransError> {
        let response = self.create_transaction(parameters)?;
        Ok(response["redirect_url"].clone())
    }

}

#[cfg(test)]
mod test {
    // When you run multiple tests, by default they run in parallel using threads,
    // meaning they finish running faster and you get feedback quicker.
    // Because the tests are running at the same time, you must make sure
    // your tests don’t depend on each other or on any shared state,
    // including a shared environment, such as the current working directory
    // or environment variables.
    //
    // If you don’t want to run the tests in parallel or if you want more
    // fine-grained control over the number of threads used, you can send
    // the --test-threads flag and the number of threads you want to use
    // to the test binary. Take a look at the following example:
    //
    // $ cargo test -- --test-threads=1
    //
    // source: https://doc.rust-lang.org/book/ch11-02-running-tests.html

    use super::*;
    use serde_json::json;

    mod helper {
        use super::*;
        use std::env;
        use chrono;

        pub(crate) fn server_key() -> String {
            env::var("MIDTRANS_SERVER_KEY").expect("SERVER_KEY NOT FOUND")
        }

        pub(crate) fn client_key() -> String {
            env::var("MIDTRANS_CLIENT_KEY").expect("CLIENT_KEY NOT FOUND")
        }

        pub(crate) fn generate_snap_api_instance() -> Snap {
            Snap::new(false, server_key())
                .client_key(client_key())
                .build()
                .unwrap()
        }

        pub(crate) fn generate_order_id(test_number: u8) -> String {
            let now = chrono::offset::Local::now().format("%Y%m%d%H%M%S").to_string();
            format!("rust-midtransclient-test{}-{}", test_number, now)
        }

        pub(crate) fn generate_param_min(order_id: &str) -> String {
            json!({
                "transaction_details": {
                    "order_id": order_id,
                    "gross_amount": 200000
                }, "credit_card":{
                    "secure" : true
                }
            }).to_string()
        }

        pub(crate) fn generate_param_max(order_id: &str) -> String {
            json!({
                "transaction_details": {
                    "order_id": order_id,
                    "gross_amount": 10000
                },
                "item_details": [{
                    "id": "ITEM1",
                    "price": 10000,
                    "quantity": 1,
                    "name": "Midtrans Bear",
                    "brand": "Midtrans",
                    "category": "Toys",
                    "merchant_name": "Midtrans"
                }],
                "customer_details": {
                    "first_name": "John",
                    "last_name": "Watson",
                    "email": "test@example.com",
                    "phone": "+628123456",
                    "billing_address": {
                        "first_name": "John",
                        "last_name": "Watson",
                        "email": "test@example.com",
                        "phone": "081 2233 44-55",
                        "address": "Sudirman",
                        "city": "Jakarta",
                        "postal_code": "12190",
                        "country_code": "IDN"
                    },
                    "shipping_address": {
                        "first_name": "John",
                        "last_name": "Watson",
                        "email": "test@example.com",
                        "phone": "0 8128-75 7-9338",
                        "address": "Sudirman",
                        "city": "Jakarta",
                        "postal_code": "12190",
                        "country_code": "IDN"
                    }
                },
                "enabled_payments": ["credit_card", "mandiri_clickpay", "cimb_clicks","bca_klikbca", "bca_klikpay", "bri_epay", "echannel", "indosat_dompetku","mandiri_ecash", "permata_va", "bca_va", "bni_va", "other_va", "gopay","kioson", "indomaret", "gci", "danamon_online"],
                "credit_card": {
                    "secure": true,
                    "channel": "migs",
                    "bank": "bca",
                    "installment": {
                        "required": false,
                        "terms": {
                            "bni": [3, 6, 12],
                            "mandiri": [3, 6, 12],
                            "cimb": [3],
                            "bca": [3, 6, 12],
                            "offline": [6, 12]
                        }
                    },
                    "whitelist_bins": [
                        "48111111",
                        "41111111"
                    ]
                },
                "bca_va": {
                    "va_number": "12345678911",
                    "free_text": {
                        "inquiry": [
                            {
                                "en": "text in English",
                                "id": "text in Bahasa Indonesia"
                            }
                        ],
                        "payment": [
                            {
                                "en": "text in English",
                                "id": "text in Bahasa Indonesia"
                            }
                        ]
                    }
                },
                "bni_va": {
                    "va_number": "12345678"
                },
                "permata_va": {
                    "va_number": "1234567890",
                    "recipient_name": "SUDARSONO"
                },
                "callbacks": {
                    "finish": "https://demo.midtrans.com"
                },
                "expiry": {
                    "start_time": "2030-12-20 18:11:08 +0700",
                    "unit": "minutes",
                    "duration": 1
                },
                "custom_field1": "custom field 1 content",
                "custom_field2": "custom field 2 content",
                "custom_field3": "custom field 3 content"
            }).to_string()
        }
    }

    mod snap {
        use super::*;
        use super::helper::*;

        #[test]
        fn new() {
            let snap = Snap::new(false, "server_key".to_string()).build().unwrap();
            assert_eq!(snap.api_config.get_is_production(), false);
            assert_eq!(snap.api_config.get_server_key(), "server_key");
            assert_eq!(snap.api_config.get_client_key(), "");
            assert!(snap.api_config.get_custom_headers().is_none());
            assert!(snap.api_config.get_proxies().is_none());
        }

        #[test]
        fn new_with_optionals() {
            let is_production = false;
            let server_key = String::from("server_key");
            let client_key = String::from("client_key");
            let mut custom_headers = HeaderMap::new();
            let proxies = reqwest::Proxy::http("https://secure.example").unwrap();
            custom_headers.insert("X-Custom-Header", "Some Value".parse().unwrap());
            let snap = Snap::new(is_production, server_key)
                .client_key(client_key)
                .custom_headers(custom_headers.clone())
                .proxies(proxies)
                .build()
                .unwrap();
            assert_eq!(snap.api_config.get_is_production(), false);
            assert_eq!(snap.api_config.get_server_key(), "server_key");
            assert_eq!(snap.api_config.get_client_key(), "client_key");
            assert_eq!(snap.api_config.get_custom_headers().clone().unwrap(), custom_headers);
            assert!(!snap.api_config.get_proxies().is_none());
        }

        #[test]
        fn create_transaction_min() -> Result<(), MidtransError> {
            let snap = generate_snap_api_instance();
            let order_id = generate_order_id(1);
            let parameters = generate_param_min(&order_id);
            let transaction = snap.create_transaction(&parameters)?;
            assert!(transaction.contains_key("token"));
            assert!(transaction.contains_key("redirect_url"));
            Ok(())
        }

        #[test]
        fn create_transaction_max() -> Result<(), MidtransError> {
            let snap = generate_snap_api_instance();
            let order_id = generate_order_id(1);
            let parameters = generate_param_max(&order_id);
            let transaction = snap.create_transaction(&parameters)?;
            assert!(transaction.contains_key("token"));
            assert!(transaction.contains_key("redirect_url"));
            Ok(())
        }

        #[test]
        fn create_transaction_token() -> Result<(), MidtransError> {
            let snap = generate_snap_api_instance();
            let order_id = generate_order_id(1);
            let parameters = generate_param_min(&order_id);
            let token = snap.create_transaction_token(&parameters)?;
            assert!(token.to_string().len() > 0);
            Ok(())
        }

        #[test]
        fn create_transaction_redirect_url() -> Result<(), MidtransError> {
            let snap = generate_snap_api_instance();
            let order_id = generate_order_id(1);
            let parameters = generate_param_min(&order_id);
            let redirect_url = snap.create_transaction_redirect_url(&parameters)?;
            assert!(redirect_url.to_string().len() > 0);
            Ok(())
        }

        #[test]
        fn status_fail_404() -> Result<(), MidtransError> {
            let snap = generate_snap_api_instance();
            let response = snap.status("non-exist-order-id".to_string());
            assert!(response.is_err());
            if let Err(MidtransError::ApiError(e)) = response {
                assert_eq!(e.status_code, 404);
            }
            Ok(())
        }

        #[test]
        fn status_fail_401() -> Result<(), MidtransError> {
            let mut snap = generate_snap_api_instance();
            snap.api_config.set_server_key("dummy".to_string());
            let order_id = generate_order_id(1);
            let parameters = generate_param_min(&order_id);
            let transaction = snap.create_transaction(&parameters);
            assert!(transaction.is_err());
            if let Err(MidtransError::ApiError(e)) = transaction {
                assert_eq!(e.status_code, 401);
                assert!(e.message.contains("unauthorized"));
            }
            Ok(())
        }

        #[test]
        fn charge_fail_empty_param() -> Result<(), MidtransError> {
            let snap = generate_snap_api_instance();
            let parameters = String::from("");
            let response = snap.create_transaction(&parameters);
            assert!(response.is_err());
            if let Err(MidtransError::ApiError(e)) = response {
                assert_eq!(e.status_code, 400);
            }
            Ok(())
        }

        #[test]
        fn charge_fail_zero_gross_amount() -> Result<(), MidtransError> {
            let snap = generate_snap_api_instance();
            let order_id = generate_order_id(1);
            let parameters = json!({
                "transaction_details": {
                    "order_id": order_id,
                    "gross_amount": 0
                }, "credit_card":{
                    "secure" : true
                }
            }).to_string();
            let response = snap.create_transaction(&parameters);
            assert!(response.is_err());
            if let Err(MidtransError::ApiError(e)) = response {
                assert_eq!(e.status_code, 400);
            }
            Ok(())
        }

        #[test]
        fn exception_midtrans_api_error() -> Result<(), MidtransError> {
            let mut snap = generate_snap_api_instance();
            snap.api_config.set_server_key("dummy".to_string());
            let order_id = generate_order_id(1);
            let parameters = generate_param_min(&order_id);
            let transaction = snap.create_transaction(&parameters);
            assert!(transaction.is_err());
            if let Err(MidtransError::ApiError(e)) = transaction {
                assert!(e.message.contains("Midtrans API is returning API error."));
                assert_eq!(e.status_code, 401);
                assert_eq!(e.response["status_code"], "401");
            }
            Ok(())
        }

        #[test]
        fn create_transaction_min_with_custom_headers_via_setter() -> Result<(), MidtransError> {
            let mut snap = generate_snap_api_instance();
            let mut headers = HeaderMap::new();
            headers.insert("X-Override-Notification", "https://example.org".parse().unwrap());
            snap.api_config.set_custom_headers(headers);
            let order_id = generate_order_id(1);
            let parameters = generate_param_min(&order_id);
            let transaction = snap.create_transaction(&parameters)?;
            assert!(transaction.contains_key("token"));
            assert!(transaction.contains_key("redirect_url"));
            Ok(())
        }
    }
}