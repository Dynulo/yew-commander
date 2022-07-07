use yew::virtual_dom::VNode;

pub trait MenuEntry {
    type Value: PartialEq;

    fn value(&self) -> Self::Value;
    fn label(&self) -> String;
    fn icon(&self) -> Option<VNode> {
        None
    }
}

impl<O> MenuEntry for (O,)
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

impl<O> MenuEntry for (O, String)
where
    O: ToString + PartialEq + Clone,
{
    type Value = O;

    fn value(&self) -> Self::Value {
        self.0.clone()
    }

    fn label(&self) -> String {
        self.1.to_string()
    }
}

impl<O> MenuEntry for (O, String, VNode)
where
    O: ToString + PartialEq + Clone,
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
