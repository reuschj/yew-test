use yew::prelude::*;

#[derive(Clone)]
pub enum Direction {
    Up(i64),
    Down(i64),
}

impl Default for Direction {
    fn default() -> Self {
        Direction::Up(1)
    }
}

#[derive(Clone, Properties)]
pub struct IncrementButtonProps {
    #[prop_or_default]
    pub direction: Direction,
    pub on_click: Callback<()>,
}

pub struct IncrementButton {
    link: ComponentLink<Self>,
    direction: Direction,
    on_click: Callback<()>,
}

impl IncrementButton {
    fn button_label(&self) -> String {
        match self.direction {
            Direction::Up(interval) => format!("+ {}", interval),
            Direction::Down(interval) => format!("- {}", interval),
        }
    }
}

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