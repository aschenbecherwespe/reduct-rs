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
pub struct BucketDefaults {
    max_block_size: String,
    max_block_records: String,
    quota_type: Quota,
    quota_size: String
}

#[derive(Serialize, Deserialize, Debug)]
pub struct Bucket {
    bucket: BucketDefaults
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
    url: Uri
}

impl Client {
    async fn info(self) -> Result<ServerInfo, Box<dyn std::error::Error + Send + Sync>> {
        let http_client = HyperClient::new();
        let res = http_client.get(self.url).await?;
        let body = hyper::body::to_bytes(res).await?;
        println!("{}", str::from_utf8(&body)?);
        let server: ServerInfo = serde_json::from_slice(&body).unwrap();
        Ok(server)
    }
}

#[cfg(test)]
mod tests {
    use super::*;
    #[tokio::test]
    async fn test_info() {
        let client = Client { url: Uri::from_static("http://localhost:8383/info") };
        let res = client.info().await;
    }
}
