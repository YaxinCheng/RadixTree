use self::FindOutcome::*;
use crate::element::Element;
use crate::util;

/// RadixTrie stores values associated with strings
///
/// # Example
/// ```rust
/// use another_radix_trie::RadixTrie;
/// let mut trie = RadixTrie::<usize>::new();
/// trie.insert("ON", 3);
/// trie.insert("ON20", 4)
/// // The internal structure of this trie will be
/// // - "ON" 3
/// //    - "20" 4
/// ```
pub struct RadixTrie<T> {
    entry: Element<T>,
}

/// Outcome of a searching with a given label against an entry
enum FindOutcome {
    /// The given label matches the label of an element.
    /// The index of that element is included
    /// Example:
    /// - given: 'label', find: 'label'
    ExactMatch(usize),
    /// An element has a label that is also a prefix for the given label.
    /// The index of that element is included
    /// Example:
    /// - given: 'label', find: 'lab'
    PrefixMatch(usize),
    /// The given label is a prefix of an element's label
    /// The index of that element is included
    /// Example:
    /// - given: 'lab', find: 'label'
    AsPrefixOf(usize),
    /// The given label is not a match to an element
    /// The expected index for the given label is returned
    /// Example:
    /// - given: 'label', find 'lazy' (intersects but not matched)
    /// - given: 'label', find 'fox' (not matched at all)
    NotMatch(usize),
    /// The given label is suppose to be the last element of the entry
    BeyondSizeLimit,
}

impl<T> RadixTrie<T> {
    /// Construct a new trie
    pub fn new() -> Self {
        RadixTrie {
            entry: Element::Base {
                label: "".to_owned(),
                children: vec![],
            },
        }
    }

    /// Insert label and associated value into the trie.
    /// Values will be override if the label provided is already in the trie
    /// # Example
    /// ```rust
    /// use another_radix_trie::RadixTrie;
    ///
    /// let mut trie = RadixTrie::<()>::new();
    /// trie.insert("label", ());
    /// ```
    pub fn insert(&mut self, mut label: &str, value: T) {
        let mut entry = (&mut self.entry).children_mut();
        while label.len() > 0 {
            match Self::find_from_entry(&entry, label) {
                BeyondSizeLimit => return entry.push(util::value_element(label, value, vec![])),
                AsPrefixOf(index) => return Self::insert_prefix_node(entry, index, label, value),
                NotMatch(index) => {
                    let target = &entry[index];
                    let shared_prefix = util::longest_shared_prefix(target.label(), label);
                    let merged_value = if shared_prefix.is_empty() {
                        // no shared prefix means no overlap with the existing value
                        util::value_element(label, value, vec![])
                    } else {
                        // creates a common parent node
                        let shared_prefix = shared_prefix.to_owned();
                        let origin = entry.remove(index);
                        let new_node =
                            util::value_element(&label[shared_prefix.len()..], value, vec![]);
                        Self::join_intersected_nodes(origin, new_node, shared_prefix)
                    };
                    return entry.insert(index, merged_value);
                }
                ExactMatch(index) => {
                    let target = &mut entry[index];
                    if let Some(old_value) = target.value_mut() {
                        // replace existing value
                        *old_value = value;
                    } else {
                        // if the existing one is Node, make it a Value
                        let children = entry.remove(index).children_own();
                        entry.insert(index, util::value_element(label, value, children));
                    }
                    return;
                }
                PrefixMatch(index) => {
                    let target = &mut entry[index];
                    label = &label[target.label().len()..];
                    entry = target.children_mut();
                }
            }
        }
    }

    fn insert_prefix_node(entry: &mut Vec<Element<T>>, index: usize, label: &str, value: T) {
        let mut origin = entry.remove(index);
        origin.remove_label_prefix(label.len());
        let new_value = util::value_element(label, value, vec![origin]);
        return entry.insert(index, new_value);
    }

    /// When two nodes have intersected labels, call this helper to process
    fn join_intersected_nodes(
        mut original: Element<T>,
        new: Element<T>,
        shared_prefix: String,
    ) -> Element<T> {
        original.remove_label_prefix(shared_prefix.len());
        let mut children = vec![original, new];
        children.sort_by(|e1, e2| e1.label().cmp(e2.label()));
        Element::Node {
            label: shared_prefix,
            children,
        }
    }

    /// Returns the borrowed value associated with related label.
    /// If the label does not exist in the
    /// # Example
    /// ```rust
    /// use another_radix_trie::RadixTrie;
    ///
    /// let mut trie = RadixTrie::<usize>::new();
    /// trie.insert("label", 5);
    /// assert_eq!(trie.find("label"), Some(&5));
    /// assert_eq!(trie.find("not exist"), None);
    /// ```
    pub fn find(&self, mut label: &str) -> Option<&T> {
        let mut entry = self.entry.children();
        while label.len() > 0 {
            match Self::find_from_entry(&entry, label) {
                NotMatch(_) | AsPrefixOf(_) | BeyondSizeLimit => break,
                PrefixMatch(target_index) => {
                    let target = &entry[target_index];
                    label = &label[target.label().len()..];
                    entry = target.children();
                }
                ExactMatch(target_index) => {
                    return entry[target_index].value();
                }
            }
        }
        None
    }

