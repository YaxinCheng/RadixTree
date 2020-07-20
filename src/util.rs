use crate::element::Element;

pub fn binary_search<T>(target: char, array: &[Element<T>]) -> usize {
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

pub fn longest_shared_prefix<'a>(s1: &'a str, s2: &'a str) -> &'a str {
    for ((index1, char1), char2) in s1.char_indices().zip(s2.chars()) {
        if char1 != char2 {
            return &s1[..index1];
        }
    }
    return if s1.len() > s2.len() { s2 } else { s1 };
}

/// A helper function to create an value element
pub fn value_element<T, S: ToString>(label: S, value: T, children: Vec<Element<T>>) -> Element<T> {
    Element::Value {
        label: label.to_string(),
        value,
        children,
    }
}

pub fn first_char<S: AsRef<str>>(s: S) -> char {
    s.as_ref()
        .chars()
        .next()
        .expect("First char called on empty string")
}

#[cfg(test)]
mod util_tests {
    use crate::util;

    #[test]
    fn longest_shared_prefix_non_alphabetic_test() {
        let s1 = "Toronto多倫多";
        let s2 = "Toronto多伦多";
        let prefix = util::longest_shared_prefix(s1, s2);
        assert_eq!(prefix, "Toronto多");
    }
}
