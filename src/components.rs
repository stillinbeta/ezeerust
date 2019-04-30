use yew::html;
use yew::html::{ChangeData, Component, ComponentLink, Html, Renderable, ShouldRender};
use yew::Callback;
use zeerust::ops::{Op, Reg16, Reg8};
use zeerust::z80::io::BufOutput;

pub struct Opcode {
    opcode: Option<Op>,
}

#[derive(PartialEq, Default, Clone)]
pub struct OpcodeProperties {
    pub opcode: Option<Op>,
    pub output: Option<&'static BufOutput>,
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

#[derive(Default, PartialEq, Clone)]
pub struct Registers {
    pub registers: zeerust::cpu::reg::Registers,
}

impl Registers {
    fn make_pair(&self, a: Reg8, b: Reg8, ab: Reg16) -> Html<Registers> {
        html! {
            <>
            <Register16: label={ format!("{}", ab) }, value = self.registers.get_reg16(&ab), />
            <tr>
                <Register8: label = { format!("{}", a) }, value = self.registers.get_reg8(a), />
                <Register8: label = { format!("{}", b) }, value = self.registers.get_reg8(b), />
            </tr>
            </>
        }
    }
}

impl Component for Registers {
    type Message = ();
    type Properties = Self;

    fn create(props: Self::Properties, _link: ComponentLink<Self>) -> Self {
        props
    }

    fn change(&mut self, props: Self::Properties) -> ShouldRender {
        self.registers = props.registers;
        true
    }

    fn update(&mut self, _msg: Self::Message) -> ShouldRender {
        false
    }
}

impl Renderable<Registers> for Registers {
    fn view(&self) -> Html<Registers> {
        html! {
            <table>
              { self.make_pair(Reg8::A, Reg8::F, Reg16::AF) }
              { self.make_pair(Reg8::B, Reg8::C, Reg16::BC,) }
              { self.make_pair(Reg8::D, Reg8::E, Reg16::DE) }
              { self.make_pair(Reg8::H, Reg8::L, Reg16::HL) }
              <Register16: label={"PC"}, value = self.registers.get_pc(), />
            </table>
        }
    }
}

#[derive(Clone, PartialEq, Default)]
struct RegisterPair {
    a: u8,
    aname: String,
    b: u8,
    bname: String,
}

impl Component for RegisterPair {
    type Message = ();
    type Properties = Self;

    fn create(props: Self::Properties, _link: ComponentLink<Self>) -> Self {
        props
    }

    fn change(&mut self, props: Self::Properties) -> ShouldRender {
        // Never update the names
        self.a = props.a;
        self.b = props.b;
        true
    }

    fn update(&mut self, _msg: Self::Message) -> ShouldRender {
        false
    }
}

#[derive(Default, PartialEq, Clone)]
struct Register16 {
    pub label: String,
    pub value: u16,
}

impl Component for Register16 {
    type Message = ();
    type Properties = Self;

    fn create(props: Self::Properties, _link: ComponentLink<Self>) -> Self {
        props
    }

    fn change(&mut self, props: Self::Properties) -> ShouldRender {
        self.value = props.value;
        true
    }

    fn update(&mut self, _msg: Self::Message) -> ShouldRender {
        false
    }
}

impl Renderable<Register16> for Register16 {
    fn view(&self) -> Html<Register16> {
        html! {
        <tr>
            <td colspan="2",>
                <strong> { &self.label }{ ":" } </strong>
                <code> { format!("{:04x}", self.value) }</code>
            </td>
        </tr>
        }
    }
}

#[derive(Default, PartialEq, Clone)]
struct Register8 {
    pub label: String,
    pub value: u8,
}

impl Component for Register8 {
    type Message = ();
    type Properties = Self;

    fn create(props: Self::Properties, _link: ComponentLink<Self>) -> Self {
        props
    }

    fn change(&mut self, props: Self::Properties) -> ShouldRender {
        self.value = props.value;
        true
    }

    fn update(&mut self, _msg: Self::Message) -> ShouldRender {
        false
    }
}

impl Renderable<Register8> for Register8 {
    fn view(&self) -> Html<Register8> {
        html! {
                <td>
                <strong> { &self.label }{ ":" } </strong>
                <code> { format!("{:02x}", self.value) }</code>
                </td>
        }
    }
}

#[derive(PartialEq, Clone, Default)]
pub struct ProgramSelect {
    pub onchange: Option<Callback<&'static [u8]>>,
}

impl Component for ProgramSelect {
    type Message = usize;
    type Properties = Self;

    fn create(props: Self::Properties, _link: ComponentLink<Self>) -> Self {
        props
    }

    fn change(&mut self, props: Self::Properties) -> ShouldRender {
        self.onchange = props.onchange;
        true
    }

    fn update(&mut self, msg: usize) -> ShouldRender {
        if let Some(ref mut callback) = self.onchange {
            if let Some(example) = zeerust::examples::EXAMPLES.get(msg) {
                callback.emit(example.binary)
            }
        }
        true
    }
}

fn program_option(opts: (usize, &str)) -> Html<ProgramSelect> {
    let (index, name) = opts;
    html! {
        <option value=index, >{ name }</option>
    }
}

impl Renderable<ProgramSelect> for ProgramSelect {
    fn view(&self) -> Html<ProgramSelect> {
        html! {
            <select onchange=|evt| {
                match evt {
                    ChangeData::Select(elem) => {
                        elem.selected_index().map(|x| x as usize).unwrap()
                    },
                    _ => unimplemented!("reached unknown location"),
                }
            }, >
                { for zeerust::examples::EXAMPLES.iter().map(|e| e.name).enumerate().map(program_option) }
            </select>
        }
    }
}
