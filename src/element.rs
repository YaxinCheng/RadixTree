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
}

macro_rules! children {
    ( $element: expr ) => {
        match $element {
            Element::Value {
                label: _,
                value: _,
                children,
            }
            | Element::Node { label: _, children } => children,
        }
    };
}

macro_rules! value {
    ( $element: expr ) => {
        match $element {
            Element::Value {
                label: _,
                value,
                children: _,
            } => Some(value),
            Element::Node {
                label: _,
                children: _,
            } => None,
        }
    };
}

impl<T> Element<T> {
    pub fn label(&self) -> &str {
        match self {
            Element::Value {
                label,
                value: _,
                children: _,
            }
            | Element::Node { label, children: _ } => label,
        }
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
        }
    }

    pub fn children_mut(&mut self) -> &mut Vec<Element<T>> {
        children!(self)
    }

    pub fn children(&self) -> &Vec<Element<T>> {
        children!(self)
    }

    pub fn children_own(self) -> Vec<Element<T>> {
        children!(self)
    }

    pub fn value(&self) -> Option<&T> {
        value!(self)
    }

    pub fn unpack(self) -> (String, Option<T>, Vec<Element<T>>) {
        match self {
            Element::Value {
                label,
                value,
                children,
            } => (label, Some(value), children),
            Element::Node { label, children } => (label, None, children),
        }
    }

    pub fn collect_all_child_values(&self) -> Vec<(String, &T)> {
        // contains all the parent labels
        let mut labels = vec![self.label().to_owned()];
        let mut res = match value!(self) {
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
            if let Some(value) = value!(element) {
                res.push((labels[index].to_owned(), value));
            }
            // update the label storage
            children.extend(children!(element).into_iter().map(|child| (index, child)))
        }
        res
    }
}
