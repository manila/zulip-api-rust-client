# Zulip API Client Library Rust Crate

This repo contains the code for an unofficial, third-party Zulip API client library crate written in the Rust programming language. This library maps Zulip API endpoints into a set of modules that can be used in a Rust project

## Installing

TODO: How to add to crate to Cargo.toml

## Examples

#### Fetch a single message

```rust
use zulip_api_rust_client::client::{ZulipCredentials, ZulipClient};

#[tokio::main]
async fn main() {
    let auth = ZulipCredentials::new(
                "USER_OR_BOT_EMAIL".to_string(),
                "API_KEY".to_string()
               );

    let zulip_client = ZulipClient::new("REALM_URL".to_string(), auth); 

    zulip_client.messages()
        .get_message(42)
        .send()
        .await;
}
```

## Contributing

PR's are welcome including documentation, typo corrections, tests, feedback, etc. There are many endpoints that have not been fully implemented and they can make for great practical learning oppunitunies (aka good first issues)

*please note: I am a beginner in Rust so it may take me some time to review any PR's and provide helpful feedback or insight*

## Endpoint Coverage

#### Messages

- [ ] send a message
- [ ] upload a file
- [ ] edit a message
- [ ] delete a message
- [ ] get messages
- [ ] construct a narrow
- [ ] add an emoji reaction
- [ ] remove an emoji reaction
- [ ] render a message
- [ ] fetch a single message
- [ ] check if messages match narrow
- [ ] get a message's edit history
- [ ] update personal message flags
- [ ] mark messages as read in bulk

#### Drafts

- [ ] get drafts
- [ ] create drafts
- [ ] edit a draft
- [ ] delete a draft

#### Streams

- [ ] get subscribed streams
- [ ] subscribe to a stream
- [ ] unsubscribe from a stream
- [ ] get subscription
- [ ] update subscription settings
- [ ] get all streams
- [ ] get a stream by ID
- [ ] get stream ID
- [ ] create a stream
- [ ] update a stream
- [ ] archive a stream
- [ ] get topics in a stream
- [ ] topic muting
- [ ] delete a topic
- [ ] add a default stream
- [ ] remove a default stream

## Attribution

The design of this library was and is inspired by [Octocrab](https://github.com/XAMPPRocky/octocrab)

