use yew::html;
use yew::html::{Component, ComponentLink, Html, Renderable, ShouldRender};

use zeerust::ops::Op;
use zeerust::z80::Z80;

pub struct Model {
    z80: Z80<'static>,
}

pub enum CPUCommand {
    Step,
    Run,
}

impl Component for Model {
    type Message = CPUCommand;
    type Properties = ();

    fn create(_props: Self::Properties, _link: ComponentLink<Self>) -> Self {
        Self {
            z80: Z80::default(),
        }
    }

    fn update(&mut self, msg: Self::Message) -> ShouldRender {
        match msg {
            CPUCommand::Step => self.z80.step(),
            CPUCommand::Run => self.z80.run(),
        }
        true
    }
}

impl Renderable<Model> for Model {
    fn view(&self) -> Html<Self> {
        let opcode = self
            .z80
            .parse_opcode(self.z80.registers.get_pc() as usize)
            .map(|(opc, _consumed)| opc);

        html! {
            <div>
            <Opcode: opcode=opcode, />
            </div>
            <button onclick=|_| CPUCommand::Step,> { "Step" }  </button>
            <button onclick=|_| CPUCommand::Run,> { "Run" } </button>
        }
    }
}

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
