//! # Midtransclient for Rust
//!
//! Unofficial Rust API client/library for Midtrans Payment API
//!
//! ## Usage
//!
//! Midtrans have [2 different products](https://docs.midtrans.com/en/welcome/index.html) (Snap & Core API) of payment that you can use:
//!
//! - [Snap](#22A-snap) - Customizable payment popup will appear on **your web/app** (no redirection). [doc ref](https://snap-docs.midtrans.com/)
//! - [Snap Redirect](#22B-snap-redirect) - Customer need to be redirected to payment url **hosted by midtrans**. [doc ref](https://snap-docs.midtrans.com/)
//! - [Core API (VT-Direct)](#22C-core-api-vt-direct) - Basic backend implementation, you can customize the frontend embedded on **your web/app** as you like (no redirection). [doc ref](https://api-docs.midtrans.com/)
//!
//! Choose one that you think best for your unique needs.
//!
//! ## Examples
//!
//! ### Core API Simple Charge
//!
//! ```no_run
//! use std::env;
//! use midtransclient::{MidtransError, CoreApi};
//! use serde_json::json;
//!
//! fn main() -> Result<(), MidtransError> {
//!     // Get MIDTRANS_SERVER_KEY and MIDTRANS_CLIENT_KEY from environment variables
//!     // or you just can type it directly
//!     // You can find it in Merchant Portal -> Settings -> Access keys
//!     let server_key = env::var("MIDTRANS_SERVER_KEY").expect("SERVER_KEY NOT FOUND");
//!     let client_key = env::var("MIDTRANS_CLIENT_KEY").expect("CLIENT_KEY NOT FOUND");
//!
//!     // Create Core API instance
//!     let core = CoreApi::new(false, server_key)
//!         .client_key(client_key)
//!         .build()?;
//!
//!     // Prepare CORE API parameter
//!     // ( refer to: https://docs.midtrans.com/en/core-api/bank-transfer?id=sample-request-and-request-body )
//!     // charge bank_transfer parameter example
//!     let parameters = json!({
//!         "payment_type": "bank_transfer",
//!         "transaction_details": {
//!             "gross_amount": 24145,
//!             "order_id": "test-transaction-321"
//!         },
//!         "bank_transfer": {
//!             "bank": "bni"
//!         }
//!     }).to_string();
//!
//!     // charge transaction
//!     let charge_response = core.charge(&parameters)?;
//!     println!("Charge Response: {:#?}", charge_response);
//!
//!     // Charge Response: {
//!     //     "transaction_status": String("pending"),
//!     //     "va_numbers": Array [Object {
//!     //         "bank": String("bca"),
//!     //         "va_number": String("28276362079")
//!     //     }],
//!     //     "transaction_time": String("2022-11-18 21:01:24"),
//!     //     "status_message": String("Success, Bank Transfer transaction is created"),
//!     //     "fraud_status": String("accept"),
//!     //     "merchant_id": String("G738628276"),
//!     //     "transaction_id": String("73ebe57c-ffb0-42a6-93fa-c5022a7f316e"),
//!     //     "status_code": String("201"),
//!     //     "order_id": String("test-transaction-321"),
//!     //     "currency": String("IDR"),
//!     //     "payment_type": String("bank_transfer"),
//!     //     "gross_amount": String("24145.00")
//!     // }
//!
//!     Ok(())
//! }
//! ```
//!
//! ### Snap Simple
//!
//! ```no_run
//! use std::env;
//! use midtransclient::{MidtransError, Snap};
//! use serde_json::json;
//!
//! fn main() -> Result<(), MidtransError> {
//!     // Get MIDTRANS_SERVER_KEY and MIDTRANS_CLIENT_KEY from environment variables
//!     // or you just can type it directly
//!     // You can find it in Merchant Portal -> Settings -> Access keys
//!     let server_key = env::var("MIDTRANS_SERVER_KEY").expect("SERVER_KEY NOT FOUND");
//!     let client_key = env::var("MIDTRANS_CLIENT_KEY").expect("CLIENT_KEY NOT FOUND");
//!
//!     // Create Core API instance
//!     let snap = Snap::new(false, server_key)
//!         .client_key(client_key)
//!         .build()?;
//!
//!     // Prepare SNAP API parameter ( refer to: https://snap-docs.midtrans.com )
//!     // this is full parameter including optionals parameter.
//!     let parameters = json!({
//!         "transaction_details": {
//!             "order_id": "test-transaction-123",
//!             "gross_amount": 200000
//!         }, "credit_card":{
//!             "secure" : true
//!         }
//!     }).to_string();
//!
//!     let transaction = snap.create_transaction(&parameters)?;
//!     println!("Create Transaction Response: {:#?}", transaction);
//!
//!     // Create Transaction Response: {
//!     //     "redirect_url": String("https://app.sandbox.midtrans.com/snap/v3/redirection/1115ee78-9e17-4089-9992-a6f39e355fa7"),
//!     //     "token": String("1115ee78-9e17-4089-9992-a6f39e355fa7"),
//!     //     "status_code": String("201"),
//!     // }
//!
//!     Ok(())
//! }
//! ```

pub mod config;
pub mod http_client;
pub mod core_api;
pub mod snap;
pub mod error_midtrans;
pub mod transactions;

#[doc(inline)]
pub use config::ApiConfig;
#[doc(inline)]
pub use core_api::CoreApi;
#[doc(inline)]
pub use snap::Snap;
#[doc(inline)]
pub use http_client::MidtransClient;
#[doc(inline)]
pub use error_midtrans::MidtransError;
#[doc(inline)]
pub use transactions::Transactions;
