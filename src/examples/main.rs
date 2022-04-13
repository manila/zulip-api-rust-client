#![allow(unused_variables, unused_imports)]
use zulip_api_rust_client::client::{ZulipCredentials, ZulipClient};

#[tokio::main]
async fn main() {
    let auth = ZulipCredentials::new(
                "USER_OR_BOT_EMAIL".to_string(),
                "API_KEY".to_string()
               );

    let zulip_client = ZulipClient::new("REALM_URL".to_string(), auth); 

    let res = zulip_client.messages()
        .get_message(42)
        .send()
        .await;

    println!("{:?}", res);
}

