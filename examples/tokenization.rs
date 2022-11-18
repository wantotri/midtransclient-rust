//! Tokenization Example
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

    // Prepare parameter ( refer to: https://api-docs.midtrans.com/#create-pay-account )
    // please update with your redirect URL
    let parameters = json!({
        "payment_type": "gopay",
        "gopay_partner": {
            "phone_number": "81234567891",
            "country_code": "62",
            "redirect_url": "https://mywebstore.com/gopay-linking-finish"
        }
    }).to_string();

    // link payment account
    let link_response = core.link_payment_account(&parameters)?;
    println!("link_response: {:#?}", link_response);

    // link_response: {
    //     "status_code": String("201"),
    //     "account_id": String("3925af29-0527-4cc3-82e8-a603a050f0dc"),
    //     "metadata": Object {
    //         "reference_id": String("7b84d990-8dc3-4a00-9815-f5e2c8b38e1c"),
    //     },
    //     "account_status": String("PENDING"),
    //     "payment_type": String("gopay"),
    //     "actions": Array [
    //         Object {
    //             "method": String("GET"),
    //             "name": String("activation-deeplink"),
    //             "url": String("https://api.sandbox.midtrans.com/v2/pay/account/gpar_acee46ab-1cf2-4ef9-95c1-a54b5d6d5c19/link"),
    //         },
    //         Object {
    //             "method": String("GET"),
    //             "name": String("activation-link-url"),
    //             "url": String("https://api.sandbox.midtrans.com/v2/pay/account/gpar_acee46ab-1cf2-4ef9-95c1-a54b5d6d5c19/link"),
    //         },
    //         Object {
    //             "method": String("GET"),
    //             "name": String("activation-link-app"),
    //             "url": String("https://simulator.sandbox.midtrans.com/gopay/partner/web/otp?id=13892f86-3558-400f-8886-b1867f1aacc9"),
    //         },
    //     ],
    // }

    // IMPORTANT: for the first link, the account status is PENDING,
    // you must activate it by accessing one of the URLs on the actions object

    // Activated account_id for example purpose
    let active_account_id = "3925af29-0527-4cc3-82e8-a603a050f0dc";

    // Get payment account by account_id
    let get_response = core.get_payment_account(active_account_id)?;
    println!("get_response: {:#?}", get_response);

    // get_response: {
    //     "payment_type": String("gopay"),
    //     "account_id": String("3925af29-0527-4cc3-82e8-a603a050f0dc"),
    //     "account_status": String("ENABLED"),
    //     "metadata": Object {
    //         "payment_options": Array [
    //             Object {
    //                 "active": Bool(true),
    //                 "balance": Object {
    //                     "currency": String("IDR"),
    //                     "value": String("8000000.00"),
    //                 },
    //                 "metadata": Object {},
    //                 "name": String("PAY_LATER"),
    //                 "token": String("f5c96fc8-0328-4a44-b840-cc5e8f3232a1"),
    //             },
    //             Object {
    //                 "active": Bool(true),
    //                 "balance": Object {
    //                     "currency": String("IDR"),
    //                     "value": String("8000000.00"),
    //                 },
    //                 "metadata": Object {},
    //                 "name": String("GOPAY_WALLET"),
    //                 "token": String("be8b90f7-cf42-4e97-a069-7ad2d4f01799"),
    //             },
    //         ],
    //     },
    //     "status_code": String("200"),
    // }

    // Request charge
    // please update with your redirect URL
    let params = json!({
        "payment_type": "gopay",
        "gopay": {
          "account_id": get_response["account_id"],
          "payment_option_token": get_response["metadata"]["payment_options"][0]["token"],
          "callback_url": "https://mywebstore.com/gopay-linking-finish"
        },
        "transaction_details": {
          "gross_amount": 100000,
          "order_id": "GOPAY-LINK-12345"
        }
    }).to_string();

    let charge_response = core.charge(&params)?;
    println!("Charge Response: {:#?}", charge_response);

    // Charge Response: {
    //     "settlement_time": String("2022-11-18 23:11:30"),
    //     "gross_amount": String("100000.00"),
    //     "status_code": String("200"),
    //     "currency": String("IDR"),
    //     "order_id": String("GOPAY-LINK-12345"),
    //     "transaction_id": String("d176240d-5c74-4dd7-b2c5-7fa365838af3"),
    //     "payment_type": String("gopay"),
    //     "status_message": String("Success, GoPay transaction is successful"),
    //     "transaction_time": String("2022-11-18 23:11:29"),
    //     "merchant_id": String("G738628276"),
    //     "transaction_status": String("settlement"),
    //     "fraud_status": String("accept"),
    // }

    // unlink payment account by account_id
    // when account status still PENDING, you will get status code 412
    // sample response
    // {
    //     "status_code": String("412"),
    //     "status_message": String("Account status cannot be updated."),
    //     "id": String("19eda9e4-37c9-4bfd-abb2-c60bb3a91084")
    // }
    let unlink_response = core.unlink_payment_account(active_account_id)?;
    println!("unlink_respone: {:#?}", unlink_response);

    // unlink_response: {
    //     "account_id": String("3925af29-0527-4cc3-82e8-a603a050f0dc"),
    //     "status_code": String("204"),
    //     "payment_type": String("gopay"),
    //     "account_status": String("DISABLED"),
    // }

    Ok(())
}