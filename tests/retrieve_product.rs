use octopust::{models::RetrieveProductQuery, Client};

#[tokio::test]
async fn test_retrieve_product() {
    let api_key = std::env::var("OCTOPUS_API_KEY").unwrap_or_default();
    if api_key.is_empty() {
        eprintln!("OCTOPUS_API_KEY not set. Skipping test.");
        return;
    }
    let client = Client::new(api_key);
    let result = client.retrieve_product(RetrieveProductQuery { 
        product_code: "AGILE-24-10-01",
        ..Default::default()
     }).await;
    println!("{:?}", result);
    match result {
        Ok(products) => {
            println!("Products: {:?}", products);
        }
        Err(e) => {
            eprintln!("Error from retrieve_products: {:?}", e);
            panic!("get_products failed");
        }
    }
}