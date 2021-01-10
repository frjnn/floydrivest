# floydrivest

[![License: MIT](https://img.shields.io/badge/License-MIT-yellow.svg)](https://opensource.org/licenses/MIT)
[![frjnn](https://circleci.com/gh/frjnn/floydrivest.svg?style=shield)](https://app.circleci.com/pipelines/github/frjnn/floydrivest)
[![codecov](https://codecov.io/gh/frjnn/floydrivest/branch/master/graph/badge.svg)](https://codecov.io/gh/frjnn/floydrivest)

A lightweight crate that brings `nth_element` to Rust. Available on [crates.io](https://crates.io/crates/floydrivest).

## Installation 

Be sure that your `Cargo.toml` looks somewhat like this:
```toml
[dependencies]
floydrivest = "0.2.2"
```
## Usage

Bring the crate into scope:

```rust
extern crate floydrivest;

use floydrivest::nth_element;
```
then  simply call `nth_element` on a vector.

```rust
let mut v = vec![10, 7, 9, 7, 2, 8, 8, 1, 9, 4];
nth_element(&mut v, 3, &mut Ord::cmp);

assert_eq!(v[3], 7);
```

This implementation also handles generic data types as long as they satisfy the `PartialEq` and `PartialOrd` traits.

## Implementation

Link to the [original paper](http://citeseerx.ist.psu.edu/viewdoc/download?doi=10.1.1.309.7108&rep=rep1&type=pdf) with ALGOL60 pseudocode.

## Performance

Floyd and Rivest’s algorithm for finding the k-th smallest of n elements requires at most __n + min(k, n−k) + o(n)__ comparisons on average and with high probability. Here's the link to [another paper](https://pdf.sciencedirectassets.com/271538/1-s2.0-S0304397505X06053/1-s2.0-S0304397505004081/main.pdf?X-Amz-Security-Token=IQoJb3JpZ2luX2VjEFAaCXVzLWVhc3QtMSJHMEUCIQDl%2F9cPuNXqSVgoeYJ99N9%2FR1fZEziJbYxXKIsoJ7FVTwIgYGDaDtCQwiGSl0PU6cE5yOK3CV%2Bm8CgqJhONLCT%2BBtIqtAMIKRADGgwwNTkwMDM1NDY4NjUiDBlhPv3vVEGQm4qiJyqRA7aLGldU3%2BiSKo9b6xM2nAEyZjlHOjtF0ktwUnfdGsEgZzgO2IYnEb4z01M64Vhipoopsc5hd7pLyYJZuW2M9Gw291mK7IZz%2FC3PJjGs%2BNYgBJRm%2Fwm%2FKEsm7jWCgPv2oUK55xFO2ZXFPPcNFzauDNkqebUtlrj5lglM7f68PSIDKlPTbzH6V108p7E2dI%2BRBBRg6AZpZdSIYIXzv0miVkwjAUBIEyEz4%2B4E3JQGwhj2lKz5nfeA713w2u2WWH779Sg1Ti6zxAHmoZo5R3x7srDCJWvBEGevqBzqRKpeIZGzANiJKqUHNnXNt7mdcY4GU0KrC62wdRkmhky5n4ab1EqLL5m793GgIItVAggR7N5gE%2FoyYR7Oee6%2FZeibLwSoJyRZgZj7I%2FQkcADKUOZTyGRDwNKBlErZrhpNcloOSp3F2W6OpqMXRQvbFlXyamexfk11exiuddhQqyi4yLYsd6LvMcCyQFsqd98HNQWK4qbzhTcZa%2B2lZdU%2FJ8a9iWsI0Z9ujUmkrEo1TAkXWPmuSHw6MN%2FQvvkFOusBVthNwgsUEim4JYbkDmzbucbRCpecT34O148zcYrftPNWZ1M14gI6pL3Y7Jzf1jJoIDjlJamRqgKxQfEFuMKlmW%2Fc2D5MbFrgUla7ewXuPsIYaDknTA227PQGzmeao45aYktYnEP0nw41W%2FDTlPxmqFf9J35uwne4BSYZjmPRyiD3Svy6n9EA%2F2ZzJlDh9OuKHHMtsC5hNEWAvEQbcy15lBgLZhEf1ZylwC7PI8R7q6wPnK3C2OGinMYu2vuwS0cmMhrAB7C0IY0Xuu68fDGofPGp6pnFjnjq%2FNurq3%2BenRw1cPG2eYPKEXhWwg%3D%3D&X-Amz-Algorithm=AWS4-HMAC-SHA256&X-Amz-Date=20200809T075125Z&X-Amz-SignedHeaders=host&X-Amz-Expires=300&X-Amz-Credential=ASIAQ3PHCVTY6TADYCRC%2F20200809%2Fus-east-1%2Fs3%2Faws4_request&X-Amz-Signature=9d7ddacf9fa23c5dceebfc032db6f212dd18cea0f63e0b4693f63d7cf795bbf1&hash=fa70d2d7494148faece1fa7d317c6d39127465a0e9cbaa204fa1423f5fdf85a1&host=68042c943591013ac2b2430a89b270f6af2c76d8dfd086a07176afe7c76c2c61&pii=S0304397505004081&tid=spdf-eef374c0-5cab-40f8-be65-cde644439ebb&sid=23bf34414d6ab740a758d979eb54cef1d3adgxrqb&type=client1).

## Benchmarks

Here are some benchmarks against other crates: [order-stat](https://crates.io/crates/order-stat), [kth](https://crates.io/crates/kth) and [pdqlselect](https://crates.io/crates/pdqselect). Thanks so much to **ilyapopov**.

![benches](imgs/bench.png)
