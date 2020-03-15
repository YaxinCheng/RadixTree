use crate::element::Element;
use crate::util;

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
            let shared_common = util::longest_shared_prefix(entry[target_index].label(), label);
            if shared_common.is_empty() {
                return entry.insert(target_index, util::element_new_value(label, value, vec![]));
            } else if shared_common == label {
                let item = util::element_new_value(label, value, vec![entry.remove(target_index)]);
                return entry.insert(target_index, item);
            } else if shared_common == entry[target_index].label() {
                label = &label[shared_common.len()..];
                entry = (&mut entry[target_index]).children_mut();
            } else {
                let shared_common = shared_common.to_owned();
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

    pub fn find(&self, mut label: &str) -> Option<&T> {
        let mut entry = &self.entry;
        while label.len() > 0 {
            let target_index = util::binary_search(label.chars().next().unwrap(), &entry);
            if target_index >= entry.len() {
                return None;
            }
            let existing_label = entry[target_index].label();
            if existing_label.starts_with(label) {
                return entry[target_index].value();
            } else if label.starts_with(existing_label) {
                label = &label[entry[target_index].label().len()..];
                entry = (&entry[target_index]).children();
            } else {
                break;
            }
        }
        None
    }
}

#[cfg(test)]
mod trie_tests {
    use crate::trie::RadixTrie;

    #[test]
    fn test_insert_find() {
        let mut trie = RadixTrie::<usize>::new();
        trie.insert("ON", 647);
        trie.insert("ON2", 416);
        assert_eq!(trie.find("ON"), Some(&647));
        assert_eq!(trie.find("ON2"), Some(&416));
        assert_eq!(trie.find("NS"), None);
    }

    #[test]
    fn test_insert_find_longer() {
        let mut trie = RadixTrie::<usize>::new();
        let words = ["Won", "Wonder", "Wonderful", "World", "Axes"];
        for word in &words {
            trie.insert(word, word.len())
        }
        for word in &words {
            assert_eq!(trie.find(word), Some(&word.len()))
        }
    }
}
