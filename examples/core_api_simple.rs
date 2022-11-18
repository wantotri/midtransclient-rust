//! Core API Simple Charge Example
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
        .client_key(client_key)
        .build()?;

    // Prepare CORE API parameter
    // ( refer to: https://docs.midtrans.com/en/core-api/bank-transfer?id=sample-request-and-request-body )
    // charge bank_transfer parameter example
    let parameters = json!({
        "payment_type": "bank_transfer",
        "transaction_details": {
            "gross_amount": 24145,
            "order_id": "test-transaction-321"
        },
        "bank_transfer": {
            "bank": "bni"
        }
    }).to_string();

    // charge transaction
    let charge_response = core.charge(&parameters)?;
    println!("Charge Response: {:#?}", charge_response);

    // Charge Response: {
    //     "transaction_status": String("pending"),
    //     "va_numbers": Array [Object {
    //         "bank": String("bca"),
    //         "va_number": String("28276362079")
    //     }],
    //     "transaction_time": String("2022-11-18 21:01:24"),
    //     "status_message": String("Success, Bank Transfer transaction is created"),
    //     "fraud_status": String("accept"),
    //     "merchant_id": String("G738628276"),
    //     "transaction_id": String("73ebe57c-ffb0-42a6-93fa-c5022a7f316e"),
    //     "status_code": String("201"),
    //     "order_id": String("test-transaction-321"),
    //     "currency": String("IDR"),
    //     "payment_type": String("bank_transfer"),
    //     "gross_amount": String("24145.00")
    // }

    Ok(())
}