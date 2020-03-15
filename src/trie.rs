use crate::element::Element;
use crate::util;

/// RadixTrie stores values associated with strings
///
/// # Example
/// ```rust
/// use self::radix_trie::RadixTrie;
/// let mut trie = RadixTrie::<usize>::new();
/// trie.insert("ON", 3);
/// trie.insert("ON20", 4)
/// // The internal structure of this trie will be
/// // - "ON" 3
/// //    - "20" 4
/// ```
pub struct RadixTrie<T> {
    entry: Vec<Element<T>>,
}

impl<T> RadixTrie<T> {
    pub fn new() -> Self {
        RadixTrie { entry: Vec::new() }
    }

    pub fn insert(&mut self, mut label: &str, value: T) {
        let mut entry = &mut self.entry;
        while label.len() > 0 {
            let label_init_char = label.chars().next().unwrap();
            let target_index = util::binary_search(label_init_char, &entry);
            if target_index >= entry.len() {
                return entry.push(util::element_new_value(label, value, vec![]));
            }
            let target = &entry[target_index];
            let shared_prefix = util::longest_shared_prefix(target.label(), label);
            if shared_prefix.is_empty() {
                // no shared prefix, insert directly
                return entry.insert(target_index, util::element_new_value(label, value, vec![]));
            } else if shared_prefix == label {
                let children = match target.label() == label {
                    true => entry.remove(target_index).children_own(), // new value to replace the old value. Inherits the old one's children
                    false => vec![entry.remove(target_index)], // add the old value as a child
                };
                let item = util::element_new_value(label, value, children);
                return entry.insert(target_index, item);
            } else if shared_prefix == target.label() {
                // existing one is the prefix
                label = &label[shared_prefix.len()..]; // search the parts after the shared prefix
                entry = (&mut entry[target_index]).children_mut();
            } else {
                // The existing and newly adding one intersect
                let shared_common = shared_prefix.to_owned();
                let joined_item = Self::join_intersected_nodes(
                    entry.remove(target_index),
                    util::element_new_value(&label[shared_common.len()..], value, vec![]),
                    shared_common,
                );
                return entry.insert(target_index, joined_item);
            }
        }
    }

    /// When two nodes have intersected labels, call this helper to process
    fn join_intersected_nodes(
        original: Element<T>,
        new: Element<T>,
        shared_common: String,
    ) -> Element<T> {
        let new_original_label = original.label()[shared_common.len()..].to_owned();
        let original_item = original.set_label(new_original_label);
        let mut children = vec![original_item, new];
        children.sort_by(|e1, e2| e1.label().cmp(e2.label()));
        Element::Node {
            label: shared_common,
            children,
        }
    }

    /// Returns the value associated with related label
    pub fn find(&self, mut label: &str) -> Option<&T> {
        let mut entry = &self.entry;
        while label.len() > 0 {
            let target_index = util::binary_search(label.chars().next().unwrap(), &entry);
            if target_index >= entry.len() {
                break;
            }
            let target = &entry[target_index];
            if target.label() == label {
                // found label
                return target.value();
            } else if label.starts_with(target.label()) {
                // existing_label matches the prefix of label. Move to next node
                label = &label[target.label().len()..];
                entry = target.children();
            } else {
                // not matched
                break;
            }
        }
        None
    }

    /// Removes the value associated with related label
    pub fn remove(&mut self, mut label: &str) -> Option<T> {
        let mut entry = &mut self.entry;
        while label.len() > 0 {
            let target_index = util::binary_search(label.chars().next().unwrap(), &entry);
            if target_index >= entry.len() {
                break;
            }
            let target = &entry[target_index];
            if target.label() == label {
                // existing_label matches label
                let (label, value, mut children) = entry.remove(target_index).unpack();
                if children.len() > 1 {
                    // target node has more than one children. Make target node a none value node
                    entry.insert(target_index, Element::Node { label, children });
                } else if children.len() == 1 {
                    // Only one child. Make the child parent
                    let child = children.pop().unwrap();
                    // Connect parent prefix with the child label
                    let child_label_prepend_parent_prefix = format!("{}{}", label, child.label());
                    entry.insert(
                        target_index,
                        child.set_label(child_label_prepend_parent_prefix),
                    );
                }
                return value;
            } else if label.starts_with(target.label()) {
                label = &label[target.label().len()..];
                entry = (&mut entry[target_index]).children_mut();
            } else {
                break;
            }
        }
        None
    }

    /// Returns all values with their labels where the labels have given prefix
    pub fn start_with(&self, mut prefix: &str) -> Vec<(String, &T)> {
        let mut entry = &self.entry;
        let mut prefixes: Vec<&str> = vec![];
        while prefix.len() > 0 {
            let target_index = util::binary_search(prefix.chars().next().unwrap(), &entry);
            if target_index >= entry.len() {
                break;
            }
            let target = &entry[target_index];
            if target.label().starts_with(prefix) {
                // found label
                let existing_prefix: String = prefixes.join("");
                return target
                    .collect_all_child_values()
                    .into_iter()
                    .map(|(prefix, value)| (format!("{}{}", existing_prefix, prefix), value))
                    .collect();
            } else if prefix.starts_with(target.label()) {
                // existing_label matches the prefix of label. Move to next node
                prefixes.push(target.label());
                prefix = &prefix[target.label().len()..];
                entry = target.children();
            } else {
                // not matched
                break;
            }
        }
        vec![]
    }
}

#[cfg(test)]
mod trie_tests {
    use crate::trie::RadixTrie;

    #[test]
    fn test_insert_find_remove() {
        let mut trie = RadixTrie::<usize>::new();
        trie.insert("ON", 647);
        trie.insert("ON2", 416);
        assert_eq!(trie.find("ON"), Some(&647));
        assert_eq!(trie.find("ON2"), Some(&416));
        assert_eq!(trie.find("NS"), None);
        assert_eq!(trie.remove("ON"), Some(647));
        assert_eq!(trie.remove("ON2"), Some(416));
        assert_eq!(trie.remove("NS"), None);
    }

    #[test]
    fn test_insert_find_remove_longer() {
        let mut trie = RadixTrie::<usize>::new();
        let words = ["Won", "Wonder", "Wonderful", "World", "Axes"];
        for word in &words {
            trie.insert(word, word.len())
        }
        for word in &words {
            assert_eq!(trie.find(word), Some(&word.len()));
            assert_eq!(trie.remove(word), Some(word.len()));
        }
    }

    #[test]
    fn test_start_with() {
        let mut trie = RadixTrie::<usize>::new();
        let words = ["Won", "Wonder", "Wonderful", "World", "Axes"];
        for word in &words {
            trie.insert(word, word.len())
        }
        let res = trie.start_with("W");
        let expected: Vec<(String, &usize)> = vec![
            ("Won".into(), &3),
            ("World".into(), &5),
            ("Wonder".into(), &6),
            ("Wonderful".into(), &9),
        ];
        assert_eq!(res, expected)
    }

    #[test]
    fn test_start_with_won() {
        let mut trie = RadixTrie::<usize>::new();
        let words = ["Won", "Wonder", "Wonderful", "World", "Axes"];
        for word in &words {
            trie.insert(word, word.len())
        }
        let res = trie.start_with("Won");
        let expected: Vec<(String, &usize)> = vec![
            ("Won".into(), &3),
            ("Wonder".into(), &6),
            ("Wonderful".into(), &9),
        ];
        assert_eq!(res, expected)
    }
}
