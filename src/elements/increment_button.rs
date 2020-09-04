use yew::prelude::*;

// Direction enum ----------------------------------- /

/// Enum of possible directions for an increment button to increment.
/// All directions take a parameter for the magnitude of the increment.
#[derive(Clone)]
pub enum Direction {
    Up(i64),
    Down(i64),
}

impl Default for Direction {

    /// By default, increment up by 1.
    fn default() -> Self {
        Direction::Up(1)
    }
}

// Increment button (props) ----------------------------------- /

/// Props for the increment button.
#[derive(Clone, Properties)]
pub struct IncrementButtonProps {
    #[prop_or_default]
    pub direction: Direction,
    pub on_click: Callback<()>,
}

// Increment button (component) ----------------------------------- /

// A button and increment by a specified direction and magnitude on press.
pub struct IncrementButton {
    link: ComponentLink<Self>,
    direction: Direction,
    on_click: Callback<()>,
}

impl IncrementButton {

    /// Gets the text label for the button.
    fn button_label(&self) -> String {
        match self.direction {
            Direction::Up(interval) => format!("+ {}", interval),
            Direction::Down(interval) => format!("- {}", interval),
        }
    }
}

/// Enum of possible messages to send to an increment button.
pub enum Msg {
    Clicked,
}

impl Component for IncrementButton {
    type Message = Msg;
    type Properties = IncrementButtonProps;

    fn create(props: Self::Properties, link: ComponentLink<Self>) -> Self {
        let Self::Properties { direction, on_click } = props;
        Self {
            link,
            direction,
            on_click,
        }
    }

    fn change(&mut self, props: Self::Properties) -> ShouldRender {
        let Self::Properties { direction, on_click } = props;
        self.direction = direction;
        self.on_click = on_click;
        true
    }

    fn update(&mut self, msg: Self::Message) -> ShouldRender {
        match msg {
            Msg::Clicked => self.on_click.emit(()),
        }
        false
    }

    fn view(&self) -> Html {
        html! {
            <button onclick=self.link.callback(|_| Msg::Clicked)>
                { self.button_label() }
            </button>
        }
    }
}