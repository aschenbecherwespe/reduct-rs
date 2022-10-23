#[cfg(test)]
mod tests {
    use reduct_rs::client::Client;
    #[tokio::test]
    async fn test_info() {
        let client = Client { url: "http://localhost:8383".to_string() };
        let res = client.info().await.unwrap();
        assert_ne!(format!("{:?}", res), "hello");
    }
}
