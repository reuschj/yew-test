use yew::prelude::*;
use yew::html::{ ChildrenRenderer };
use yew::virtual_dom::{ VChild, VComp, VNode };
use crate::elements::labeled_input::{ LabeledInput, LabeledInputProps };
use crate::elements::labeled_checkbox::{ LabeledCheckbox, LabeledCheckboxProps };

// Variant enum ----------------------------------- /

/// All types of components that can be children of a settings block.
#[derive(Clone, PartialEq)]
pub enum Variant {
    Input(<LabeledInput as Component>::Properties),
    Checkbox(<LabeledCheckbox as Component>::Properties),
}

impl From<LabeledInputProps> for Variant {

    /// Creates a `Variant` from `LabeledInputProps`.
    fn from(props: LabeledInputProps) -> Self {
        Variant::Input(props)
    }
}

impl From<LabeledCheckboxProps> for Variant {

    /// Creates a `Variant` from `LabeledCheckboxProps`.
    fn from(props: LabeledCheckboxProps) -> Self {
        Variant::Checkbox(props)
    }
}

// Settings block item ----------------------------------- /

/// A wrapper struct for a settings block component.
#[derive(PartialEq, Clone)]
pub struct SettingsBlockItem {
    props: Variant,
}

impl<C> From<VChild<C>> for SettingsBlockItem where C: Component, C::Properties: Into<Variant> {
    
    /// Creates a `SettingsBlockItem` from a virtual child.
    fn from(v_child: VChild<C>) -> Self {
        SettingsBlockItem {
            props: v_child.props.into(),
        }
    }
}
impl Into<VNode> for SettingsBlockItem {

    /// Creates a virtual node from a `SettingsBlockItem`.
    fn into(self) -> VNode {
        match self.props {
            Variant::Input(props) => {
                VComp::new::<LabeledInput>(props, NodeRef::default(), None).into()
            },
            Variant::Checkbox(props) => {
                VComp::new::<LabeledCheckbox>(props, NodeRef::default(), None).into()
            },
        }
    }
}

// Settings block (props) ----------------------------------- /

/// Props for the settings block.
#[derive(Clone, Properties)]
pub struct SettingsBlockProps {
    pub title: String,
    pub children: ChildrenRenderer<SettingsBlockItem>,
}

// Settings block (component) ----------------------------------- /

/// A wrapper component for a block of app settings (labeled inputs and/or checkboxes).
pub struct SettingsBlock(SettingsBlockProps);

impl Component for SettingsBlock {
    type Message = ();
    type Properties = SettingsBlockProps;

    fn create(props: Self::Properties, _link: ComponentLink<Self>) -> Self {
        Self(props)
    }

    fn change(&mut self, props: Self::Properties) -> ShouldRender {
        self.0 = props;
        true
    }

    fn update(&mut self, _msg: Self::Message) -> ShouldRender {
        false
    }

    fn view(&self) -> Html {
        html! {
            <div class="app-container boxed-section">
                { self.make_title() }
                { self.make_settings() }
            </div>
        }
    }
}

impl SettingsBlock {

    /// Makes the UI title HTML for the settings block.
    fn make_title(&self) -> Html {
        html! { <h4 class="section-title">{ self.0.title.clone() }</h4> }
    }

    /// Makes the HTML for all content below the title (input and checkbox children).
    fn make_settings(&self) -> Html {
        html! {
            for self.0.children.iter().map(|child_component| {
                child_component
            })
        }
    }
}
