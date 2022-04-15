#![allow(dead_code)]
use reqwest;
use std::collections::HashMap;
use crate::api::messages::MessagesClient;
use crate::api::drafts::DraftsClient;

#[derive(Copy, Clone, Debug)]
pub enum RequestMethod {
    GET,
    POST,
    PATCH,
    DELETE,
}

#[derive(Clone, Debug)]
pub struct ZulipRequest<'z> {
    httpclient: &'z reqwest::Client,
    method: RequestMethod,
    realm: &'z String,
    endpoint: String,
    parameters: HashMap<String, String>,
    credentials: &'z ZulipCredentials,
}

impl<'z> ZulipRequest<'z> {
    pub fn new(
        client: &'z ZulipClient, 
        method: RequestMethod, 
        endpoint: String
    ) -> Self {
        Self {
            httpclient: &client.httpclient,
            method,
            realm: &client.realm,
            endpoint,
            credentials: &client.credentials,
            parameters: HashMap::new(),
        }
    }

    pub fn add_parameter(&mut self, key: String, value: &String) -> &mut Self {
        self.parameters.insert(key, value.to_string());
        self
    }

    pub fn add_parameter_if_some(
        &mut self, 
        key: String, 
        value: &Option<String>
    ) -> &mut Self {
        match value {
            Some(v) => self.add_parameter(key, v),
            None => self
        }
    }

    pub async fn send(&self) -> String {
        let Self { method, endpoint, credentials, parameters, realm, httpclient } = self;
        let endpoint = format!("https://{}/api/v1/{}", realm, endpoint);

        let req = match method {
            RequestMethod::GET => httpclient.get(endpoint),
            RequestMethod::POST => httpclient.post(endpoint),
            RequestMethod::PATCH => httpclient.patch(endpoint),
            RequestMethod::DELETE => httpclient.delete(endpoint),
        };

        let res = req
            .basic_auth(
                credentials.email.to_string(), 
                Some(credentials.api_key.to_string())
            )
            .query(&self.parameters)
            .send()
            .await
            .unwrap()
            .text()
            .await;

        format!("{:#?}",res)
    }
}

#[derive(Debug)]
pub struct ZulipCredentials {
    pub email: String,
    pub api_key: String,
}

impl ZulipCredentials {
    pub fn new(email: String, api_key: String) -> Self {
        Self {
            email,
            api_key,
        }
    }
}

pub struct ZulipClient {
    pub realm: String,
    pub httpclient: reqwest::Client,
    pub credentials: ZulipCredentials,
}

impl ZulipClient {
    pub fn new(realm: String, credentials: ZulipCredentials) -> Self {
        Self {
            realm,
            credentials,
            httpclient: reqwest::Client::new(),
        }
    }

    pub fn messages(&self) -> MessagesClient {
        MessagesClient::new(self)
    }

    pub fn drafts(&self) -> DraftsClient {
        DraftsClient::new(self)
    }

    pub fn get(&self, endpoint: String) -> ZulipRequest {
        ZulipRequest::new(self, RequestMethod::GET, endpoint)
    }

    pub fn post(&self, endpoint: String) -> ZulipRequest {
        ZulipRequest::new(self, RequestMethod::POST, endpoint)
    }

    pub fn patch(&self, endpoint: String) -> ZulipRequest {
        ZulipRequest::new(self, RequestMethod::PATCH, endpoint)
    }

    pub fn delete(&self, endpoint: String) -> ZulipRequest {
        ZulipRequest::new(self, RequestMethod::DELETE, endpoint)
        
    }
}
