use yew::prelude::*;

#[derive(Clone, Properties)]
pub struct LabeledCheckboxProps {
    pub label: String,
    pub id: String,
    pub checked: bool,
    pub on_click: Callback<()>,
}

pub struct LabeledCheckbox {
    link: ComponentLink<Self>,
    label: String,
    id: String,
    checked: bool,
    on_click: Callback<()>,
}

pub enum Msg {
    Changed,
}

impl Component for LabeledCheckbox {
    type Message = Msg;
    type Properties = LabeledCheckboxProps;

    fn create(props: Self::Properties, link: ComponentLink<Self>) -> Self {
        let Self::Properties { label, id, checked, on_click } = props;
        Self {
            link,
            label,
            id,
            checked,
            on_click,
        }
    }

    fn change(&mut self, props: Self::Properties) -> ShouldRender {
        let Self::Properties { label, id, checked, on_click } = props;
        self.label = label;
        self.id = id;
        self.checked = checked;
        self.on_click = on_click;
        true
    }

    fn update(&mut self, msg: Self::Message) -> ShouldRender {
        match msg {
            Msg::Changed => self.on_click.emit(()),
        }
        false
    }

    fn view(&self) -> Html {
        html! {
            <div class="labeled-input">
                <label for=self.id.clone()>{ self.label.clone() }</label><br />
                <input
                    class="checkbox"
                    id=self.id.clone()
                    type="checkbox" 
                    checked=self.checked
                    oninput=self.link.callback(|_| Msg::Changed)
                />
            </div>
        }
    }
}