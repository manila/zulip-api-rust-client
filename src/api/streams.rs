use crate::client::{ZulipClient, ZulipRequest};
use crate::models::Draft;

pub struct StreamsClient<'z> {
    client: &'z ZulipClient
}

impl<'z> StreamsClient<'z> {
    pub fn new(client: &'z ZulipClient) -> Self {
        Self {
            client
        }
    }
}

pub struct StreamsSubscribedClient<'z> {
    client: &'z ZulipClient,
    include_subscribers: Option<String>
}

impl<'z> StreamsSubscribedClient<'z> {
    fn new(client: &'z ZulipClient) -> Self {
        Self {
            client,
            include_subscribers: None
        }
    }

    fn include_subscribers(mut self, include: bool) -> Self {
        match include {
            true => self.include_subscribers = Some("true".to_string()),
            false => self.include_subscribers = Some("false".to_string()),
        }
        self
    }

    pub async fn send(&self) {
        self.client.get(format!("/users/me/subscriptions"))
            .add_parameter_if_some(
                "include_subscribers".to_string(), &self.include_subscribers)
            .send()
            .await;
    }
}

pub struct SteamsSubscriptionStatusClient<'z> {
    client: &'z ZulipClient,
    user_id: u32,
    stream_id: u32,
}

impl<'z> SteamsSubscriptionStatusClient<'z> {
    pub fn new(client: &'z ZulipClient, user_id: u32, stream_id: u32) -> Self {
        Self {
            client,
            user_id,
            stream_id,
        }
    }

    pub async fn send(&self) {
       self.client.get(format!("/users/{}/subscriptions/{}", self.user_id, self.stream_id))
           .send()
           .await;
    }
}
