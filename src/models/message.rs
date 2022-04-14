use crate::client::ZulipClient;

#[derive(Default, Debug)]
pub struct Message {
    pub message_id: Option<u32>,
    pub message_to: Option<String>,
    pub message_content: Option<String>,
    pub message_type: Option<String>,
    pub message_topic: Option<String>
}
