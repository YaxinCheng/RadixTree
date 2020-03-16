use std::collections::VecDeque;

#[derive(Debug)]
pub enum Element<T> {
    Value {
        label: String,
        value: T,
        children: Vec<Element<T>>,
    },
    Node {
        label: String,
        children: Vec<Element<T>>,
    },
    Base {
        label: String,
        children: Vec<Element<T>>,
    },
}

macro_rules! unpack {
    ( $element: expr ) => {
        match $element {
            Element::Value {
                label,
                value,
                children,
            } => (label, Some(value), children),
            Element::Node { label, children } => (label, None, children),
            Element::Base { label, children } => (label, None, children),
        }
    };
}

impl<T> Element<T> {
    pub fn label(&self) -> &str {
        unpack!(self).0
    }

    pub fn set_label(self, label: String) -> Self {
        match self {
            Element::Value {
                label: _,
                value,
                children,
            } => Element::Value {
                label,
                value,
                children,
            },
            Element::Node { label: _, children } => Element::Node { label, children },
            Element::Base {
                label: _,
                children: _,
            } => panic!("Cannot set base"),
        }
    }

    pub fn children_mut(&mut self) -> &mut Vec<Element<T>> {
        unpack!(self).2
    }

    pub fn children(&self) -> &Vec<Element<T>> {
        unpack!(self).2
    }

    pub fn children_own(self) -> Vec<Element<T>> {
        unpack!(self).2
    }

    pub fn value(&self) -> Option<&T> {
        unpack!(self).1
    }

    pub fn value_mut(&mut self) -> Option<&mut T> {
        unpack!(self).1
    }

    pub fn is_node(&self) -> bool {
        match self {
            Element::Node {
                label: _,
                children: _,
            } => true,
            _ => false,
        }
    }

    /// Unpack element into label, value, and children
    pub fn unpack(self) -> (String, Option<T>, Vec<Element<T>>) {
        unpack!(self)
    }

    /// Collect all the descendant values with their labels
    pub fn collect_all_child_values(&self) -> Vec<(String, &T)> {
        // contains all the parent labels
        let mut labels = vec![self.label().to_owned()];
        let mut res = match self.value() {
            Some(value) => vec![(self.label().to_owned(), value)],
            None => vec![],
        };
        // for all children with value, pack the parent label with the child element
        let mut children = self
            .children()
            .into_iter()
            .map(|child| (labels.len() - 1, child))
            .collect::<VecDeque<_>>();
        while let Some((prefix_index, element)) = children.pop_front() {
            // if element is Value, get the value and joined label
            let label = format!("{}{}", labels[prefix_index], element.label());
            labels.push(label);
            let index = labels.len() - 1;
            if let Some(value) = element.value() {
                res.push((labels[index].to_owned(), value));
            }
            // update the label storage
            children.extend(element.children().into_iter().map(|child| (index, child)))
        }
        res
    }
}

#[cfg(test)]
mod element_tests {
    use crate::element::Element;

    fn get_test_example() -> Element<()> {
        // vec![ "in", "industry", "industrial", "industrialization", "india", "indian", ];
        Element::Base {
            label: "in".into(),
            children: vec![Element::Node {
                label: "d".into(),
                children: vec![
                    Element::Value {
                        label: "ustry".into(),
                        value: (),
                        children: vec![],
                    },
                    Element::Node {
                        label: "ustri".into(),
                        children: vec![Element::Value {
                            label: "al".into(),
                            value: (),
                            children: vec![Element::Value {
                                label: "ization".into(),
                                value: (),
                                children: vec![],
                            }],
                        }],
                    },
                    Element::Value {
                        label: "ia".into(),
                        value: (),
                        children: vec![Element::Value {
                            label: "n".into(),
                            value: (),
                            children: vec![],
                        }],
                    },
                ],
            }],
        }
    }

    #[test]
    fn test_collect_all_child_values() {
        let test_example = get_test_example();
        let res = test_example
            .collect_all_child_values()
            .into_iter()
            .map(|(label, _)| label)
            .collect::<Vec<_>>();
        let expected = vec![
            "industry",
            "india",
            "industrial",
            "indian",
            "industrialization",
        ]
        .into_iter()
        .map(String::from)
        .collect::<Vec<_>>();
        assert_eq!(res, expected)
    }
}
