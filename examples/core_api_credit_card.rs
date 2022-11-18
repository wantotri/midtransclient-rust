//! Core API Simple Credit Card Charge Example
//!
//! This is just for very basic implementation reference,
//! in production, you should validate the incoming requests
//! and implement your backend more securely.

use std::env;
use midtransclient::{MidtransError, CoreApi};
use serde_json::json;

fn main() -> Result<(), MidtransError> {
    // Get MIDTRANS_SERVER_KEY and MIDTRANS_CLIENT_KEY from environment variables
    // or you just can type it directly
    // You can find it in Merchant Portal -> Settings -> Access keys
    let server_key = env::var("MIDTRANS_SERVER_KEY").expect("SERVER_KEY NOT FOUND");
    let client_key = env::var("MIDTRANS_CLIENT_KEY").expect("CLIENT_KEY NOT FOUND");

    // Create Core API instance
    let core = CoreApi::new(false, server_key)
        .client_key(client_key.clone())
        .build()?;

    // Prepare CORE API parameter to get credit card token
    // another sample of card number can refer to
    // https://docs.midtrans.com/en/technical-reference/sandbox-test?id=card-payments
    let parameters = json!({
        "card_number": "5264 2210 3887 4659",
        "card_exp_month": "12",
        "card_exp_year": "2025",
        "card_cvv": "123",
        "client_key": client_key
    }).to_string();

    // Get cc_token
    let card_token_response = core.card_token(&parameters)?;
    let cc_token = card_token_response["token_id"].as_str().unwrap();

    // prepare CORE API parameter to charge credit card
    // ( refer to: https://docs.midtrans.com/en/core-api/credit-card?id=_2-sending-transaction-data-to-charge-api )
    let param = json!({
        "payment_type": "credit_card",
        "transaction_details": {
            "gross_amount": 12145,
            "order_id": "test-transaction-54321",
        },
        "credit_card":{
            "token_id": cc_token
        }
    }).to_string();

    // Charge transaction
    let charge_response = core.charge(&param)?;
    println!("Charge Response: {:#?}", charge_response);

    // Charge Response: {
    //     "gross_amount": String("12145.00"),
    //     "merchant_id": String("G738628276"),
    //     "fraud_status": String("accept"),
    //     "channel_response_message": String("Approved"),
    //     "status_message": String("Success, Credit Card transaction is successful"),
    //     "channel_response_code": String("00"),
    //     "payment_type": String("credit_card"),
    //     "approval_code": String("1668784902694"),
    //     "transaction_status": String("capture"),
    //     "masked_card": String("52642210-4659"),
    //     "on_us": Bool(true),
    //     "card_type": String("debit"),
    //     "status_code": String("200"),
    //     "currency": String("IDR"),
    //     "order_id": String("test-transaction-54321"),
    //     "bank": String("bni"),
    //     "transaction_id": String("d4e7d38d-b68f-4b40-a488-6a935bca7607"),
    //     "transaction_time": String("2022-11-18 22:21:41"),
    // }

    Ok(())
}