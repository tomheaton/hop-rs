# hop-rs (wip)

hop sdk rust

## Installation

```toml
[dependencies]
hop = "0.0.0"
```

## Usage

Create a [project token](https://docs.hop.io/reference/project_tokens) or personal access token.

```rust
extern crate hop;
extern crate rand;

use hop::Hop;
use rand::Rng;

#[tokio::main]
async fn main() {
    let my_token = "ptk_xxx";
    let hop = Hop::new(my_token);

    // Example: Creating a project secret
    hop.projects.create_secret(
        "RANDOM_NUMBER",
        rand::thread_rng().gen_range(0..100).to_string(),
    ).await.unwrap();
}
```

[//]: # (// let projects = hop.projects&#40;&#41;.list&#40;&#41;.unwrap&#40;&#41;;)
[//]: # (// println!&#40;"{:?}", projects&#41;;)

## Examples

To run examples, add a Personal token and a Project token to the `.env` file.

```bash
cargo run --example <example_name>
```

Examples can be found [here](./examples).
