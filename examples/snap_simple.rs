//! Snap Simple Example
//!
//! This is just for very basic implementation reference,
//! in production, you should validate the incoming requests
//! and implement your backend more securely.
//!
//! Please refer to this docs for snap popup:
//! https://docs.midtrans.com/en/snap/integration-guide?id=integration-steps-overview
//!
//! Please refer to this docs for snap-redirect:
//! https://docs.midtrans.com/en/snap/integration-guide?id=alternative-way-to-display-snap-payment-page-via-redirect

use std::env;
use midtransclient::{MidtransError, Snap};
use serde_json::json;

fn main() -> Result<(), MidtransError> {
    // Get MIDTRANS_SERVER_KEY and MIDTRANS_CLIENT_KEY from environment variables
    // or you just can type it directly
    // You can find it in Merchant Portal -> Settings -> Access keys
    let server_key = env::var("MIDTRANS_SERVER_KEY").expect("SERVER_KEY NOT FOUND");
    let client_key = env::var("MIDTRANS_CLIENT_KEY").expect("CLIENT_KEY NOT FOUND");

    // Create Core API instance
    let snap = Snap::new(false, server_key)
        .client_key(client_key)
        .build()?;

    // Prepare SNAP API parameter ( refer to: https://snap-docs.midtrans.com )
    // this is full parameter including optionals parameter.
    let parameters = json!({
        "transaction_details": {
            "order_id": "test-transaction-123",
            "gross_amount": 200000
        }, "credit_card":{
            "secure" : true
        }
    }).to_string();

    let transaction = snap.create_transaction(&parameters)?;
    println!("Create Transaction Response: {:#?}", transaction);

    // Create Transaction Response: {
    //     "redirect_url": String("https://app.sandbox.midtrans.com/snap/v3/redirection/1115ee78-9e17-4089-9992-a6f39e355fa7"),
    //     "token": String("1115ee78-9e17-4089-9992-a6f39e355fa7"),
    //     "status_code": String("201"),
    // }

    Ok(())
}