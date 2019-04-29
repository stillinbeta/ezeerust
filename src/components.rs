use yew::html;
use yew::html::{Component, ComponentLink, Html, Renderable, ShouldRender};
use zeerust::ops::Op;

pub struct Opcode {
    opcode: Option<Op>,
}

#[derive(PartialEq, Default, Clone)]
pub struct OpcodeProperties {
    opcode: Option<Op>,
}

impl Component for Opcode {
    type Message = ();
    type Properties = OpcodeProperties;

    fn create(props: Self::Properties, _link: ComponentLink<Self>) -> Self {
        Self {
            opcode: props.opcode,
        }
    }

    fn change(&mut self, props: Self::Properties) -> ShouldRender {
        self.opcode = props.opcode;
        true
    }

    fn update(&mut self, _msg: Self::Message) -> ShouldRender {
        false
    }
}

impl Renderable<Opcode> for Opcode {
    fn view(&self) -> Html<Opcode> {
        html!(
            <div> {
                if let Some(opcode) = &self.opcode {
                    format!("{:?}", opcode)
                } else {
                    "No instruction found".into()
                }
            }
            </div>
        )
    }
}
