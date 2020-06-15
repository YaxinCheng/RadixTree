use crate::element::Element;

pub fn binary_search<T>(target: char, array: &Vec<Element<T>>) -> usize {
    let mut first = 0;
    let mut last = array.len();
    while first < last {
        let mid = first + (last - first) / 2;
        let mid_val = array[mid].label();
        if mid_val.chars().next().unwrap() < target {
            first = mid + 1;
        } else {
            last = mid;
        }
    }
    first
}

pub fn longest_shared_prefix(s1: &str, s2: &str) -> String {
    let mut shared = String::new();
    for (char1, char2) in s1.chars().zip(s2.chars()) {
        if char1 != char2 {
            return shared;
        } else {
            shared.push(char1);
        }
    }
    return (if s1.len() > s2.len() { s2 } else { s1 }).to_owned();
}

/// A helper function to create an value element
pub fn element_new_value<T, S: ToString>(
    label: S,
    value: T,
    children: Vec<Element<T>>,
) -> Element<T> {
    Element::Value {
        label: label.to_string(),
        value,
        children,
    }
}
