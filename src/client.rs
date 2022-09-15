use hyper::Client as HyperClient;
use hyper::Uri;
use std::str;


pub struct Client {
    url: Uri
}

impl Client {
    async fn info(self) -> Result<(), Box<dyn std::error::Error + Send + Sync>> {
        let http_client = HyperClient::new();
        let res = http_client.get(self.url).await?;
        let body = hyper::body::to_bytes(res).await?;
        println!("{}", str::from_utf8(&body)?);
        Ok(())
    }
}

#[cfg(test)]
mod tests {
    use super::*;
    #[tokio::test]
    async fn test_info() {
        let client = Client { url: Uri::from_static("http://localhost:8383/info") };
        let _res = client.info().await;
    }
}
