//! Transactions

use std::collections::HashMap;
use reqwest::Method;
use serde_json::Value;
use crate::{MidtransError, ApiConfig, http_client::MidtransClient};

type MidtransResult = Result<HashMap<String, Value>, MidtransError>;

pub trait Transactions: MidtransClient {
    fn get_api_config(&self) -> &ApiConfig;

    fn set_api_config(&mut self, api_config: ApiConfig);

    fn status(&self, transaction_id: String) -> MidtransResult {
        let api_url = format!(
            "{}/v2/{}/status",
            self.get_api_config().get_core_api_base_url(),
            transaction_id
        );

        let response = self.request(
            Method::GET,
            self.get_api_config().get_server_key(),
            &api_url,
            "",
            self.get_api_config().get_custom_headers().clone(),
            self.get_api_config().get_proxies().clone()
        )?;

        Ok(response)
    }

    fn statusb2b(&self, transaction_id: String) -> MidtransResult {
        let api_url = format!(
            "{}/v2/{}/status/b2b",
            self.get_api_config().get_core_api_base_url(),
            transaction_id
        );

        let response = self.request(
            Method::GET,
            self.get_api_config().get_server_key(),
            &api_url,
            "",
            self.get_api_config().get_custom_headers().clone(),
            self.get_api_config().get_proxies().clone()
        )?;

        Ok(response)
    }

    fn approve(&self, transaction_id: String) -> MidtransResult {
        let api_url = format!(
            "{}/v2/{}/approve",
            self.get_api_config().get_core_api_base_url(),
            transaction_id
        );

        let response = self.request(
            Method::POST,
            self.get_api_config().get_server_key(),
            &api_url,
            "",
            self.get_api_config().get_custom_headers().clone(),
            self.get_api_config().get_proxies().clone()
        )?;

        Ok(response)
    }

    fn deny(&self, transaction_id: String) -> MidtransResult {
        let api_url = format!(
            "{}/v2/{}/deny",
            self.get_api_config().get_core_api_base_url(),
            transaction_id
        );

        let response = self.request(
            Method::POST,
            self.get_api_config().get_server_key(),
            &api_url,
            "",
            self.get_api_config().get_custom_headers().clone(),
            self.get_api_config().get_proxies().clone()
        )?;

        Ok(response)
    }

    fn cancel(&self, transaction_id: String) -> MidtransResult {
        let api_url = format!(
            "{}/v2/{}/cancel",
            self.get_api_config().get_core_api_base_url(),
            transaction_id
        );

        let response = self.request(
            Method::POST,
            self.get_api_config().get_server_key(),
            &api_url,
            "",
            self.get_api_config().get_custom_headers().clone(),
            self.get_api_config().get_proxies().clone()
        )?;

        Ok(response)
    }

    fn expire(&self, transaction_id: String) -> MidtransResult {
        let api_url = format!(
            "{}/v2/{}/expire",
            self.get_api_config().get_core_api_base_url(),
            transaction_id
        );

        let response = self.request(
            Method::POST,
            self.get_api_config().get_server_key(),
            &api_url,
            "",
            self.get_api_config().get_custom_headers().clone(),
            self.get_api_config().get_proxies().clone()
        )?;

        Ok(response)
    }

    fn refund(&self, transaction_id: String, parameters: &str) -> MidtransResult {
        let api_url = format!(
            "{}/v2/{}/refund",
            self.get_api_config().get_core_api_base_url(),
            transaction_id
        );

        let response = self.request(
            Method::POST,
            self.get_api_config().get_server_key(),
            &api_url,
            parameters,
            self.get_api_config().get_custom_headers().clone(),
            self.get_api_config().get_proxies().clone()
        )?;

        Ok(response)
    }

    fn refund_direct(&self, transaction_id: String, parameters: &str) -> MidtransResult {
        let api_url = format!(
            "{}/v2/{}/refund/online/direct",
            self.get_api_config().get_core_api_base_url(),
            transaction_id
        );

        let response = self.request(
            Method::POST,
            self.get_api_config().get_server_key(),
            &api_url,
            parameters,
            self.get_api_config().get_custom_headers().clone(),
            self.get_api_config().get_proxies().clone()
        )?;

        Ok(response)
    }

    fn notification_from_json(&self, notification: HashMap<String, Value>) -> MidtransResult {
        let transaction_id = notification["transaction_id"].as_str().unwrap();
        self.status(transaction_id.to_string())
    }

    fn notification_from_str(&self, notification: &str) -> MidtransResult {
        let notification: HashMap<String, Value> = serde_json::from_str(notification)?;
        self.notification_from_json(notification)
    }
}