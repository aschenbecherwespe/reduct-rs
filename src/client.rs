use hyper::Client as HyperClient;
use hyper::body::HttpBody as _;
use hyper::Uri;
use std::str;

pub struct Client {
    url: Uri
}

impl Client {
    async fn info(self) -> Result<(), Box<dyn std::error::Error + Send + Sync>> {
        let http_client = HyperClient::new();
        let mut res = http_client.get(self.url).await?;
        let mut body = "".to_owned();
        while let Some(next) = res.data().await {
            let chunk = next?;
            body.push_str(str::from_utf8(&chunk)?);
        }
        println!("{}", body);
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
