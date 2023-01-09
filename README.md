# hop-rs

hop sdk rust

## Usage

Create a [project token](https://docs.hop.io/reference/project_tokens) or personal access token.

```rust
extern crate hop;
extern crate rand;

use hop::Hop;
use rand::Rng;

fn main() {
    let my_token = "ptk_xxx";
    let hop = Hop::new(my_token);

    // Example: Creating a project secret
    hop.projects.create_secret(
        "RANDOM_NUMBER",
        rand::thread_rng().gen_range(0, 100).to_string(),
    );
}
```

[//]: # (// let projects = hop.projects&#40;&#41;.list&#40;&#41;.unwrap&#40;&#41;;)
[//]: # (// println!&#40;"{:?}", projects&#41;;)
