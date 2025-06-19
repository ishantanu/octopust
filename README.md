# octopust

**octopust** is an idiomatic, high-performance Rust client for the [Octopus Energy API](https://developer.octopus.energy/rest/guides).  
Built for speed, safety, and ergonomics, it helps you interact with Octopus Energy's Kraken platform with ease. 🐙⚡

*Note: This is currently in active development. This maybe unusable until basic foundations are complete.*
---

## Features

- Async/await support for high performance
- Strongly-typed request and response models using Serde
- Simple and extensible API client
- Covers core Octopus Energy endpoints (products, accounts, consumption and industry)
- Comprehensive error handling
- Well-documented and tested

## Example

```rust
use octopust::Client;

#[tokio::main]
async fn main() {
    let client = Client::new("YOUR_API_KEY");
    match client.list_products().await {
        Ok(products) => println!("Products: {:?}", products),
        Err(e) => eprintln!("Error: {:?}", e),
    }
}
```

## Getting Started

1. Add to your `Cargo.toml`:
    ```toml
    octopust = "0.1"
    ```

2. See the [examples/](examples/) directory for more usage patterns.

## Documentation

- [API Reference (docs.rs)](https://docs.rs/octopust/)
- [Official Octopus Energy API Docs](https://developer.octopus.energy/rest/guides)

## Contributing

Contributions are welcome! Please open issues or PRs for bug reports, feature requests, or documentation improvements.

## License

MIT

---

*octopust* is not affiliated with Octopus Energy.  
Logo and trademark rights belong to their respective owners.