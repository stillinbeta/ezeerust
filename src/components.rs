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
            <div class={"opcode"},> {
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
            <tr>
                <td colspan=2,>
                    <Register16: label={ format!("{}", ab) }, value = self.registers.get_reg16(&ab), />
                </td>
            </tr>
            <tr>
                <td>
                    <Register8: label = { format!("{}", a) }, value = self.registers.get_reg8(a), />
                </td>
                <td>
                    <Register8: label = { format!("{}", b) }, value = self.registers.get_reg8(b), />
                </td>
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
                <td colspan=2,>
                    <Register16: label={"PC"}, value = self.registers.get_pc(), />
                </td>
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

pub struct Register16 {
    pub label: String,
    pub value: u16,
}

#[derive(Default, PartialEq, Clone)]
pub struct RegisterProps<T> {
    pub label: String,
    pub value: T,
}

impl Component for Register16 {
    type Message = ();
    type Properties = RegisterProps<u16>;

    fn create(props: Self::Properties, _link: ComponentLink<Self>) -> Self {
        Self {
            label: props.label,
            value: props.value,
        }
    }

    fn change(&mut self, props: Self::Properties) -> ShouldRender {
        self.label = props.label;
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
            <div class={"register-16"},>
                <strong> { &self.label }{ ":" } </strong>
                <code> { format!("{:04x}", self.value) }</code>
            </div>
        }
    }
}

pub struct Register8 {
    pub label: String,
    pub value: u8,
}

impl Component for Register8 {
    type Message = ();
    type Properties = RegisterProps<u8>;

    fn create(props: Self::Properties, _link: ComponentLink<Self>) -> Self {
        Self {
            label: props.label,
            value: props.value,
        }
    }

    fn change(&mut self, props: Self::Properties) -> ShouldRender {
        self.label = props.label;
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
                <div class={"register-8"},>
                <strong>{ format!("{}:", self.label) }</strong>
                <code>{ format!("{:02x}", self.value) }</code>
                </div>
        }
    }
}

#[derive(Default, PartialEq, Clone)]
pub struct Literal16 {
    pub value: u16,
}

impl Component for Literal16 {
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

impl Renderable<Literal16> for Literal16 {
    fn view(&self) -> Html<Literal16> {
        html! {
            <>
                <code> { format!("{:04x}", self.value) }</code>
            </>
        }
    }
}

#[derive(Default, PartialEq, Clone)]
pub struct Literal8 {
    pub value: u8,
}

impl Component for Literal8 {
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

impl Renderable<Literal8> for Literal8 {
    fn view(&self) -> Html<Literal8> {
        html! {
            <>
                <code> { format!("{:02x}", self.value) }</code>
                </>
        }
    }
}

#[derive(PartialEq, Clone, Default)]
pub struct ProgramSelect {
    pub onchange: Option<Callback<&'static [u8]>>,
    pub disabled: bool,
}

impl Component for ProgramSelect {
    type Message = Option<usize>;
    type Properties = Self;

    fn create(props: Self::Properties, _link: ComponentLink<Self>) -> Self {
        props
    }

    fn change(&mut self, props: Self::Properties) -> ShouldRender {
        self.onchange = props.onchange;
        self.disabled = props.disabled;
        true
    }

    fn update(&mut self, msg: Self::Message) -> ShouldRender {
        let example = msg.and_then(|v| zeerust::examples::EXAMPLES.get(v));

        if let Some(ref mut callback) = self.onchange {
            if let Some(example) = example {
                callback.emit(example.binary);
                return true;
            }
        }
        false
    }
}

fn program_option(opts: (usize, &str)) -> Html<ProgramSelect> {
    let (index, name) = opts;
    let index = index + 1;
    html! {
        <option value=index + 1, >{ name }</option>
    }
}

impl Renderable<ProgramSelect> for ProgramSelect {
    fn view(&self) -> Html<ProgramSelect> {
        html! {
            <select disabled=self.disabled, onchange=|evt| {
                match evt {
                    ChangeData::Select(elem) => {
                        // First item will be zero, so it'll underflow.
                        elem.selected_index().and_then(|x| (x as usize).checked_sub(1))
                    },
                    _ => None
                }
            }, >
                <option>{ "Load a program" }</option>
                { for zeerust::examples::EXAMPLES.iter().map(|e| e.name).enumerate().map(program_option) }
            </select>
        }
    }
}
