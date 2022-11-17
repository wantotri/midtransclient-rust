Midtrans Client - Rust
======================

This is Unofficial Rust API client/library for Midtrans Payment API.

## 1. Installation

```
cargo install --git https://github.com/wantotri/midtransclient.git
```


## 2. Usage

### 2.1 Choose Product/Method

- [Snap](#22A-snap) - Customizable payment popup will appear on **your web/app** (no redirection). [doc ref](https://snap-docs.midtrans.com/)
- [Snap Redirect](#22B-snap-redirect) - Customer need to be redirected to payment url **hosted by midtrans**. [doc ref](https://snap-docs.midtrans.com/)
- [Core API (VT-Direct)](#22C-core-api-vt-direct) - Basic backend implementation, you can customize the frontend embedded on **your web/app** as you like (no redirection). [doc ref](https://api-docs.midtrans.com/)

### 2.2 Client Initialization and Configuration

Get your client key and server key from [Midtrans Dashboard](https://dashboard.midtrans.com)

Create API client object

```rust
// Create Core API instance
let core = CoreApi::new(false, "YOUR_SERVER_KEY".to_string())
    .client_key("YOUR_CLIENT_KEY".to_string())
    .build()
    .unwrap();
```


```rust
// Create Snap API instance
let snap = Snap::new(false, "YOUR_SERVER_KEY")
    .client_key("YOUR_CLIENT_KEY")
    .build()
    .unwrap();
```


You can also re-set config using `core.set_api_config(...)` or `core.api_cofig.set_<fieldname>(...)`
example:


```rust
let core = CoreApi::new(true, "SOME_KEY".to_string()).build().unwrap();
let new_config = ApiConfig::new(false, "SERVER_KEY".to_string()).build();
core.set_api_config(new_config);

// Or you can change spesific config
let snap = Snap::new(true, "SOME_KEY".to_string()).build().unwrap();
snap.api_config.set_is_production(false);
snap.api_config.set_client_key("CLIENT_KEY".to_string());
```


### 2.2.A Snap
You can see Snap example [here](examples/snap).

Available methods for `Snap` class
```rust
pub fn create_transaction(&self, parameters: &str) -> MidtransResult

pub fn create_transaction_token(&self, parameters: &str) -> Result<Value, MidtransError>

pub fn create_transaction_redirect_url(&self, parameters: &str) -> Result<Value, MidtransError>
```
`parameters` is String of JSON of [SNAP Parameter](https://snap-docs.midtrans.com/#json-objects)


#### Get Snap Token

```rust
// Create Snap API instance
let snap = Snap::new(false, "YOUR_SERVER_KEY")
    .client_key("YOUR_CLIENT_KEY")
    .build()
    .unwrap();

// Prepare parameter
let parameters = r#"{
    "transaction_details": {
        "order_id": "test-transaction-123",
        "gross_amount": 200000
    }, "credit_card":{
        "secure" : True
    }
}"#;

let transaction = snap.create_transaction(parameters).unwrap();
let transaction_token = transaction["token"];
// alternative way to create transaction_token:
let transaction_token = snap.create_transaction_token(&parameters).unwrap();
```


#### Initialize Snap JS when customer click pay button

Replace `PUT_TRANSACTION_TOKEN_HERE` with `transaction_token` acquired above
```html
<html>
  <body>
    <button id="pay-button">Pay!</button>
    <pre><div id="result-json">JSON result will appear here after payment:<br></div></pre>

    <!-- TODO: Remove ".sandbox" from script src URL for production environment. Also input your client key in "data-client-key" -->
    <script src="https://app.sandbox.midtrans.com/snap/snap.js" data-client-key="<Set your ClientKey here>"></script>
    <script type="text/javascript">
      document.getElementById('pay-button').onclick = function(){
        // SnapToken acquired from previous step
        snap.pay('PUT_TRANSACTION_TOKEN_HERE', {
          // Optional
          onSuccess: function(result){
            /* You may add your own js here, this is just example */
            document.getElementById('result-json').innerHTML += JSON.stringify(result, null, 2);
          },
          // Optional
          onPending: function(result){
            /* You may add your own js here, this is just example */
            document.getElementById('result-json').innerHTML += JSON.stringify(result, null, 2);
          },
          // Optional
          onError: function(result){
            /* You may add your own js here, this is just example */
            document.getElementById('result-json').innerHTML += JSON.stringify(result, null, 2);
          }
        });
      };
    </script>
  </body>
</html>
```

#### Implement Notification Handler
[Refer to this section](#23-handle-http-notification)


### 2.2.B Snap Redirect

#### Get Redirection URL of a Payment Page

```rust
// Create Snap API instance
let snap = Snap::new(false, "YOUR_SERVER_KEY")
    .client_key("YOUR_CLIENT_KEY")
    .build()
    .unwrap();

// Prepare parameter
let parameters = r#"{
    "transaction_details": {
        "order_id": "test-transaction-123",
        "gross_amount": 200000
    }, "credit_card":{
        "secure" : True
    }
}"#;

let transaction = snap.create_transaction(param).unwrap();
let transaction_redirect_url = transaction['redirect_url'];
// alternative way to create redirect_url
let transaction_redirect_url = snap.create_transaction_redirect_url(param).unwrap();
```
#### Implement Notification Handler
[Refer to this section](#23-handle-http-notification)


### 2.2.C Core API (VT-Direct)

Available methods for `CoreApi` struct

```rust
pub fn charge(&self, parameters: &str) -> MidtransResult

pub fn capture(&self, parameters: &str) -> MidtransResult

pub fn card_register(&self, parameters: &str) -> MidtransResult

pub fn card_token(&self, parameters: &str) -> MidtransResult

pub fn card_point_inquiry(&self, token_id: &str) -> MidtransResult
```
`parameters` is String of JSON of [SNAP Parameter](https://snap-docs.midtrans.com/#json-objects)

#### Credit Card Get Token

Get token should be handled on  Frontend please refer to [API docs](https://api-docs.midtrans.com)

#### Credit Card Charge

```rust
// Create Core API instance
let snap = CoreApi::new(false, "YOUR_SERVER_KEY")
    .client_key("YOUR_CLIENT_KEY")
    .build()
    .unwrap();

// Prepare parameter
let parameters = r#"{
    "payment_type": "credit_card",
    "transaction_details": {
        "gross_amount": 12145,
        "order_id": "test-transaction-54321",
    },
    "credit_card":{
        "token_id": "<CREDIT_CARD_TOKEN>",
        "authentication": True
    }
}"#;

// charge transaction
let charge_response = core.charge(param).unwrap();
println!("charge_response: {:?}", charge_response);
```

#### Credit Card 3DS Authentication

The credit card charge result may contains `redirect_url` for 3DS authentication. 3DS Authentication should be handled on Frontend please refer to [API docs](https://api-docs.midtrans.com/#card-features-3d-secure)

For full example on Credit Card 3DS transaction refer to:
- [Flask App examples](/examples/flask_app) that implement Snap & Core Api


### 2.2.D Subscription API

#### Subscription API for Credit Card

To use subscription API for credit card, you should first obtain the 1-click saved token, [refer to this docs.](https://docs.midtrans.com/en/core-api/advanced-features?id=recurring-transaction-with-subscriptions-api)

You will receive `saved_token_id` as part of the response when the initial card payment is accepted (will also available in the HTTP notification's JSON), [refer to this docs.](https://docs.midtrans.com/en/core-api/advanced-features?id=sample-3ds-authenticate-json-response-for-the-first-transaction)


```rust
// Create Subscription API instance
let core = CoreApi::new(false, "SERVER_KEY")
    .client_key("CLIENT_KEY")
    .build()
    .unwrap();

// Prepare parameter
let parameters = r#"{
    "name": "SUBSCRIPTION-STARTER-1",
    "amount": "100000",
    "currency": "IDR",
    "payment_type": "credit_card",
    "token": "436502qFfqfAQKScMtPRPdZDOaeg7199",
    "schedule": {
      "interval": 1,
      "interval_unit": "month",
      "max_interval": 3,
      "start_time": "2021-10-01 07:25:01 +0700"
    },
    "metadata": {
      "description": "Recurring payment for STARTER 1"
    },
    "customer_details": {
      "first_name": "John A",
      "last_name": "Doe A",
      "email": "johndoe@email.com",
      "phone": "+62812345678"
    }
}"#;

let create_response = core.create_subscription(parameters).unwrap();
let subscription_id = create_response["id"].as_str().unwrap();

// Get subscription by subscription_id
let get_response = core.get_subscription(subscription_id).unwrap();

// Disable subscription by subscription_id
let disable_response = core.disable_subscription(subscription_id).unwrap();

// Enable subscription by subscription_id
let enable_response = core.enable_subscription(subscription_id).unwrap();

// Update subscription by subscription_id
let parameters = r#"{
    "name": "SUBSCRIPTION-STARTER-1-UPDATE",
    "amount": "100000",
    "currency": "IDR",
    "token": "436502qFfqfAQKScMtPRPdZDOaeg7199",
    "schedule": {
      "interval": 1
    }
}"#;

let update_response = core.update_subscription(subscription_id, parameters).unwrap();
```

#### Subscription API for Gopay

To use subscription API for gopay, you should first link your customer gopay account with gopay tokenization API, [refer to this section](#22e-tokenization-api)

You will receive gopay payment token using `get_payment_account` API call

### 2.2.E Tokenization API

```rust
// Create Subscription API instance
let core = CoreApi::new(false, "SERVER_KEY")
    .client_key("CLIENT_KEY")
    .build()
    .unwrap();

// Prepare parameter
// please redirect_url update with your redirect URL
let parameters = r#"{
    "payment_type": "gopay",
    "gopay_partner": {
        "phone_number": "81234567891",
        "country_code": "62",
        "redirect_url": "https://mywebstore.com/gopay-linking-finish"
    }
}"#;

// Link payment account
let link_response = core.link_payment_account(parameters).unwrap();

// Get payment account
let get_response = core.get_payment_account(active_account_id).unwrap();

// unlink payment account
let unlink_resposne = core.unlink_payment_account(active_account_id).unwrap();
```


## 2.3 Hanlde HTTP Notification

> **IMPORTANT NOTE**: To update transaction status on your backend/database, **DO NOT** solely rely on frontend callbacks! For security reason to make sure the status is authentically coming from Midtrans, only update transaction status based on HTTP Notification or API Get Status.

Create separated web endpoint (notification url) to receive HTTP POST notification callback/webhook.
HTTP notification will be sent whenever transaction status is changed

```rust
// Create Core API / Snap instance (both have shared `transactions` methods)
let core = CoreApi::new(false, "SERVER_KEY")
    .client_key("CLIENT_KEY")
    .build()
    .unwrap();

let status_response = core.status(transaction_id).unwrap();
let notification_response = core.notification_from_json(status_response).unwrap();

println!(
    "Transaction notification received. Order ID: {}. Transaction status: {}. Fraud status: {}",
    notification_response["order_id"],
    notification_response["transaction_status"],
    notification_response["fraud_status"]
);

// Sample transaction_status handling logic
if transaction_status == "capture" {
  if fraud_status == "challenge" {
    // TODO set transaction status on your databaase to "challenge"
  }
  else if fraud_status == "accept" {
    // TODO set transaction status on your databaase to"success"
  }
} else if transaction_status == "cancel"
|| transaction_status == "deny"
|| transaction_status == "expire" {
    // TODO set transaction status on your databaase to "failure"
} else if transaction_status == "pending" {
    // TODO set transaction status on your databaase to "pending" / waiting payment
}
```


### 2.4 Transaction Actions

#### Get Status
```rust
// Get status of transaction that already recorded on midtrans (already `charge`-ed)
let status_response = core.status(transaction_id).unwrap();
```

#### Get Status B2B
```rust
// Get transaction status of VA b2b transaction
let status_response = core.statusb2b(transaction_id).unwrap();
```

#### Approve Transaction
```rust
// Approve a credit card transaction with `challange` fraud status
let status_response = core.approve(transaction_id).unwrap();
```

#### Deny Transaction
```rust
// Deny a credit card transaction with `challange` fraud status
let status_response = core.deny(transaction_id).unwrap();
```

#### Cancel Transaction
```rust
// cancel a credit card transaction or pending transaction
let status_response = core.cancel(transaction_id).unwrap();
```

#### Expire Transaction
```rust
// expire a pending transaction
let status_response = core.expire(transaction_id).unwrap();
```

#### Refund Transaction
```rust
// refund a transaction (not all payment channel allow refund via API)
let parameters = r#"{
    "refund_key": "order1-ref1",
    "amount": 5000,
    "reason": "Item out of stock"
}"#;
let status_response = core.refund(transaction_id, parameters).unwrap();
```

#### Refund Transaction with Direct Refund
```rust
// refund a transaction (not all payment channel allow refund via API) with Direct Refund
let parameters = r#"{
    "refund_key": "order1-ref1",
    "amount": 5000,
    "reason": "Item out of stock"
}"#;
let status_response = core.refund_direct(transaction_id, parameters).unwrap();
```


## 3. Handling MidtransError

When using method that return `Result<_, MidtransError>` in Midtrans API calls: `core.charge(...)` or `snap.create_transaction(...)`, you can handle it kinda like this:

```rust
let charge_response = core.charge(parameters);

match charge_response {
    Ok(ref result) => println!("{:?}", result),
    Err(ref err) => match err {
        MidtransError::RequestError(e) => println!("{e}"),
        MidtransError::JsonDecodeError(e) => println!("{e}"),
        MidtransError::ParseError(e) => println!("{e}"),
        MidtransError::ApiError(e) => println!("{e}")
    }
};
```


## 4. Advanced Usage

### Custom HTTP Headers

```rust
// Create Core API instance
let core = CoreApi::new(false, "SERVER_KEY")
    .client_key("CLIENT_KEY")
    .build()
    .unwrap();

// Set custom HTTP header for every request from this instance
let mut custom_headers = HeaderMap::new();
custom_headers.insert("X-Custom-Header", "Some Value".parse().unwrap());
core.api_config.set_custom_headers(custom_headers);
```

### Override/Append HTTP Notification URL

As [described in API docs](https://snap-docs.midtrans.com/#override-notification-url), merchant can opt to change or add custom notification urls on every transaction. It can be achieved by adding additional HTTP headers into charge request.

This can be achieved by:
```rust
// Create Snap instance
let snap = Snap::new(false, "SERVER_KEY")
    .client_key("CLIENT_KEY")
    .build()
    .unwrap();

// set custom HTTP header that will be used by Midtrans API to override notification url
let mut custom_headers = HeaderMap::new();
custom_headers.insert("x-override-notification", "https://example.org".parse().unwrap());
snap.api_config.set_custom_headers(custom_headers);

// or append notification
let mut custom_headers = HeaderMap::new();
custom_headers.insert("x-append-notification", "https://example.org".parse().unwrap());
snap.api_config.set_custom_headers(custom_headers);
```


### Custom HTTP Proxy

```rust
// Create Snap instance
let snap = Snap::new(false, "SERVER_KEY")
    .client_key("CLIENT_KEY")
    .build()
    .unwrap();

let proxies = reqwest::Proxy::http("https://secure.example").unwrap();
snap.api_config.set_proxies(proxies);
```

Under the hood this API wrapper is using [reqwest](https://docs.rs/reqwest/latest/reqwest/) as http client. You can further [learn about proxies on its documentation](https://docs.rs/reqwest/latest/reqwest/struct.Proxy.html)


#### Get help

* [Midtrans Docs](https://docs.midtrans.com)
* [Midtrans Dashboard ](https://dashboard.midtrans.com/)
* [SNAP documentation](http://snap-docs.midtrans.com)
* [Core API documentation](http://api-docs.midtrans.com)


## License

[MIT License](LICENSE)