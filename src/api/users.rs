use crate::client::{ZulipClient, ZulipRequest};

pub struct UsersClient<'z> {
    client: &'z ZulipClient
}

impl<'z> UsersClient<'z> {
    pub fn new(client: &'z ZulipClient) -> Self {
        Self {
            client
        }
    }
    
    pub fn get_users(&self) -> UsersGetBuilder {
        UsersGetBuilder::new(self.client)
    }

    pub fn get_user_by_id(&self, user_id: u32) -> UsersGetBuilder<'_> {
        UsersGetBuilder::new(self.client).id(user_id)
    }

    pub fn get_user_by_email(&self, email: &str) -> UsersGetBuilder<'_> {
        UsersGetBuilder::new(self.client).email(email)
    }

    pub async fn get_profile(&self) -> String {
       self.client.get(format!("users/me")).send().await
    }
}

pub struct UsersGetBuilder<'z> {
    client: &'z ZulipClient,
    email: Option<String>,
    user_id: Option<u32>,
    client_gravatar: Option<String>,
    include_custom_profile_fields: Option<String>,
}

impl <'z> UsersGetBuilder<'z> {
    pub fn new(client: &'z ZulipClient) -> Self {
        Self {
            client,
            email: None,
            user_id: None,
            client_gravatar: None,
            include_custom_profile_fields: None,
        }
    }

    pub fn id(mut self, user_id: u32) -> Self {
        self.user_id = Some(user_id);
        self
    }

    pub fn email(mut self, email: &str) -> Self {
        self.email = Some(String::from(email));
        self
    }

    pub fn client_gravatar(mut self, include: bool) -> Self {
        match include {
            true => self.client_gravatar = Some("true".to_string()),
            false => self.client_gravatar = Some("false".to_string()),
        }
        self
    }

    pub fn include_custom_profile_fields(mut self, include: bool) -> Self {
        match include {
            true => self.include_custom_profile_fields = Some("true".to_string()),
            false => self.include_custom_profile_fields = Some("false".to_string()),
        }
        self
    }

    pub async fn send(&self) -> String { 
        let mut endpoint = String::from("users");

        if let Some(id) = self.user_id { 
            endpoint = format!("{}/{}", endpoint, id);
        } else if let Some(email) = &self.email {
            endpoint = format!("{}/{}", endpoint, email);
        }

        self.client.get(endpoint)
            .add_parameter_if_some(
                "client_gravatar".to_string(), 
                &self.client_gravatar
            )
            .add_parameter_if_some(
                "include_custom_profile_fields".to_string(),
                &self.include_custom_profile_fields,
            )
            .send()
            .await
    }
}


