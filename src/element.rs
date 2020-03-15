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
        match self {
            Element::Value {
                label: _,
                value: _,
                children,
            }
            | Element::Node { label: _, children } => children,
        }
    }

    pub fn children(&self) -> &Vec<Element<T>> {
        match self {
            Element::Value {
                label: _,
                value: _,
                children,
            }
            | Element::Node { label: _, children } => children,
        }
    }

    pub fn value(&self) -> Option<&T> {
        match self {
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
    }
}
