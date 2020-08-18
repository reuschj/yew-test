#![recursion_limit="512"]

#[cfg(test)]
mod tests {
    #[test]
    fn it_works() {
        assert_eq!(2 + 2, 4);
    }
}

use wasm_bindgen::prelude::*;
use yew::prelude::*;

struct Model {
    link: ComponentLink<Self>,
    value: i64,
    interval: i64,
    limited: bool,
    min: i64,
    max: i64,
}

enum Msg {
    Increment,
    Decrement,
    ToggleLimit,
    SetInterval(i64),
    SetMin(i64),
    SetMax(i64),
}

impl Model {
    fn increment(&mut self) {
        self.value += self.interval;
    }
    fn decrement(&mut self) {
        self.value -= self.interval;
    }
    fn can_increment(&self) -> bool {
        !self.limited || self.value + self.interval <= self.max
    }
    fn can_decrement(&self) -> bool {
        !self.limited || self.value - self.interval >= self.min
    }
    fn limit(&mut self) {
        if self.limited {
            if self.value > self.max {
                self.value = self.max;
            }
            if self.value < self.min {
                self.value = self.min;
            }
        }
    }
}

impl Component for Model {
    type Message = Msg;
    type Properties = ();
    fn create(_: Self::Properties, link: ComponentLink<Self>) -> Self {
        Self {
            link,
            value: 0,
            interval: 1,
            limited: false,
            min: 0,
            max: 100,
        }
    }

    fn update(&mut self, msg: Self::Message) -> ShouldRender {
        match msg {
            Msg::Increment => {
                if self.can_increment() { self.increment() }
            },
            Msg::Decrement => {
                if self.can_decrement() { self.decrement() }
            },
            Msg::ToggleLimit => {
                self.limited = !self.limited;
                self.limit();
            },
            Msg::SetInterval(new_interval) => {
                self.interval = new_interval;
            },
            Msg::SetMin(new_min) => {
                if new_min <= self.max {
                    self.min = new_min;
                }
                self.limit();
            },
            Msg::SetMax(new_max) => {
                if new_max >= self.min {
                    self.max = new_max;
                }
                self.limit();
            },
        }
        true
    }

    fn change(&mut self, _props: Self::Properties) -> ShouldRender {
        // Should only return "true" if new properties are different to
        // previously received properties.
        // This component has no properties so we will always return "false".
        false
    }

    fn view(&self) -> Html {
        let empty = html! { <></> };
        let min_max = html! {
            <div class="app-container boxed-section">
                <h4 class="section-title">{ "Limits" }</h4>
                <div class="labeled-input">
                    <label for="min-input">{ "Minimum" }</label>
                    <input
                        class="input-box"
                        id="min-input"
                        type="number" 
                        value=&self.min
                        oninput=self.link.callback(|e: InputData| Msg::SetMin(e.value.parse().unwrap()))
                    />
                </div>
                <div class="labeled-input">
                    <label for="max-input">{ "Maximum" }</label><br />
                    <input
                        class="input-box"
                        id="max-input"
                        type="number" 
                        value=&self.max
                        oninput=self.link.callback(|e: InputData| Msg::SetMax(e.value.parse().unwrap()))
                    />
                </div>
            </div>
        };
        let decrement_button = html! { 
            <button onclick=self.link.callback(|_| Msg::Decrement)>
                { format!("- {}", self.interval) }
            </button>
        };
        let increment_button = html! {
            <button onclick=self.link.callback(|_| Msg::Increment)>
                { format!("+ {}", self.interval) }
            </button>
        };
        html! {
            <div class="app">
                <div class="app-header">
                    <h2>{ "Increment Counter" }</h2>
                </div>
                <div class="app-body">
                    <div class="app-container counter-container">
                        <div class="counter-component">
                            { if self.can_decrement() { decrement_button } else { empty.clone() } }
                        </div>
                        <div class="counter-component">
                            <h1 class="current-value">{ format!("{}", self.value ) }</h1>
                        </div>
                        <div class="counter-component">
                            { if self.can_increment() { increment_button } else { empty.clone() } }
                        </div>
                    </div>
                    <div class="settings-blocks">
                        <div class="app-container boxed-section">
                            <h4 class="section-title">{ "Settings" }</h4>
                            <div class="labeled-input">
                                <label for="interval-input">{ "Interval" }</label><br />
                                <input
                                    class="input-box"
                                    id="interval-input"
                                    type="number" 
                                    value=&self.interval
                                    oninput=self.link.callback(|e: InputData| Msg::SetInterval(e.value.parse().unwrap()))
                                />
                            </div>
                            <div class="labeled-input">
                                <input
                                    class="checkbox"
                                    id="limit-toggle"
                                    type="checkbox" 
                                    checked=self.limited
                                    onclick=self.link.callback(|_| Msg::ToggleLimit)
                                />
                                <label for="limit-toggle">{ "Limit to range" }</label>
                            </div>
                        </div>
                        { if self.limited { min_max } else { empty.clone() } }
                    </div>
                </div>
            </div>
        }
    }
}

#[wasm_bindgen(start)]
pub fn run_app() {
    App::<Model>::new().mount_to_body();
}