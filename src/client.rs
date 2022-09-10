use url::{ Url };
use hyper::Client as HyperClient;
use tokio::io::{self, AsyncWriteExt as _};

pub struct Client {
    url: Url
}

impl Client {
    async fn info(self) {
        let client = HyperClient::new();
        let res = client.get(self.url).await?;
        while let Some(next) = res.data().await {
            let chunk = next?;
            io::stdout().write_all(&chunk).await?;
        }
    }
}


