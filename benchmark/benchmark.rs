// Basic benchmark placeholder

// To use criterion for benchmarking, add it to [dev-dependencies] in Cargo.toml
// criterion = "0.5"
//
// Then uncomment the code below

// use criterion::{criterion_group, criterion_main, Criterion};
// use octopust::Client;

// pub fn criterion_benchmark(c: &mut Criterion) {
//     c.bench_function("get_products", |b| {
//         b.to_async(tokio::runtime::Runtime::new().unwrap())
//             .iter(|| async {
//                 let client = Client::new("YOUR_API_KEY");
//                 let _ = client.get_products().await;
//             })
//     });
// }

// criterion_group!(benches, criterion_benchmark);
// criterion_main!(benches);