//! CoreApi

use std::collections::HashMap;
use reqwest::{self, Method, header::HeaderMap, Proxy};
use serde_json::Value;
use crate::{ApiConfig, MidtransError, Transactions, http_client::MidtransClient};

type MidtransResult = Result<HashMap<String, Value>, MidtransError>;

/// CoreApi struct used to do request to Midtrans Core API
pub struct CoreApi {
    pub api_config: ApiConfig,
}

impl MidtransClient for CoreApi {}

impl Transactions for CoreApi {
    /// Getter for ApiConfig
    fn get_api_config(&self) -> &ApiConfig {
        &self.api_config
    }

    /// Setter for ApiConfig
    fn set_api_config(&mut self, api_config: ApiConfig) {
        self.api_config = api_config
    }
}
pub struct CoreApiBuilder {
    is_production: bool,
    server_key: String,
    client_key: Option<String>,
    custom_headers: Option<HeaderMap>,
    proxies: Option<Proxy>
}

impl CoreApiBuilder {
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

    pub fn build(&self) -> Result<CoreApi, MidtransError> {
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

        Ok(CoreApi { api_config })
    }
}

impl CoreApi {
    pub fn new(is_production: bool, server_key: String) -> CoreApiBuilder {
        CoreApiBuilder {
            is_production,
            server_key,
            client_key: None,
            custom_headers: None,
            proxies: None
        }
    }

