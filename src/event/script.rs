use druid::Data;

use std::fmt::{Debug, Display, Formatter, Result as FmtResult};
use std::ops::Deref as _;
use std::sync::Arc;

use crate::util::bool_switch;

#[derive(Clone, Data)]
pub struct Script {
    pub contents: Arc<Vec<ScriptEntry>>,
}

impl Script {
    pub fn new() -> Script {
        Script::new_with(|_| {})
    }

    pub fn new_with<F>(mut f: F) -> Script 
    where F: FnMut(&mut Vec<ScriptEntry>) {
        let mut vec = Vec::new();
        f(&mut vec);

        Script { contents: Arc::new(vec) }
    }
    
    pub fn contents(&self) -> &Vec<ScriptEntry> {
        self.contents.deref()
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
