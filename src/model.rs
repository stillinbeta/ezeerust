use yew::html;
use yew::html::{Component, ComponentLink, Html, Renderable, ShouldRender};
use yew::virtual_dom::{VList, VNode};
use zeerust::ops::{Location16, Location8, Op};
use zeerust::z80::{io::BufOutput, Z80};

use crate::components::{
    Literal16, Literal8, Opcode, ProgramSelect, Register16, Register8, Registers,
};
use crate::util::{op_dst_src, Location};

pub struct Model {
    machine: Machine,
    show_memory: bool,
    loaded: bool,
}

pub enum CPUCommand {
    Step,
    Run,
    Reset,
    ShowMemory(bool),
    LoadProgram(&'static [u8]),
}

fn byte_view(byte: (usize, &u8)) -> Html<Model> {
    let (index, byte) = byte;
    let b = format!("{:02x} ", byte);
    let sp = match index + 1 {
        i if i % 20 == 0 => "\n",
        i if i % 10 == 0 => "   ",
        _ => " ",
    };
    let addr = format!("{:04x}", index);
    html! {<><a id={ &addr }, title={ &addr },>{ b }</a>{ sp }</>}
}

struct Machine {
    z80: Z80,
    output: BufOutput,
}

impl Machine {
    fn new() -> Self {
        let mut z80 = Z80::default();
        let output = BufOutput::default();
        z80.install_output(0, Box::new(output.clone()));

        Machine { z80, output }
    }
}

impl Model {
    fn output(&self) -> String {
        let out = self.machine.output.result();
        match String::from_utf8(out.clone()) {
            Ok(s) => s,
            Err(_) => format!("{:x?}", out),
        }
    }
    fn memory_view(&self) -> Html<Self> {
        html! {
            <pre>
            { for self.machine.z80.memory.memory.iter().enumerate().map(byte_view) }
            </pre>
        }
    }

    fn memory_ui(&self) -> Html<Self> {
        if self.show_memory {
            html! {
                <>
                <button onclick=|_| CPUCommand::ShowMemory(false),>
                    { "Hide Memory" }
                </button>
                { self.memory_view() }
                </>
            }
        } else {
            html! {
                <>
                <button onclick=|_| CPUCommand::ShowMemory(true),>
                    { "Show Memory " }
                </button>
                </>
            }
        }
    }

    fn instruction(&self) -> Option<Op> {
        let z80 = &self.machine.z80;
        z80.parse_opcode(z80.registers.get_pc() as usize)
            .map(|(opc, _consumed)| opc)
    }

    fn location_view(&self, loc: Location) -> Html<Self> {
        let regs = &self.machine.z80.registers;
        let memory = &self.machine.z80.memory.memory;
        match loc {
            Location::Loc8(loc) => match loc {
                Location8::Reg(reg) => html! {
                    <>
                    <Register8: label={ format!("{}", reg) }, value=regs.get_reg8(reg), />
                    </>
                },
                Location8::RegIndirect(reg) => {
                    let reg_val = regs.get_reg16(&reg);
                    html! {
                        <>
                        <Register16: label = { format!("{}", reg) }, value=reg_val, />
                        <br />
                        <Literal8: value = memory[reg_val as usize], />
                        </>
                    }
                }

                Location8::ImmediateIndirect(val) => {
                    html! {
                        <>
                            <Literal16: value=val, />
                            <br />
                            <Literal8: value = memory[val as usize], />
                        </>
                    }
                }
                Location8::Immediate(val) => {
                    html! {
                        <>
                            <Literal8: value = val, />
                        </>
                    }
                }
            },
            Location::Loc16(loc) => match loc {
                Location16::Reg(reg) => html! {
                    <>
                        <Register16: label = { format!("{}", reg) }, value=regs.get_reg16(&reg), />

                    </>
                },
                Location16::ImmediateIndirect(_) => unimplemented!(),
                Location16::Immediate(val) => html! {
                    <>
                        <>
                        <Literal16: value=val, />
                    </>
                },
            },
        }
    }
}

impl Component for Model {
    type Message = CPUCommand;
    type Properties = ();

    fn create(_props: Self::Properties, _link: ComponentLink<Self>) -> Self {
        Self {
            machine: Machine::new(),
            show_memory: false,
            loaded: false,
        }
    }

    fn update(&mut self, msg: Self::Message) -> ShouldRender {
        match msg {
            CPUCommand::Step => self.machine.z80.step(),
            CPUCommand::Run => self.machine.z80.run(),
            CPUCommand::ShowMemory(b) => self.show_memory = b,
            CPUCommand::LoadProgram(program) => {
                self.machine.z80.load(program);
                self.loaded = true
            }
            CPUCommand::Reset => {
                self.machine = Machine::new();
                self.loaded = false;
            }
        }
        true
    }
}

fn empty<T>() -> Html<T>
where
    T: Component,
{
    VNode::from(VList::new())
}

impl Renderable<Model> for Model {
    fn view(&self) -> Html<Self> {
        let z80 = &self.machine.z80;
        let (dst, src) = self.instruction().map(op_dst_src).unwrap_or((None, None));
        html! {
            <content>
                <div id={ "monitor" },>
                    <div id={ "output" },>
                        <textarea disabled=true, >{ self.output() }</textarea>
                    </div>
                        <div id={ "opcode" },>
                            <Opcode: opcode={ self.instruction() }, />
                        </div>
                        <div id={ "destination" },>
                            { dst.map(|dst| self.location_view(dst)).unwrap_or_else(|| empty()) }
                        </div>
                        // <div id={ "source" },>
                        //     { src.map(|src| self.location_view(src)).unwrap_or_else(|| empty()) }
                        // </div>
                    <div id={ "registers" }, >
                        <Registers: registers=z80.registers.clone(), />
                    </div>
                </div>
                <div id={ "buttons" },>
                    <button onclick=|_| CPUCommand::Step,> { "Step" }  </button>
                    <button onclick=|_| CPUCommand::Run,> { "Run" } </button>
                    <button onclick=|_| CPUCommand::Reset,> { "Reset" } </button>
                    <ProgramSelect: disabled=self.loaded, onchange=|program| CPUCommand::LoadProgram(program), />
                </div>
                <div id={ "memory" },>
                    { self.memory_ui() }
                </div>
            </content>
        }
    }
}
