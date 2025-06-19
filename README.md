# octopust

[![rust-clippy analyze](https://github.com/ishantanu/octopust/actions/workflows/rust_clippy.yml/badge.svg)](https://github.com/ishantanu/octopust/actions/workflows/rust_clippy.yml)

**octopust** is an idiomatic, high-performance Rust client for the [Octopus Energy API](https://developer.octopus.energy/rest/guides).  
Built for speed, safety, and ergonomics, it helps you interact with Octopus Energy's Kraken platform with ease. 🐙⚡

---

## Features

- Async/await support for high performance
- Strongly-typed request and response models using Serde
- Simple and extensible API client
- Covers core Octopus Energy endpoints (products, accounts, consumption and industry)
- Comprehensive error handling
- Well-documented and tested

## Supported APIs

octopust currently supports the following public Octopus Energy API endpoints:

- **Products**: List available electricity and gas products/tariffs.
- **Tariffs & Rates**: Retrieve tariff unit rates and standing charges.
- **Consumption**: Fetch electricity and gas consumption data by MPAN/MPRN.
- **Industry Reference Data**: Get grid supply points (GSPs).

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
    octopust = "0.1.0"
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