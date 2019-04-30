use yew::html;
use yew::html::{Component, ComponentLink, Html, Renderable, ShouldRender};
use zeerust::z80::{io::BufOutput, Z80};

use crate::components::{Opcode, ProgramSelect, Registers};

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

impl Renderable<Model> for Model {
    fn view(&self) -> Html<Self> {
        let z80 = &self.machine.z80;
        let opcode = z80
            .parse_opcode(z80.registers.get_pc() as usize)
            .map(|(opc, _consumed)| opc);

        html! {
            <content>
                <div>
                <textarea disabled=true, >{ self.output() }</textarea>
                </div>

                <div>
                    <Opcode: opcode=opcode, />
                </div>
                <div>
                    <Registers: registers=z80.registers.clone(), />
                </div>
                <button onclick=|_| CPUCommand::Step,> { "Step" }  </button>
                <button onclick=|_| CPUCommand::Run,> { "Run" } </button>
                <button onclick=|_| CPUCommand::Reset,> { "Reset" } </button>
                <ProgramSelect: disabled=self.loaded, onchange=|program| CPUCommand::LoadProgram(program), />
                { self.memory_ui() }
            </content>
        }
    }
}
