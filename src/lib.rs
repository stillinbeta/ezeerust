use yew::html;
use yew::html::{Component, ComponentLink, Html, Renderable, ShouldRender};
use yew::Callback;

pub struct Clicker {
    clicks: u64,
}

impl Component for Clicker {
    type Message = u32;
    type Properties = u64;

    fn create(props: Self::Properties, _link: ComponentLink<Self>) -> Self {
        Clicker { clicks: props }
    }

    fn update(&mut self, msg: Self::Message) -> ShouldRender {
        self.clicks += msg as u64;
        true
    }
}

impl Renderable<Clicker> for Clicker {
    fn view(&self) -> Html<Self> {
        html! {
            <span class="clicks",>
            { self.clicks }
            </span>
            <br />
            <ClickButton: size={ 1 }, clicks= { self.clicks }, threshold=Some(0), callback=|i| i as u32, />
            <ClickButton: size={ 5 }, clicks={ self.clicks }, callback=|i| i as u32, />
            <ClickButton: size={ 10 }, clicks={ self.clicks }, callback=|i| i as u32, />
        }
    }
}

pub struct ClickButton {
    increment: u8,
    clicks: u64,
    threshold: u8,
    callback: Option<Callback<u8>>,
}

pub enum ButtonMessage {
    Clicked,
}

#[derive(Default, PartialEq, Clone)]
pub struct ButtonProperties {
    size: u8,
    clicks: u64,
    threshold: Option<u8>,
    callback: Option<Callback<u8>>,
}

impl Component for ClickButton {
    type Message = ButtonMessage;
    type Properties = ButtonProperties;

    fn create(props: Self::Properties, _link: ComponentLink<Self>) -> Self {
        Self {
            increment: props.size,
            clicks: props.clicks,
            threshold: props.threshold.unwrap_or(props.size),
            callback: props.callback,
        }
    }

    fn change(&mut self, props: Self::Properties) -> ShouldRender {
        self.clicks = props.clicks;
        true
    }

    fn update(&mut self, msg: Self::Message) -> ShouldRender {
        match msg {
            ButtonMessage::Clicked => {
                if let Some(ref mut cb) = self.callback {
                    cb.emit(self.increment)
                }
            }
        }
        false
    }
}

impl Renderable<ClickButton> for ClickButton {
    fn view(&self) -> Html<Self> {
        html! {
            <button disabled={ self.clicks < (self.threshold as u64)}, onclick = |_| ButtonMessage::Clicked,>
            { format!("Add {}", self.increment) }
            </button>
        }
    }
}
