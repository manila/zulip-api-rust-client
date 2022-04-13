use serde::ser::{Serialize, SerializeStruct, Serializer};
use std::result::Result;
use std::collections::HashMap;
use crate::models::{Message};
use crate::client::{ZulipClient, ZulipRequest};
use reqwest::StatusCode;

impl Serialize for Message {
    fn serialize<S>(&self, serializer: S) -> Result<S::Ok, S::Error>
        where
            S: Serializer,
        {
            let mut state = serializer.serialize_struct("Message", 1)?;
            
            if let Some(message_type) = &self.message_type {
               state.serialize_field("type", message_type)?;
            }

            if let Some(message_id) = &self.message_id {
                state.serialize_field("id", &self.message_id)?;
            }

            state.end()
        }
}

pub struct MessageClient<'z> {
    client: &'z ZulipClient,
    message: Option<Message>
}

impl<'z> MessageClient<'z> {
    pub fn new(client: &'z ZulipClient) -> Self {
        Self {
            client,
            message: None,
        }
    }

    pub fn delete_message(&self, message_id: u32) {
    }

    pub fn get_message(&self, message_id: u32) -> MessagesGetBuilder {
        MessagesGetBuilder::new(self.client)
    }
}

pub struct MessageSendBuilder<'z> {
    client: &'z ZulipClient,
    msg_type: String,
    to: String,
    content: String,
    topic: Option<String>,
    quene_id: Option<String>,
    local_id: Option<String>,
}

impl<'z> MessageSendBuilder<'z> {
    pub fn new(
        client: &'z ZulipClient,
        msg_type: String, 
        to: String, 
        content: String
        ) -> Self {
        Self {
            client,
            msg_type,
            to,
            content,
            topic: None,
            quene_id: None,
            local_id: None
        }
    }

    pub fn topic(mut self, msg_topic: String) -> Self {
        self.topic = Some(msg_topic);
        self
    }

    pub fn quene_id(mut self, id: String) -> Self {
        self.quene_id = Some(id.to_owned());
        self
    }

    pub fn local_id(mut self, id: String) -> Self {
        self.local_id = Some(id.to_owned());
        self
    }

    pub async fn send(&self) -> Result<String, String> {
        self.client.post(format!("messages"))
            .add_parameter("type".to_string(), "private".to_string())
            .add_parameter("to".to_string(), self.to.to_string())
            .add_parameter("content".to_string(), self.content.to_string());

        Ok("Yay!".to_string())
    }
}

pub struct MessageEditBuilder<'z> {
    client: &'z ZulipClient,
    message_id: u32,
    topic: Option<String>,
    propogate_mode: Option<String>,
    send_notification_to_old_thread: Option<bool>,
    send_notification_to_new_thread: Option<bool>,
    content: Option<String>,
    stream_id: Option<u32>,
}

impl<'z> MessageEditBuilder<'z> {
    pub fn new(client: &'z ZulipClient, message_id: u32) -> Self {
        Self {
            client,
            message_id,
            topic: None,
            propogate_mode: None,
            send_notification_to_old_thread: None,
            send_notification_to_new_thread: None,
            content: None,
            stream_id: None,
        }
    }

    pub fn topic(mut self, message_topic: String) -> Self {
        self.topic = Some(message_topic.to_owned());
        self
    }

    pub fn propogate_mode(mut self, mode: String) -> Self {
        self.propogate_mode = Some(mode);
        self
    }

    pub fn send_notification_to_old_thread(mut self, ans: bool) -> Self {
        self.send_notification_to_old_thread = Some(ans);
        self
    }

    pub fn send_notification_to_new_thread(mut self, ans: bool) -> Self {
        self.send_notification_to_new_thread = Some(ans);
        self
    }

    pub fn content(mut self, message_content: String) -> Self {
        self.content = Some(message_content);
        self
    }

    pub fn stream_id(mut self, id: u32) -> Self {
        self.stream_id = Some(id);
        self
    }

    pub async fn send(&self) {
        self.client
            .get(format!("messages/{}", self.message_id));
    }
}

pub struct MessagesGetBuilder<'z> {
    client: &'z ZulipClient,
    message_id: Option<u32>,
}

impl<'z> MessagesGetBuilder<'z> {
    fn new(client: &'z ZulipClient) -> Self {
        Self {
            client,
            message_id: None
        }
    }

    pub fn id(&mut self, message_id: u32) -> Self {
        Self {
            client: self.client,
            message_id: Some(message_id)
        }
    }

    pub async fn send(&self) {
        let Self { client, message_id } = self;

        let res = client.get(format!("messages/{}", message_id.unwrap()))
            //.add_parameter("apply_markdown".to_string(), "true".to_string())
            .send()
            .await;
        
        println!("{:#?}", res);
    }
}

pub struct MessageBuilder<'z> {
    client: &'z ZulipClient,
    message: Option<Message>,
}

impl<'z> MessageBuilder<'z> {
    pub fn new(client: &'z ZulipClient) -> Self {
        Self {
            client: client,
            message: None,
        }
    }

    pub fn send_message(&mut self, msg_type: String, to: String, content: String) -> MessageSendBuilder {
        MessageSendBuilder::new(self.client, msg_type, to, content)
    }

    pub fn get_message(&self, message_id: u32) -> MessagesGetBuilder<'_> {
        MessagesGetBuilder::new(self.client).id(message_id)
    }

    pub fn get_messages(&mut self, num_before: u32, num_after: u32) {
        /*
        self.client
            .get("messages")
            .add_parameter()
            .add_parameter()
        */
        /*
        self.request = ZulipRequest::new(self.client.clone())
            .get()
            .endpoint("messages".to_string())
            .parameters(HashMap::from([
                ("num_before".to_string(), format!("{}", num_before)),
                ("num_after".to_string(), format!("{}", num_after)),
            ]))
            .build()
            */
    }

    /*
    pub async fn send(&self) {
        let ZulipClient { realm, httpclient, credentials } = self.client;
        let res = httpclient
            .get(format!("https://{}/api/v1/{}", realm, self.request.endpoint))
            .basic_auth(
                credentials.email.clone(), 
                Some(credentials.api_key.clone())
            )
            .send()
            .await
            .unwrap();

        match res.status() {
            _ => println!("{:?}",res.text().await)
        }
    }
    */
}
