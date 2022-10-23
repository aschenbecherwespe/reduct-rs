use serde::{Deserialize, Serialize};
use serde_json;
use hyper::Client as HyperClient;
use hyper::Uri;
use std::str;


#[derive(Serialize, Deserialize, Debug)]
pub enum Quota {
    NONE,
    FIFO,
}

#[derive(Serialize, Deserialize, Debug)]
pub struct BucketInfo {
    max_block_size: String,
    max_block_records: String,
    quota_type: Quota,
    quota_size: String
}

#[derive(Serialize, Deserialize, Debug)]
pub struct Bucket {
    bucket: BucketInfo
}


#[derive(Serialize, Deserialize, Debug)]
pub struct ServerInfo {
    version: String,
    bucket_count: String,
    usage: String,
    uptime: String,
    oldest_record: String,
    latest_record: String,
    defaults: Bucket
}


pub struct Client {
    pub url: String
}

impl Client {
    pub async fn info(self) -> Result<ServerInfo, Box<dyn std::error::Error + Send + Sync>> {
        let http_client = HyperClient::new();
        let path = "/api/v1/info";
        let info_url_str  = self.url.to_owned().clone() + path; 
        let info_uri = info_url_str.parse::<Uri>().unwrap();
        let res = http_client.get(info_uri).await?;
        let body = hyper::body::to_bytes(res).await?;
        // println!("{}", str::from_utf8(&body)?);
        let server: ServerInfo = serde_json::from_slice(&body).unwrap();
        Ok(server)
    }

    pub async fn list(self) -> Result<Vec<BucketInfo>, Box<dyn std::error::Error + Send + Sync>> {
        let http_client = HyperClient::new();
        let path = "/api/v1/list";
        let info_url_str  = self.url.to_owned().clone() + path; 
        let info_uri = info_url_str.parse::<Uri>().unwrap();
        let res = http_client.get(info_uri).await?;
        let body = hyper::body::to_bytes(res).await?;
        println!("{}", str::from_utf8(&body)?);
        let buckets: Vec<BucketInfo> = serde_json::from_slice(&body).unwrap();
        Ok(buckets)
    }
}
