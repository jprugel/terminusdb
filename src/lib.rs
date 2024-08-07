use serde::{Deserialize, Serialize};
use std::net::{IpAddr, Ipv4Addr, SocketAddr};

#[derive(Debug)]
pub struct Client {
    pub address: SocketAddr,
    username: String,
    password: String,
    organization: Option<Organization>,
    database: Option<Database>,
}

impl Default for Client {
    fn default() -> Self {
        let address = SocketAddr::new(IpAddr::V4(Ipv4Addr::new(127, 0, 0, 1)), 6363);
        let username = "admin".to_string();
        let password = "root".to_string();
        Self {
            address,
            username,
            password,
            organization: None,
            database: None,
        }
    }
}

impl Client {
    pub fn new() -> Self {
        let address = SocketAddr::new(IpAddr::V4(Ipv4Addr::new(127, 0, 0, 1)), 6363);
        let username = "admin".to_string();
        let password = "root".to_string();
        let client = reqwest::blocking::Client::new();
        let organizations = client
            .get(format!("http://{}/api/organizations", address))
            .basic_auth(&username, Some(&password))
            .send()
            .unwrap()
            .json::<Vec<Organization>>()
            .unwrap();
        let databases = client
            .get(format!("http://{}/api/", address))
            .basic_auth(&username, Some(&password))
            .send()
            .unwrap()
            .json::<Vec<Database>>()
            .unwrap();
        Self {
            address,
            username,
            password,
            organization: Some(organizations[0].clone()),
            database: Some(databases[0].clone()),
        }
    }
    pub async fn organizations(&self) -> Vec<Organization> {
        let client = reqwest::Client::new();
        client
            .get(format!("http://{}/api/organizations", self.address))
            .basic_auth(&self.username, Some(&self.password))
            .send()
            .await
            .unwrap()
            .json::<Vec<Organization>>()
            .await
            .unwrap()
    }

    async fn databases(&self) -> Vec<Database> {
        let client = reqwest::Client::new();
        client
            .get(format!("http://{}/api/db/?as_list=true", self.address))
            .basic_auth(&self.username, Some(&self.password))
            .send()
            .await
            .unwrap()
            .json::<Vec<Database>>()
            .await
            .unwrap()
    }
}

#[derive(Serialize, Deserialize, Debug, Clone)]
pub struct Organization {
    name: String,
}

impl Organization {
    fn new(name: &str) -> Self {
        Self {
            name: name.to_string(),
        }
    }
}

#[derive(Serialize, Deserialize, Debug, Clone)]
pub struct Database {
    name: String,
}

impl Database {
    fn new(name: &str) -> Self {
        Self {
            name: name.to_string(),
        }
    }
}
