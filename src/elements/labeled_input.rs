use yew::prelude::*;

#[derive(Clone, Properties, PartialEq)]
pub struct LabeledInputProps {
    pub label: String,
    pub id: String,
    pub value: i64,
    pub on_input: Callback<InputData>,
}

pub struct LabeledInput {
    link: ComponentLink<Self>,
    label: String,
    id: String,
    value: i64,
    on_input: Callback<InputData>,
}

pub enum Msg {
    Changed(InputData),
}

impl Component for LabeledInput {
    type Message = Msg;
    type Properties = LabeledInputProps;

    fn create(props: Self::Properties, link: ComponentLink<Self>) -> Self {
        let Self::Properties { label, id, value, on_input } = props;
        Self {
            link,
            label,
            id,
            value,
            on_input,
        }
    }

    fn change(&mut self, props: Self::Properties) -> ShouldRender {
        let Self::Properties { label, id, value, on_input } = props;
        self.label = label;
        self.id = id;
        self.value = value;
        self.on_input = on_input;
        true
    }

    fn update(&mut self, msg: Self::Message) -> ShouldRender {
        match msg {
            Msg::Changed(e) => self.on_input.emit(e),
        }
        false
    }

    fn view(&self) -> Html {
        html! {
            <div class="labeled-input">
                <label for=self.id.clone()>{ self.label.clone() }</label><br />
                <input
                    class="input-box"
                    id=self.id.clone()
                    type="number" 
                    value=self.value
                    oninput=self.link.callback(|e: InputData| Msg::Changed(e))
                />
            </div>
        }
    }
}