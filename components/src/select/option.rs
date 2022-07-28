use yew::virtual_dom::VNode;

pub trait SelectOption {
    type Value: PartialEq;

    fn value(&self) -> Self::Value;
    fn label(&self) -> String;
    fn icon(&self) -> Option<VNode> {
        None
    }
}

impl<O> SelectOption for (O,)
where
    O: ToString + PartialEq + Clone,
{
    type Value = O;

    fn value(&self) -> Self::Value {
        self.0.clone()
    }

    fn label(&self) -> String {
        self.0.to_string()
    }
}

impl<O> SelectOption for (O, String)
where
    O: PartialEq + Clone,
{
    type Value = O;

    fn value(&self) -> Self::Value {
        self.0.clone()
    }

    fn label(&self) -> String {
        self.1.to_string()
    }
}

impl<O> SelectOption for (O, String, VNode)
where
    O: PartialEq + Clone,
{
    type Value = O;

    fn value(&self) -> Self::Value {
        self.0.clone()
    }

    fn label(&self) -> String {
        self.1.to_string()
    }

    fn icon(&self) -> Option<VNode> {
        Some(self.2.clone())
    }
}
