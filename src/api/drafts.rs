use crate::client::{ZulipClient, ZulipRequest};
use crate::models::Draft;

pub struct DraftsClient<'z> {
    client: &'z ZulipClient
}

impl<'z> DraftsClient<'z> {
    pub fn new(client: &'z ZulipClient) -> Self {
        Self {
            client
        }
    }

    pub fn get_drafts(&self) -> DraftsGetBuilder {
        DraftsGetBuilder::new(self.client)
    }

    /*
    pub fn create_draft() -> DraftsCreateBuilder {
    }

    pub fn edit_draft() -> DraftsEditBuilder {
    }

    */
    pub fn delete_draft(&self, draft_id: u32) -> DraftsDeleteBuilder {
        DraftsDeleteBuilder::new(self.client, draft_id)
    }
}
 
pub struct DraftsBuilder<'z> {
    client: &'z ZulipClient,
    id: Option<u32>,
    draft_type: String,
    to: String,
    content: String,
    timestamp: Option<String>
}


pub struct DraftsGetBuilder<'z> {
    client: &'z ZulipClient,
}

impl<'z> DraftsGetBuilder<'z> {
    pub fn new(client: &'z ZulipClient) -> Self {
        Self {
            client
        }
    }

    pub async fn send(&self) {
        self.client.get(format!("drafts"))
            .send()
            .await;
    }
}

pub struct DraftsCreateBuilder<'z> {
    client: &'z ZulipClient,
}

pub struct DraftsEditBuilder<'z> {
    client: &'z ZulipClient,
    draft: Draft,
}

pub struct DraftsDeleteBuilder<'z> {
    client: &'z ZulipClient,
    draft_id: u32,
}

impl<'z> DraftsDeleteBuilder<'z> {
    pub fn new(client: &'z ZulipClient, draft_id: u32) -> Self {
        Self {
            client,
            draft_id
        }
    }

    pub async fn send(&self) {
        self.client.delete(format!("drafts/{}", self.draft_id))
            .send()
            .await;
    }
}
