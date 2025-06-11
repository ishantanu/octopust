use octopust::Client;

#[tokio::test]
 async fn test_list_products() {
    let api_key = std::env::var("OCTOPUS_API_KEY").unwrap_or_default();
    if api_key.is_empty() {
        eprintln!("OCTOPUS_API_KEY not set. Skipping test.");
        return;
    }
    let client = Client::new(api_key);
    let result = client.list_products().await;
    match result {
        Ok(products) => {
            println!("Products: {:?}", products);
        }
        Err(e) => {
            eprintln!("Error from get_products: {:?}", e);
            panic!("get_products failed");
        }
    }
}