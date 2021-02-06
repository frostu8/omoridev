use druid::Data;

use std::fmt::{Debug, Display, Formatter, Result as FmtResult};

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
    PluginCommand(String),
}

impl Display for ScriptInstruction {
    fn fmt(&self, f: &mut Formatter<'_>) -> FmtResult {
        match self {
            ScriptInstruction::NoOp => write!(f, "No Operation"),
            ScriptInstruction::Wait(frames) => write!(f, "Wait: {} frame{}", frames, if *frames == 1 { "" } else { "s" }),
            ScriptInstruction::PluginCommand(args) => write!(f, "Plugin Command: {}", args),
        }
    }
}
