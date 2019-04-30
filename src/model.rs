use yew::html;
use yew::html::{Component, ComponentLink, Html, Renderable, ShouldRender};
use zeerust::z80::{io::BufOutput, Z80};

use crate::components::{Opcode, ProgramSelect, Registers};

pub struct Model {
    z80: Z80,
    show_memory: bool,
    output: BufOutput,
}

pub enum CPUCommand {
    Step,
    Run,
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

impl Model {
    fn output(&self) -> String {
        let out = self.output.result();
        match String::from_utf8(out.clone()) {
            Ok(s) => s,
            Err(_) => format!("{:x?}", out),
        }
    }
    fn memory_view(&self) -> Html<Self> {
        html! {
            <pre>
            { for self.z80.memory.memory.iter().enumerate().map(byte_view) }
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
        let mut z80 = Z80::default();
        let output = BufOutput::default();
        z80.install_output(0, Box::new(output.clone()));

        Self {
            z80,
            show_memory: false,
            output,
        }
    }

    fn update(&mut self, msg: Self::Message) -> ShouldRender {
        match msg {
            CPUCommand::Step => self.z80.step(),
            CPUCommand::Run => self.z80.run(),
            CPUCommand::ShowMemory(b) => self.show_memory = b,
            CPUCommand::LoadProgram(program) => self.z80.load(program),
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
            <content>
                <div>
                    <Opcode: opcode=opcode, />
                </div>
                <div>
                    <Registers: registers=self.z80.registers.clone(), />
                </div>
                <div>
                <textarea disabled=true, >{ self.output() }</textarea>
                </div>
                <button onclick=|_| CPUCommand::Step,> { "Step" }  </button>
                <button onclick=|_| CPUCommand::Run,> { "Run" } </button>
                <ProgramSelect: onchange=|program| CPUCommand::LoadProgram(program), />
                { self.memory_ui() }
            </content>
        }
    }
}
