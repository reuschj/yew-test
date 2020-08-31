use yew::prelude::*;
use crate::containers::settings_block::{ SettingsBlock };
use crate::elements::increment_button::{ IncrementButton, Direction };
use crate::elements::labeled_input::{ LabeledInput };
use crate::elements::labeled_checkbox::{ LabeledCheckbox };
use crate::elements::empty::make_empty;

/// The component that renders the main screen of the app.
/// This component also stores top-level state and counter logic.
pub struct Home {
    link: ComponentLink<Self>,
    value: i64,
    interval: i64,
    limited: bool,
    min: i64,
    max: i64,
}

impl Home {

    /// Increments the stored value by the current interval.
    fn increment(&mut self) {
        self.value += self.interval;
    }

    /// Increments the stored value by the current interval.
    fn decrement(&mut self) {
        self.value -= self.interval;
    }

    /// Checks if the value is allowed to be incremented.
    fn can_increment(&self) -> bool {
        !self.limited || self.value + self.interval <= self.max
    }

    /// Checks if the value is allowed to be decremented.
    fn can_decrement(&self) -> bool {
        !self.limited || self.value - self.interval >= self.min
    }

    /// Limits the value to the minimum or maximum value (if limited at all).
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

/// Possible app action messages that can be handled by the home component.
pub enum Msg {
    /// Increments the value up (if possible) by the current interval.
    Increment,
    /// Decrements the value down (if possible) by the current interval.
    Decrement,
    /// Toggles the limiting setting (ability to set a min/max value) on/off.
    ToggleLimit,
    /// Sets the current counter interval for incrementing and decrementing.
    SetInterval(i64),
    /// Sets the minimum possible value (if limiting option is enabled).
    SetMin(i64),
    /// Sets the maximum possible value (if limiting option is enabled).
    SetMax(i64),
}

impl Component for Home {
    type Message = Msg;
    type Properties = ();

    fn create(_: Self::Properties, link: ComponentLink<Self>) -> Self {
        Home {
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
        let min_max = html! {
            <SettingsBlock title="Limits">
                <LabeledInput
                    label="Minimum"
                    id="min-input"
                    value=&self.min
                    on_input=self.link.callback(|e: InputData| Msg::SetMin(e.value.parse().unwrap()))
                />
                <LabeledInput
                    label="Maximum"
                    id="max-input"
                    value=&self.max
                    on_input=self.link.callback(|e: InputData| Msg::SetMax(e.value.parse().unwrap()))
                />
            </SettingsBlock>
        };
        let decrement_button = html! {
            <IncrementButton
                direction=Direction::Down(self.interval)
                on_click=self.link.callback(|_| Msg::Decrement)
            />
        };
        let increment_button = html! {
            <IncrementButton
                direction=Direction::Up(self.interval)
                on_click=self.link.callback(|_| Msg::Increment)
            />
        };
        html! {
            <div class="app">
                <div class="app-header">
                    <h2>{ "Increment Counter" }</h2>
                </div>
                <div class="app-body">
                    <div class="app-container counter-container">
                        <div class="counter-component">
                            { if self.can_decrement() { decrement_button } else { make_empty() } }
                        </div>
                        <div class="counter-component">
                            <h1 class="current-value">{ format!("{}", self.value ) }</h1>
                        </div>
                        <div class="counter-component">
                            { if self.can_increment() { increment_button } else { make_empty() } }
                        </div>
                    </div>
                    <div class="settings-blocks">
                        <SettingsBlock title="Settings">
                            <LabeledInput
                                label="Interval"
                                id="interval-input"
                                value=&self.interval
                                on_input=self.link.callback(|e: InputData| Msg::SetInterval(e.value.parse().unwrap()))
                            />
                            <LabeledCheckbox
                                label="Limit to range"
                                id="limit-toggle"
                                checked=&self.limited
                                on_click=self.link.callback(|_| Msg::ToggleLimit)
                            />
                        </SettingsBlock>
                        { if self.limited { min_max } else { make_empty() } }
                    </div>
                </div>
            </div>
        }
    }
}

// TODO: Tests
#[cfg(test)]
mod tests {
    #[test]
    fn it_works() {
        assert_eq!(2 + 2, 4);
    }
}