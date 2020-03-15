# RadixTree
Rust built radix tree with sorted vec

## Example

Construct
```rust
use self::radix_trie::RadixTrie;
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
