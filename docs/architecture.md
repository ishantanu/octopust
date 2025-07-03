# Architecture Overview

## Project Structure

- `src/client.rs`: Main API client logic. Handles authentication, request dispatch.
- `src/models.rs`: Data types for API resources (e.g., Product).
- `src/error.rs`: Error types for the library.
- `src/api/`: Submodules for each endpoint/resource grouping.
- `examples/`: Example usage.
- `tests/`: Integration tests.
- `benchmark/`: Benchmarks and performance tests. TBD
- `.github/workflows/`: CI configuration.

## Design Philosophy

- Async-first using `tokio` and `reqwest`
- Idiomatic Rust error handling with `thiserror`
- Extensible API for future endpoints

## Extending

To add a new endpoint:
1. Add a method in `src/client.rs`.
2. Implement the endpoint logic in a new or existing file in `src/api/`.
3. Add corresponding data types to `src/models.rs`.

## References

- [Octopus Energy API Docs](https://developer.octopus.energy/rest/guides)