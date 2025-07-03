use octopust::{models::ProductQuery, Client};

#[tokio::main]
async fn main() {
    let client = Client::new("YOUR_API_KEY");
    match client.list_products(ProductQuery {
        ..Default::default()
    }).await {
        Ok(products) => {
            for product in products {
                println!("{:?}", product);
            }
        }
        Err(e) => eprintln!("Error: {:?}", e),
    }
}