# another_radix_trie
Rust built radix tree with sorted vec

[![crate](https://img.shields.io/badge/crates.io-0.1.4-orange)](https://crates.io/crates/another_radix_trie)
[![doc](https://img.shields.io/badge/docs-0.1.4-blue)](https://docs.rs/another_radix_trie/0.1.4/another_radix_trie/)
![build](https://img.shields.io/badge/build-passing-success)
![licence](https://img.shields.io/badge/licence-MIT-informational)

## Example

Construct
```rust
use another_radix_trie::RadixTrie;
let mut trie = RadixTrie::<String>::new();
```

Insert
```rust
trie.insert("label", String::from("value"));
```

Find
```rust
trie.find("label");
// returns Some(&"value")
```

Find_mut
```rust
trie.find_mut("label");
// returns Some(&mut "value")
```

Remove
```rust
trie.remove("label");
// returns Some("value")
```

Start with
```rust
trie.insert("lab", "laboratory");
trie.insert("label", "label");
trie.starts_with("la");
// returns vec![("lab", &"laboratory"), ("label", &"label")]
```