    /// Trigger `/charge` API call to Core API
    ///
    /// ### Argument
    ///
    /// `parameters` is a `&str` of Core API JSON
    /// (more params detail refer to: <https://api-docs.midtrans.com>)
    ///
    pub fn charge(&self, parameters: &str) -> MidtransResult {
        let api_url = format!(
            "{}/v2/charge",
            self.api_config.get_core_api_base_url()
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

    /// Trigger `/capture` API call to Core API
    ///
    /// ### Argument
    ///
    /// `parameters` is a `&str` of Core API JSON
    /// (more params detail refer to: <https://api-docs.midtrans.com>)
    ///
    pub fn capture(&self, parameters: &str) -> MidtransResult {
        let api_url = format!(
            "{}/v2/capture",
            self.api_config.get_core_api_base_url()
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

    /// Trigger `/card/register` API call to Core API
    ///
    /// ### Argument
    ///
    /// `parameters` is a `&str` of Core API JSON body as parameter
    /// (more params detail refer to: <https://api-docs.midtrans.com>)
    ///
    pub fn card_register(&self, parameters: &str) -> MidtransResult {
        let api_url = format!(
            "{}/v2/card/register",
            self.api_config.get_core_api_base_url()
        );

        let response = self.request(
            Method::GET,
            self.api_config.get_server_key(),
            &api_url,
            parameters,
            self.api_config.get_custom_headers().clone(),
            self.api_config.get_proxies().clone()
        )?;

        Ok(response)
    }

    /// Trigger `/token` API call to Core API
    ///
    /// ### Argument
    ///
    /// `parameters` is a `&str` of Core API JSON body as parameter
    /// (more params detail refer to: <https://api-docs.midtrans.com>)
    ///
    pub fn card_token(&self, parameters: &str) -> MidtransResult {
        let api_url = format!(
            "{}/v2/token",
            self.api_config.get_core_api_base_url()
        );

        let response = self.request(
            Method::GET,
            self.api_config.get_server_key(),
            &api_url,
            parameters,
            self.api_config.get_custom_headers().clone(),
            self.api_config.get_proxies().clone()
        )?;

        Ok(response)
    }

    /// Trigger `/point_inquiry/<token-id>` API call to Core API
    ///
    /// ### Argument
    ///
    /// `token_id` token id of credit card
    /// (more params detail refer to: <https://api-docs.midtrans.com>)
    ///
    pub fn card_point_inquiry(&self, token_id: &str) -> MidtransResult {
        let api_url = format!(
            "{}/v2/point_inquiry/{}",
            self.api_config.get_core_api_base_url(),
            token_id
        );

        let response = self.request(
            Method::GET,
            self.api_config.get_server_key(),
            &api_url,
            "",
            self.api_config.get_custom_headers().clone(),
            self.api_config.get_proxies().clone()
        )?;

        Ok(response)
    }

    /// Trigger `/v1/subscriptions` API call to Core API.
    ///
    /// Create a subscription transaction by sending all the details required to create a transaction
    ///
    /// ### Argument
    ///
    /// `parameters` is a `&str` of Core API JSON
    /// (more params detail refer to: <https://api-docs.midtrans.com/#create-subscription>)
    ///
    pub fn create_subscription(&self, parameters: &str) -> MidtransResult {
        let api_url = format!(
            "{}/v1/subscriptions",
            self.api_config.get_core_api_base_url()
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

    /// Trigger `/v1/subscriptions/<subscription_id>` API call to Core API
    ///
    /// Retrieve the subscription details of a customer using the subscription_id
    ///
    /// (more params detail refer to: <https://api-docs.midtrans.com/#get-subscription>)
    ///
    pub fn get_subscription(&self, subscription_id: &str) -> MidtransResult {
        let api_url = format!(
            "{}/v1/subscriptions/{}",
            self.api_config.get_core_api_base_url(),
            subscription_id
        );

        let response = self.request(
            Method::GET,
            self.api_config.get_server_key(),
            &api_url,
            "",
            self.api_config.get_custom_headers().clone(),
            self.api_config.get_proxies().clone()
        )?;

        Ok(response)
    }

    /// Trigger `/v1/subscriptions/<subscription_id>/disable` API call to Core API
    ///
    /// Disable the customer's subscription. The customer will not be charged in the future for this subscription
    ///
    /// (more params detail refer to: <https://api-docs.midtrans.com/#disable-subscription>)
    ///
    pub fn disable_subscription(&self, subscription_id: &str) -> MidtransResult {
        let api_url = format!(
            "{}/v1/subscriptions/{}/disable",
            self.api_config.get_core_api_base_url(),
            subscription_id
        );

        let response = self.request(
            Method::POST,
            self.api_config.get_server_key(),
            &api_url,
            "",
            self.api_config.get_custom_headers().clone(),
            self.api_config.get_proxies().clone()
        )?;

        Ok(response)
    }

    /// Trigger `/v1/subscriptions/<subscription_id>/enable` API call to Core API
    ///
    /// Enable the customer's subscription.
    ///
    /// (more params detail refer to: <https://api-docs.midtrans.com/#enable-subscription>)
    ///
    pub fn enable_subscription(&self, subscription_id: &str) -> MidtransResult {
        let api_url = format!(
            "{}/v1/subscriptions/{}/enable",
            self.api_config.get_core_api_base_url(),
            subscription_id
        );

        let response = self.request(
            Method::POST,
            self.api_config.get_server_key(),
            &api_url,
            "",
            self.api_config.get_custom_headers().clone(),
            self.api_config.get_proxies().clone()
        )?;

        Ok(response)
    }

    /// Trigger `/v1/subscriptions/<subscription_id>` API call to Core API
    ///
    /// Update existing subscription details
    ///
    /// (more params detail refer to: <https://api-docs.midtrans.com/#update-subscription>)
    ///
    pub fn update_subscription(&self, subscription_id: &str, parameters: &str) -> MidtransResult {
        let api_url = format!(
            "{}/v1/subscriptions/{}",
            self.api_config.get_core_api_base_url(),
            subscription_id
        );

        let response = self.request(
            Method::PATCH,
            self.api_config.get_server_key(),
            &api_url,
            parameters,
            self.api_config.get_custom_headers().clone(),
            self.api_config.get_proxies().clone()
        )?;

        Ok(response)
    }

    /// Trigger `/v2/pay/account` API call to Core API
    ///
    /// Link the customer account to be used for specific payment channels.
    ///
    /// ### Argument
    ///
    /// `parameters` is a `&str` of Core API JSON
    /// (more params detail refer to: <https://api-docs.midtrans.com/#create-pay-account>)
    ///
    pub fn link_payment_account(&self, parameters: &str) -> MidtransResult {
        let api_url = format!(
            "{}/v2/pay/account",
            self.api_config.get_core_api_base_url()
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

    /// Trigger `/v2/pay/account/<account-id>` API call to Core API
    ///
    /// Retrieve the payment account details of a customer using the account_id
    ///
    /// (more params detail refer to: <https://api-docs.midtrans.com/#get-pay-account>)
    ///
    pub fn get_payment_account(&self, account_id: &str) -> MidtransResult {
        let api_url = format!(
            "{}/v2/pay/account/{}",
            self.api_config.get_core_api_base_url(),
            account_id
        );

        let response = self.request(
            Method::GET,
            self.api_config.get_server_key(),
            &api_url,
            "",
            self.api_config.get_custom_headers().clone(),
            self.api_config.get_proxies().clone()
        )?;

        Ok(response)
    }

    /// Trigger `/v2/pay/account/<account-id>/unbind` API call to Core API
    ///
    /// To remove the linked customer account
    ///
    /// (more params detail refer to: <https://api-docs.midtrans.com/#unbind-pay-account>)
    ///
    pub fn unlink_payment_account(&self, account_id: &str) -> MidtransResult {
        let api_url = format!(
            "{}/v2/pay/account/{}/unbind",
            self.api_config.get_core_api_base_url(),
            account_id
        );

        let response = self.request(
            Method::POST,
            self.api_config.get_server_key(),
            &api_url,
            "",
            self.api_config.get_custom_headers().clone(),
            self.api_config.get_proxies().clone()
        )?;

        Ok(response)
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

        pub(super) fn server_key() -> String {
            env::var("MIDTRANS_SERVER_KEY").expect("SERVER_KEY NOT FOUND")
        }

        pub(super) fn client_key() -> String {
            env::var("MIDTRANS_CLIENT_KEY").expect("CLIENT_KEY NOT FOUND")
        }

        pub(super) fn generate_core_api_instance() -> CoreApi {
            CoreApi::new(false, server_key()).build().unwrap()
        }

        pub(super) fn generate_order_id(test_number: u8) -> String {
            let now = chrono::offset::Local::now().format("%Y%m%d%H%M%S").to_string();
            format!("rust-midtransclient-test{}-{}", test_number, now)
        }

        pub(super) fn generate_param_min(order_id: &str) -> String {
            json!({
                "payment_type": "bank_transfer",
                "transaction_details": {
                    "gross_amount": 44145,
                    "order_id": order_id
                },
                "bank_transfer": {
                    "bank": "bca"
                }
            }).to_string()
        }

        pub(super) fn generate_param_cc_min(order_id: &str, cc_token: &str) -> String {
            json!({
                "payment_type": "credit_card",
                "transaction_details": {
                    "gross_amount": 12145,
                    "order_id": order_id
                },
                "credit_card": {
                    "token_id": cc_token
                }
            }).to_string()
        }

        pub(super) fn generate_param_charge_min(transaction_id: &str) -> String {
            json!({
                "payment_type": "bank_transfer",
                "transaction_details": {
                    "gross_amount": 25000,
                    "order_id": transaction_id
                },
                "bank_transfer": {
                    "bank": "bca"
                }
            }).to_string()
        }

        pub(super) fn generate_param_card_token_min() -> String {
            json!({
                "card_number": "5264 2210 3887 4659",
                "card_exp_month": "12",
                "card_exp_year": "2025",
                "card_cvv": "123",
                "client_key": client_key()
            }).to_string()
        }

        pub(super) fn generate_param_card_register_min() -> String {
            json!({
                "card_number": "4811 1111 1111 1114",
                "card_exp_month": "12",
                "card_exp_year": "2030",
                "card_cvv": "123",
                "client_key": client_key()
            }).to_string()
        }

        pub(super) fn generate_param_subscription(cc_token: &str) -> String {
            let now = chrono::offset::Local::now().format("%Y%m%d%H%M%S").to_string();
            let name = format!("SUBS-RUST-{}", now);
            json!({
                "name": name,
                "amount": "100000",
                "currency": "IDR",
                "payment_type": "credit_card",
                // "token": "436502qFfqfAQKScMtPRPdZDOaeg7199",
                "token": cc_token,
                "schedule": {
                    "interval": 1,
                    "interval_unit": "day",
                    "max_interval": 7
                },
                "metadata": {
                    "description": "Recurring payment for A"
                },
                "customer_details": {
                    "first_name": "John A",
                    "last_name": "Doe A",
                    "email": "johndoe@email.com",
                    "phone": "+62812345678"
                }
            }).to_string()
        }

        pub(super) fn generate_cc_token() -> Result<String, MidtransError> {
            let core = generate_core_api_instance();
            let parameters = generate_param_card_register_min();
            let response = core.card_register(&parameters)?;
            Ok(response["saved_token_id"].to_string())
        }

        pub(super) fn generate_param_tokenization(phone_number: &str) -> String {
            json!({
                "payment_type": "gopay",
                "gopay_partner": {
                    "phone_number": phone_number,
                    "country_code": "62",
                    "redirect_url": "https://mywebstore.com/gopay-linking-finish"
                }
            }).to_string()
        }
    }

    mod core {
        use super::*;
        use super::helper::*;

        #[test]
        fn new() -> Result<(), MidtransError> {
            let core = CoreApi::new(false, "server_key".to_string()).build()?;
            assert_eq!(core.api_config.get_is_production(), false);
            assert_eq!(core.api_config.get_server_key(), "server_key");
            assert_eq!(core.api_config.get_client_key(), "");
            assert!(core.api_config.get_custom_headers().is_none());
            assert!(core.api_config.get_proxies().is_none());
            Ok(())
        }

        #[test]
        fn new_with_optionals() -> Result<(), MidtransError> {
            let is_production = false;
            let server_key = String::from("server_key");
            let client_key = String::from("client_key");
            let mut custom_headers = HeaderMap::new();
            custom_headers.insert("X-Custom-Header", "Some Value".parse().unwrap());
            let proxies = reqwest::Proxy::http("https://secure.example")?;
            let core = CoreApi::new(is_production, server_key)
                .client_key(client_key)
                .custom_headers(custom_headers.clone())
                .proxies(proxies)
                .build()?;
            assert_eq!(core.api_config.get_is_production(), false);
            assert_eq!(core.api_config.get_server_key(), "server_key");
            assert_eq!(core.api_config.get_client_key(), "client_key");
            assert_eq!(core.api_config.get_custom_headers().clone(), Some(custom_headers));
            assert!(!core.api_config.get_proxies().is_none());
            Ok(())
        }

        #[test]
        fn card_token() -> Result<(), MidtransError> {
            let core = generate_core_api_instance();
            let parameters = generate_param_card_token_min();
            let response = core.card_token(&parameters)?;
            assert_eq!(response["status_code"], "200");
            Ok(())
        }

        #[test]
        fn card_register() -> Result<(), MidtransError> {
            let core = generate_core_api_instance();
            let parameters = generate_param_card_register_min();
            let response = core.card_register(&parameters)?;
            assert_eq!(response["status_code"], "200");
            Ok(())
        }

        #[test]
        fn card_point_inquiry_valid_bni_card() -> Result<(), MidtransError> {
            let core = generate_core_api_instance();
            let parameters = generate_param_card_token_min();
            let response = core.card_token(&parameters)?;
            let token_id = response["token_id"].as_str().unwrap();
            let inquiry = core.card_point_inquiry(token_id)?;
            assert!(inquiry["status_message"].as_str().unwrap().contains("Success"));
            Ok(())
        }

        #[test]
        fn charge_cc_simple() -> Result<(), MidtransError> {
            let core = generate_core_api_instance();
            let parameters = generate_param_card_token_min();
            let response = core.card_token(&parameters)?;
            let cc_token = response["token_id"].as_str().unwrap();
            let order_id = generate_order_id(2);
            let parameters = generate_param_cc_min(&order_id, cc_token);
            let charge_cc = core.charge(&parameters)?;
            assert_eq!(charge_cc["status_code"], "200");
            assert_eq!(charge_cc["transaction_status"], "capture");
            assert_eq!(charge_cc["fraud_status"], "accept");
            Ok(())
        }

        #[test]
        fn charge_cc_one_click() -> Result<(), MidtransError> {
            let core = generate_core_api_instance();
            let parameters = generate_param_card_register_min();
            let response = core.card_register(&parameters)?;
            let saved_token_id = response["saved_token_id"].as_str().unwrap();
            let order_id = generate_order_id(3);
            let parameters = generate_param_cc_min(&order_id, saved_token_id);
            let charge_one_click = core.charge(&parameters)?;
            assert_eq!(charge_one_click["status_code"], "200");
            assert_eq!(charge_one_click["transaction_status"], "capture");
            assert_eq!(charge_one_click["fraud_status"], "accept");
            Ok(())
        }

        #[test]
        fn charge_bank_transfer_bca_simple() -> Result<(), MidtransError> {
            let core = generate_core_api_instance();
            let order_id = generate_order_id(1);
            let parameters = generate_param_min(&order_id);
            let response = core.charge(&parameters)?;
            assert_eq!(response["status_code"], "201");
            assert_eq!(response["transaction_status"], "pending");
            Ok(())
        }

        #[test]
        fn charge_fail_401() -> Result<(), MidtransError> {
            let mut core = generate_core_api_instance();
            core.api_config.set_server_key("invalid_key".to_string());
            let order_id = generate_order_id(1);
            let parameters = generate_param_min(&order_id);
            let response = core.charge(&parameters);
            assert!(response.is_err());
            if let Err(MidtransError::ApiError(e)) = response {
                assert_eq!(e.status_code, 401);
            }
            Ok(())
        }

        #[test]
        fn charge_fail_empty_param() -> Result<(), MidtransError> {
            let core = generate_core_api_instance();
            let parameters = String::from("");
            let response = core.charge(&parameters);
            assert!(response.is_err());
            if let Err(MidtransError::ApiError(e)) = response {
                assert_eq!(e.status_code, 500);
            }
            Ok(())
        }

        #[test]
        fn charge_fail_zero_gross_amount() -> Result<(), MidtransError> {
            let core = generate_core_api_instance();
            let order_id = generate_order_id(1);
            let parameters = json!({
                "payment_type": "bank_transfer",
                "transaction_details": {
                    "gross_amount": 0,
                    "order_id": order_id
                },
                "bank_transfer": {
                    "bank": "bca"
                }
            }).to_string();
            let response = core.charge(&parameters);
            assert!(response.is_err());
            if let Err(MidtransError::ApiError(e)) = response {
                assert_eq!(e.status_code, 400);
            }
            Ok(())
        }

        #[test]
        fn exception_midtrans_api_error() -> Result<(), MidtransError> {
            let core = generate_core_api_instance();
            let response = core.status("non-exist-order-id".to_string());
            assert!(response.is_err());
            if let Err(MidtransError::ApiError(e)) = response {
                println!("{}", e);
                assert!(e.message.contains("Midtrans API is returning API error."));
                assert_eq!(e.status_code, 404);
                assert_eq!(e.response["status_message"], "Transaction doesn't exist.");
            }
            Ok(())
        }

    }

    mod subscription {
        use super::*;
        use super::helper::*;

        #[test]
        fn create_subscription() -> Result<(), MidtransError> {
            let subs = generate_core_api_instance();
            let cc_token = generate_cc_token()?;
            let parameters = generate_param_subscription(&cc_token);
            let response = subs.create_subscription(&parameters)?;
            assert!(response.contains_key("id"));
            assert_eq!(response["status"], "active");
            let subscription_id = response["id"].as_str().unwrap();
            // disable subscription to prevent Core API continue to execute subscription
            subs.disable_subscription(subscription_id)?;
            Ok(())
        }

        #[test]
        fn fail_empty_param() -> Result<(), MidtransError> {
            let subs = generate_core_api_instance();
            let parameters = String::from("");
            let response = subs.create_subscription(&parameters);
            assert!(response.is_err());
            if let Err(MidtransError::ApiError(e)) = response {
               assert_eq!(e.status_code, 400);
            }
            Ok(())
        }

        #[test]
        fn fail_zero_amount() -> Result<(), MidtransError> {
            let subs = generate_core_api_instance();
            let now = chrono::offset::Local::now().format("%Y%m%d%H%M%S").to_string();
            let name = format!("SUBS-RUST-{}", now);
            let parameters = json!({
                "name": name,
                "amount": 0,
                "currency": "IDR",
                "payment_type": "credit_card",
                "token": "436502qFfqfAQKScMtPRPdZDOaeg7199",
                "schedule": {
                    "interval": 1,
                    "interval_unit": "day",
                    "max_interval": 7
                },
                "metadata": {
                    "description": "Recurring payment for A"
                },
                "customer_details": {
                    "first_name": "John A",
                    "last_name": "Doe A",
                    "email": "johndoe@email.com",
                    "phone": "+62812345678"
                }
            }).to_string();
            let response = subs.create_subscription(&parameters);
            assert!(response.is_err());
            if let Err(MidtransError::ApiError(e)) = response {
                assert_eq!(e.status_code, 400);
            }
            Ok(())
        }

        #[test]
        fn get_subscription() -> Result<(), MidtransError> {
            let subs = generate_core_api_instance();
            let cc_token = generate_cc_token()?;
            let parameters = generate_param_subscription(&cc_token);
            let response = subs.create_subscription(&parameters)?;
            let subscription_id = response["id"].as_str().unwrap();
            let response = subs.get_subscription(subscription_id)?;
            assert_eq!(response["id"], subscription_id);
            assert_eq!(response["status"], "active");
            // disable subscription to prevent Core API continue to execute subscription
            subs.disable_subscription(subscription_id)?;
            Ok(())
        }

        #[test]
        fn get_subscription_not_fount() -> Result<(), MidtransError> {
            let subs = generate_core_api_instance();
            let response = subs.get_subscription("123");
            assert!(response.is_err());
            if let Err(MidtransError::ApiError(e)) = response {
                assert_eq!(e.status_code, 404);
            }
            Ok(())
        }

        #[test]
        fn disable_subscription() -> Result<(), MidtransError> {
            let subs = generate_core_api_instance();
            let cc_token = generate_cc_token()?;
            let parameters = generate_param_subscription(&cc_token);
            let response = subs.create_subscription(&parameters)?;
            let subscription_id = response["id"].as_str().unwrap();
            let response = subs.disable_subscription(subscription_id)?;
            assert_eq!(response["status_message"], "Subscription is updated.");
            let response = subs.get_subscription(subscription_id)?;
            assert_eq!(response["id"], subscription_id);
            assert_eq!(response["status"], "inactive");
            Ok(())
        }

        #[test]
        fn enable_subscription() -> Result<(), MidtransError> {
            let subs = generate_core_api_instance();
            let cc_token = generate_cc_token()?;
            let parameters = generate_param_subscription(&cc_token);
            let response = subs.create_subscription(&parameters)?;
            let subscription_id = response["id"].as_str().unwrap();
            let _ = subs.disable_subscription(subscription_id)?;
            let response = subs.enable_subscription(subscription_id)?;
            assert_eq!(response["status_message"], "Subscription is updated.");
            let response = subs.get_subscription(subscription_id)?;
            assert_eq!(response["id"], subscription_id);
            assert_eq!(response["status"], "active");
            // disable subscription to prevent Core API continue to execute subscription
            subs.disable_subscription(subscription_id)?;
            Ok(())
        }

        #[test]
        fn update_subscription() -> Result<(), MidtransError> {
            let subs = generate_core_api_instance();
            let cc_token = generate_cc_token()?;
            let parameters = generate_param_subscription(&cc_token);
            let response = subs.create_subscription(&parameters)?;
            let subscription_id = response["id"].as_str().unwrap();

            let now = chrono::offset::Local::now().format("%Y%m%d%H%M%S").to_string();
            let name = format!("SUBS-RUST-{}", now);
            let parameters = json!({
                "name": name,
                "amount": "100000",
                "currency": "IDR",
                "payment_type": "credit_card",
                "token": cc_token,
                "schedule": {
                    "interval": 1,
                    "interval_unit": "day",
                    "max_interval": 7
                },
                "metadata": {
                    "description": "update recurring payment for ABC"
                },
                "customer_details": {
                    "first_name": "John A",
                    "last_name": "Doe A",
                    "email": "johndoe@email.com",
                    "phone": "+62812345678"
                }
            }).to_string();

            let response = subs.update_subscription(subscription_id, &parameters)?;
            assert_eq!(response["status_message"], "Subscription is updated.");
            let response = subs.get_subscription(subscription_id)?;
            assert_eq!(response["id"], subscription_id);
            assert_eq!(response["metadata"]["description"], "update recurring payment for ABC");
            // disable subscription to prevent Core API continue to execute subscription
            subs.disable_subscription(subscription_id)?;
            Ok(())
        }
    }

    mod tokenization {
        use super::*;
        use super::helper::*;

        const PHONEUNREGISTERED: &'static str = "123450001";
        const PHONEBLOCKED: &'static str = "123450002";

        #[test]
        fn link_account() -> Result<(), MidtransError> {
            let tokenize = generate_core_api_instance();
            let parameters = generate_param_tokenization("81234567891");
            let response = tokenize.link_payment_account(&parameters)?;
            assert!(response.contains_key("account_id"));
            assert_eq!(response["status_code"], "201");
            assert_eq!(response["account_status"], "PENDING");
            Ok(())
        }

        #[test]
        fn link_account_user_not_found() -> Result<(), MidtransError> {
            let tokenize = generate_core_api_instance();
            let parameters = generate_param_tokenization(PHONEUNREGISTERED);
            let response = tokenize.link_payment_account(&parameters)?;
            assert_eq!(response["status_code"], "202");
            assert_eq!(response["channel_response_message"], "User Not Found");
            Ok(())
        }

        #[test]
        fn link_account_user_blocked() -> Result<(), MidtransError> {
            let tokenize = generate_core_api_instance();
            let parameters = generate_param_tokenization(PHONEBLOCKED);
            let response = tokenize.link_payment_account(&parameters)?;
            assert_eq!(response["status_code"], "202");
            assert_eq!(response["channel_response_message"], "Wallet is Blocked");
            Ok(())
        }

        #[test]
        fn link_account_phone_start_with_0() -> Result<(), MidtransError> {
            let tokenize = generate_core_api_instance();
            let parameters = generate_param_tokenization("081234567891");
            let response = tokenize.link_payment_account(&parameters);
            assert!(response.is_err());
            if let Err(MidtransError::ApiError(e)) = response {
                assert_eq!(e.status_code, 400);
            }
            Ok(())
        }

        #[test]
        fn get_account() -> Result<(), MidtransError> {
            let tokenize = generate_core_api_instance();
            let parameters = generate_param_tokenization("81234567891");
            let response = tokenize.link_payment_account(&parameters)?;
            let account_id = response["account_id"].as_str().unwrap();
            let response = tokenize.get_payment_account(account_id)?;
            assert_eq!(response["status_code"], "201");
            assert_eq!(response["account_id"], account_id);
            Ok(())
        }

        #[test]
        fn unlink_account() -> Result<(), MidtransError> {
            let tokenize = generate_core_api_instance();
            let parameters = generate_param_tokenization("81234567891");
            let response = tokenize.link_payment_account(&parameters)?;
            let account_id = response["account_id"].as_str().unwrap();
            let response = tokenize.unlink_payment_account(account_id);
            assert!(response.is_err());
            if let Err(MidtransError::ApiError(e)) = response {
                assert_eq!(e.status_code, 412);
            }
            Ok(())
        }
    }

    mod transaction {
        use super::*;
        use super::helper::*;

        #[test]
        fn status() -> Result<(), MidtransError> {
            let core = generate_core_api_instance();
            let transaction_id = generate_order_id(1);
            let parameters = generate_param_charge_min(&transaction_id);
            let _ = core.charge(&parameters)?;
            let response = core.status(transaction_id)?;
            assert_eq!(response["status_code"], "201");
            assert_eq!(response["transaction_status"], "pending");
            Ok(())
        }

        #[test]
        #[ignore = "To Be Implemented"]
        fn statusb2b() -> Result<(), MidtransError> {
            todo!() // TODO transaction statusb2b test
        }

        #[test]
        fn notification_from_json() -> Result<(), MidtransError> {
            let core = generate_core_api_instance();
            let transaction_id = generate_order_id(1);
            let parameters = generate_param_charge_min(&transaction_id);
            let _ = core.charge(&parameters)?;
            let response = core.status(transaction_id)?;
            let notification_json = core.notification_from_json(response)?;
            assert_eq!(notification_json["status_code"], "201");
            assert_eq!(notification_json["transaction_status"], "pending");
            Ok(())
        }

        #[test]
        fn notification_from_str() -> Result<(), MidtransError> {
            let core = generate_core_api_instance();
            let transaction_id = generate_order_id(1);
            let parameters = generate_param_charge_min(&transaction_id);
            let _ = core.charge(&parameters)?;
            let response = core.status(transaction_id)?;
            let notification = json!({
                "transaction_id": response["transaction_id"].as_str().unwrap()
            }).to_string();
            let notification_json = core.notification_from_str(&notification)?;
            assert_eq!(notification_json["status_code"], "201");
            assert_eq!(notification_json["transaction_status"], "pending");
            Ok(())
        }

        #[test]
        fn notification_from_json_fail() -> Result<(), MidtransError> {
            let core = generate_core_api_instance();
            let notification = String::new();
            assert!(core.notification_from_str(&notification).is_err());
            Ok(())
        }

        #[test]
        fn expire() -> Result<(), MidtransError> {
            let core = generate_core_api_instance();
            let transaction_id = generate_order_id(1);
            let parameters = generate_param_charge_min(&transaction_id);
            let _ = core.charge(&parameters)?;
            let response = core.expire(transaction_id);
            assert!(response.is_err());
            if let Err(MidtransError::ApiError(e)) = response {
                assert_eq!(e.status_code, 407);
                assert_eq!(e.response["transaction_status"], "expire");
            }
            Ok(())
        }

        #[test]
        fn approve_fail_cannot_be_updated() -> Result<(), MidtransError> {
            let core = generate_core_api_instance();
            let parameters = generate_param_card_token_min();
            let response = core.card_token(&parameters)?;
            let token_id = response["token_id"].as_str().unwrap();
            let order_id = generate_order_id(2);
            let parameters = generate_param_cc_min(&order_id, token_id);
            let _charge_cc = core.charge(&parameters)?;
            let approve_response = core.approve(order_id);
            assert!(approve_response.is_err());
            if let Err(MidtransError::ApiError(e)) = approve_response {
                assert_eq!(e.status_code, 412)
            }
            Ok(())
        }

        #[test]
        fn deny_fail_cannot_be_updated() -> Result<(), MidtransError> {
            let core = generate_core_api_instance();
            let parameters = generate_param_card_token_min();
            let response = core.card_token(&parameters)?;
            let token_id = response["token_id"].as_str().unwrap();
            let order_id = generate_order_id(2);
            let parameters = generate_param_cc_min(&order_id, token_id);
            let _charge_cc = core.charge(&parameters)?;
            let deny_response = core.deny(order_id);
            assert!(deny_response.is_err());
            if let Err(MidtransError::ApiError(e)) = deny_response {
                assert_eq!(e.status_code, 412)
            }
            Ok(())
        }

        #[test]
        fn cancel() -> Result<(), MidtransError> {
            let core = generate_core_api_instance();
            let parameters = generate_param_card_token_min();
            let response = core.card_token(&parameters)?;
            let token_id = response["token_id"].as_str().unwrap();
            let order_id = generate_order_id(2);
            let parameters = generate_param_cc_min(&order_id, token_id);
            let _charge_cc = core.charge(&parameters)?;
            let cancel_response = core.cancel(order_id)?;
            assert_eq!(cancel_response["status_code"], "200");
            assert_eq!(cancel_response["transaction_status"], "cancel");
            Ok(())
        }

        #[test]
        fn refund_fail_not_yet_settlement() -> Result<(), MidtransError> {
            let core = generate_core_api_instance();
            let parameters = generate_param_card_register_min();
            let response = core.card_register(&parameters)?;
            let saved_token_id = response["saved_token_id"].as_str().unwrap();
            let transaction_id = generate_order_id(3);
            let parameters = generate_param_cc_min(&transaction_id, saved_token_id);
            let _charge_one_click = core.charge(&parameters)?;
            let parameters = json!({
                "refund_key": "order1-ref1",
                "amount": 5000,
                "reason": "for some reason"
            }).to_string();
            let refund_response = core.refund(transaction_id, &parameters);
            assert!(refund_response.is_err());
            if let Err(MidtransError::ApiError(e)) = refund_response {
                assert_eq!(e.status_code, 412);
            }
            Ok(())
        }

        #[test]
        fn direct_refund_not_yet_settlement() -> Result<(), MidtransError> {
            let core = generate_core_api_instance();
            let parameters = generate_param_card_register_min();
            let response = core.card_register(&parameters)?;
            let saved_token_id = response["saved_token_id"].as_str().unwrap();
            let transaction_id = generate_order_id(3);
            let parameters = generate_param_cc_min(&transaction_id, saved_token_id);
            let _charge_one_click = core.charge(&parameters)?;
            let parameters = json!({
                "refund_key": "order1-ref1",
                "amount": 5000,
                "reason": "for some reason"
            }).to_string();
            let refund_response = core.refund_direct(transaction_id, &parameters);
            assert!(refund_response.is_err());
            if let Err(MidtransError::ApiError(e)) = refund_response {
                assert_eq!(e.status_code, 412);
            }
            Ok(())
        }

        #[test]
        fn status_fail_404() -> Result<(), MidtransError> {
            let core = generate_core_api_instance();
            let transaction_id = String::from("non-exist-order-id");
            let response = core.status(transaction_id);
            assert!(response.is_err());
            if let Err(MidtransError::ApiError(e)) = response {
                assert_eq!(e.status_code, 404);
            }
            Ok(())
        }

        #[test]
        fn status_server_key_change_via_property() -> Result<(), MidtransError> {
            let mut core = CoreApi::new(false, "server_key".to_string()).build()?;
            core.api_config.server_key = server_key();
            let transaction_id = generate_order_id(1);
            let parameters = generate_param_charge_min(&transaction_id);
            let _ = core.charge(&parameters)?;
            let response = core.status(transaction_id)?;
            assert_eq!(response["status_code"], "201");
            assert_eq!(response["transaction_status"], "pending");
            Ok(())
        }

        #[test]
        fn status_server_key_change_via_setter() -> Result<(), MidtransError> {
            let mut core = CoreApi::new(false, server_key()).build()?;
            assert_eq!(core.api_config.is_production, false);
            assert_eq!(core.api_config.server_key, server_key());
            let response = core.status("non-exist-order-id".to_string());
            assert!(response.is_err());
            if let Err(MidtransError::ApiError(e)) = response {
                assert_eq!(e.status_code, 404);
            }

            core.api_config.set_is_production(true);
            core.api_config.set_server_key("abc".to_string());

            let transaction_id = generate_order_id(1);
            let parameters = generate_param_charge_min(&transaction_id);
            let _response = core.charge(&parameters);
            let response_status = core.status(transaction_id.clone());
            assert!(response_status.is_err());
            if let Err(MidtransError::ApiError(err)) = response_status {
                assert_eq!(err.status_code, 401);
            }

            let new_config = ApiConfig::new(false, server_key())
                .client_key(client_key()).build();
            core.set_api_config(new_config);
            let _response = core.charge(&parameters);
            let response_status = core.status(transaction_id)?;
            assert_eq!(response_status["status_code"], "201");
            assert_eq!(response_status["transaction_status"], "pending");

            Ok(())
        }
    }
}