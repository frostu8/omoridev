use druid::Data;

use std::fmt::{Debug, Display, Formatter, Result as FmtResult};

use crate::util::bool_switch;

#[derive(Clone, Data)]
pub struct Script {
    #[data(same_fn = "PartialEq::eq")]
    pub contents: Vec<ScriptEntry>,
}

impl Script {
    pub fn new() -> Script {
        Script { contents: Vec::new() }
    }
}

pub type ScriptEntry = ScriptInstruction;

#[derive(Clone, Data, Debug, PartialEq)]
pub enum ScriptInstruction {
    NoOp,
    Wait(u32),
    ControlSelfSwitch(SelfSwitch, bool),
    PluginCommand(String),
}

impl Display for ScriptInstruction {
    fn fmt(&self, f: &mut Formatter<'_>) -> FmtResult {
        match self {
            ScriptInstruction::NoOp => write!(f, "No Operation"),
            ScriptInstruction::Wait(frames) => write!(f, "Wait: {} frame{}", frames, if *frames == 1 { "" } else { "s" }),
            ScriptInstruction::ControlSelfSwitch(swi, on) => write!(f, "Control Self Switch: {} = {}", swi, bool_switch(on)),
            ScriptInstruction::PluginCommand(args) => write!(f, "Plugin Command: {}", args),
        }
    }
}

#[derive(Clone, Data, Debug, PartialEq)]
pub enum SelfSwitch { A, B, C, D }

impl Display for SelfSwitch {
    fn fmt(&self, f: &mut Formatter<'_>) -> FmtResult {
        match *self {
            SelfSwitch::A => write!(f, "A"),
            SelfSwitch::B => write!(f, "B"),
            SelfSwitch::C => write!(f, "C"),
            SelfSwitch::D => write!(f, "D"),
        }
    }
}
