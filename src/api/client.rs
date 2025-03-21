use anyhow::Result;
use reqwest::{Client, Url};
use crate::models::{Server, Application};

pub struct CoolifyClient {
    client: Client,
    base_url: Url,
    api_key: String,
}

impl CoolifyClient {
    pub fn new(config: &str) -> Self {
        Self {  
            client: Client::new(),
            base_url: Url::parse(config).unwrap(),
            api_key: "".to_string(),
        }
    }

    pub async fn list_servers(&self) -> Result<Vec<Server>> {
        Ok(vec![])
    }
    
    pub async fn list_applications(&self) -> Result<Vec<Application>> {
        Ok(vec![]) 
    }

    pub async fn deploy_application(&self, id: &str) -> Result<()> {
        Ok(())
    }

    pub async fn get_application_logs(&self, id: &str) -> Result<String> {
        Ok(String::new())
    }
}
