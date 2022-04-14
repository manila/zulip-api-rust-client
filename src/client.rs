#![allow(dead_code)]
use reqwest;
use std::collections::HashMap;
use crate::api::messages::MessagesClient;
use crate::api::drafts::DraftsClient;

#[derive(Copy, Clone, Debug)]
pub enum RequestMethod {
    GET,
    POST,
    DELETE,
}

#[derive(Clone, Debug)]
pub struct ZulipRequest {
    method: RequestMethod,
    realm: String,
    endpoint: String,
    parameters: HashMap<String, String>,
    credentials: ZulipCredentials,
}

impl ZulipRequest {
    pub fn new(method: RequestMethod, realm: String, endpoint: String) -> Self {
        Self {
            method,
            realm,
            endpoint,
            credentials: ZulipCredentials::new("".to_string(), "".to_string()),
            parameters: HashMap::new(),
        }
    }
    
    pub fn endpoint(&mut self, endpoint: String) -> &mut Self {
        self.endpoint = endpoint;
        self
    }

    pub fn add_parameter(&mut self, key: String, value: String) -> &mut Self {
        self.parameters.insert(key, value);
        self
    }

    pub fn auth(&mut self, credentials: ZulipCredentials) -> &mut Self {
        self.credentials = credentials;
        self
    }

    pub fn build(&self) -> ZulipRequest {
       ZulipRequest {
            method: self.method,
            realm: self.realm.clone(),
            endpoint: self.endpoint.clone(),
            parameters: self.parameters.clone(),
            credentials: self.credentials.clone(),
       }
    }

    pub async fn send(&self) -> String {
        let Self { method, endpoint, credentials, parameters, realm } = self;
        let client = reqwest::Client::new();
        let endpoint = format!("https://{}/api/v1/{}", realm, endpoint);

        let req = match method {
            RequestMethod::GET => client.get(endpoint),
            RequestMethod::POST => client.post(endpoint),
            RequestMethod::DELETE => client.delete(endpoint),
        };

        let res = req
            .basic_auth(credentials.email.to_string(),Some(credentials.api_key.to_string()))
            .query(&self.parameters)
            .send()
            .await
            .unwrap()
            .text()
            .await;

        format!("{:#?}",res)
    }
}

#[derive(Clone, Debug)]
pub struct ZulipCredentials {
    pub email: String,
    pub api_key: String,
}

impl Default for ZulipCredentials {
    fn default() -> Self {
        Self {
            email: "example@example.com".to_string(),
            api_key: "example@example.com".to_string(),
        }
    }
}

impl ZulipCredentials {
    pub fn new(email: String, api_key: String) -> Self {
        Self {
            email,
            api_key,
        }
    }
}

#[derive(Debug, Clone)]
pub struct ZulipClient {
    pub realm: String,
    pub httpclient: reqwest::Client,
    pub credentials: ZulipCredentials,
}

impl Default for ZulipClient {
    fn default() -> Self {
        Self {
            realm: "".to_string(),
            httpclient: reqwest::Client::new(),
            credentials: ZulipCredentials::default()
        }
    }
}

impl ZulipClient {
    pub fn new(realm: String, credentials: ZulipCredentials) -> Self {
        Self {
            realm,
            credentials,
            ..Default::default()
        }
    }

    pub fn messages(&self) -> MessagesClient {
        MessagesClient::new(self)
    }

    pub fn drafts(&self) -> DraftsClient {
        DraftsClient::new(self)
    }

    pub fn get(&self, endpoint: String) -> ZulipRequest {
        ZulipRequest::new(RequestMethod::GET, self.realm.clone(), endpoint)
            .auth(self.credentials.clone())
            .build()
    }

    pub fn post(&self, endpoint: String) -> ZulipRequest {
        ZulipRequest::new(RequestMethod::POST, self.realm.clone(), endpoint)
            .auth(self.credentials.clone())
            .build()

    }

    pub fn delete(&self, endpoint: String) -> ZulipRequest {
        ZulipRequest::new(RequestMethod::DELETE, self.realm.clone(), endpoint)
            .auth(self.credentials.clone())
            .build()
        
    }
}