    /// Returns the mutable borrowed value associated with related label.
    /// If the label does not exist in the
    /// # Example
    /// ```rust
    /// use another_radix_trie::RadixTrie;
    ///
    /// let mut trie = RadixTrie::<usize>::new();
    /// trie.insert("label", 5);
    /// assert_eq!(trie.find_mut("label"), Some(&mut 5));
    /// assert_eq!(trie.find("not exist"), None);
    /// ```
    pub fn find_mut(&mut self, mut label: &str) -> Option<&mut T> {
        let mut entry = self.entry.children_mut();
        while label.len() > 0 {
            match Self::find_from_entry(&entry, label) {
                NotMatch(_) | AsPrefixOf(_) | BeyondSizeLimit => break,
                PrefixMatch(target_index) => {
                    let target = &mut entry[target_index];
                    label = &label[target.label().len()..];
                    entry = target.children_mut();
                }
                ExactMatch(target_index) => {
                    return entry[target_index].value_mut();
                }
            }
        }
        None
    }

    /// Removes the value associated with related label.
    /// If the provided label does not exist in the trie, return None
    /// # Example
    /// ```rust
    /// use another_radix_trie::RadixTrie;
    ///
    /// let mut trie = RadixTrie::<usize>::new();
    /// trie.insert("label", 5);
    /// assert_eq!(trie.remove("label"), Some(5));
    /// assert_eq!(trie.remove("not exist"), None);
    /// ```
    pub fn remove(&mut self, mut label: &str) -> Option<T> {
        let mut parent = &mut self.entry;
        while label.len() > 0 {
            match Self::find_from_entry(parent.children(), label) {
                BeyondSizeLimit | NotMatch(_) | AsPrefixOf(_) => break,
                ExactMatch(target_index) => {
                    let parent_is_node = parent.is_node();
                    let (label, value, mut children) =
                        parent.children_mut().remove(target_index).unpack();
                    if children.len() > 1 {
                        // target node has more than one children. Make target node a none value node
                        parent
                            .children_mut()
                            .insert(target_index, Element::Node { label, children });
                    } else if children.len() == 1 {
                        // Only one child. Make the child parent
                        let mut child = children.pop().unwrap();
                        child.add_label_prefix(label);
                        parent.children_mut().insert(target_index, child);
                    }
                    // if parent has only one node child and parent is node. Merge them
                    if parent.children().len() == 1 && parent_is_node {
                        let mut another_child = parent.children_mut().pop().unwrap();
                        another_child.add_label_prefix(parent.label());
                        *parent = another_child;
                    }
                    return value;
                }
                PrefixMatch(target_index) => {
                    let target = &parent.children()[target_index];
                    label = &label[target.label().len()..];
                    parent = &mut parent.children_mut()[target_index];
                }
            }
        }
        None
    }

    /// Returns all values with their labels where the labels start with given prefix
    /// # Example
    /// ```rust
    /// use another_radix_trie::RadixTrie;
    ///
    /// let mut trie = RadixTrie::<usize>::new();
    /// trie.insert("lab", 3);
    /// trie.insert("label", 5);
    /// assert_eq!(trie.start_with("la"), vec![(String::from("lab"), &3), (String::from("label"), &5)])
    /// ```
    pub fn start_with(&self, mut prefix: &str) -> Vec<(String, &T)> {
        let mut entry = self.entry.children();
        let mut prefixes: Vec<&str> = vec![];
        while prefix.len() > 0 {
            match Self::find_from_entry(entry, prefix) {
                BeyondSizeLimit | NotMatch(_) => break,
                PrefixMatch(target_index) => {
                    // existing_label matches the prefix of label. Move to next node
                    let target = &entry[target_index];
                    prefixes.push(target.label());
                    prefix = &prefix[target.label().len()..];
                    entry = target.children();
                }
                ExactMatch(target_index) | AsPrefixOf(target_index) => {
                    let existing_prefix: String = prefixes.join("");
                    return Self::format_children(&entry[target_index], &existing_prefix);
                }
            }
        }
        vec![]
    }

    fn format_children<'a>(entry: &'a Element<T>, prefix: &str) -> Vec<(String, &'a T)> {
        entry
            .collect_all_child_values()
            .into_iter()
            .map(|(label, value)| (format!("{}{}", prefix, label), value))
            .collect()
    }

    /// Run a binary search on the given entry and return outcome based on different conditions
    fn find_from_entry(entry: &[Element<T>], label: &str) -> FindOutcome {
        let char = util::first_char(label);
        let target_index = util::binary_search(char, entry);
        if target_index >= entry.len() {
            return BeyondSizeLimit;
        }
        let target = entry[target_index].label();
        if target == label {
            ExactMatch(target_index)
        } else if label.starts_with(target) {
            PrefixMatch(target_index)
        } else if target.starts_with(label) {
            AsPrefixOf(target_index)
        } else {
            NotMatch(target_index)
        }
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

    #[test]
    fn test_remove_with_merge_down() {
        let mut trie = RadixTrie::<usize>::new();
        trie.insert("exe", 3);
        trie.insert("execute", 7);
        trie.insert("exec", 4);
        trie.insert("example", 7);
        trie.remove("exec").expect("Removed exec");
        let cute = &trie.entry.children()[0].children()[1].children()[0];
        assert_eq!(cute.label(), "cute");
    }

    #[test]
    fn test_remove_with_merge_up() {
        let mut trie = RadixTrie::<usize>::new();
        trie.insert("exe", 3);
        trie.insert("execute", 7);
        trie.insert("exec", 4);
        trie.insert("example", 7);
        trie.remove("example").expect("Removed example");
        assert_eq!(trie.entry.children()[0].label(), "exe");
    }

    #[test]
    fn test_insert_find_mut() {
        let mut trie = RadixTrie::<usize>::new();
        trie.insert("ON", 647);
        let found = trie.find_mut("ON");
        assert_eq!(found, Some(&mut 647));
        *found.unwrap() = 416;
        let found = trie.find("ON");
        assert_eq!(found, Some(&416));
    }
}
