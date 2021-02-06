use druid::{AppLauncher, WindowDesc, Widget, PlatformError};
use druid::widget::{Scroll};

use omoridev::event::script::{Script, ScriptInstruction, SelfSwitch};
use omoridev::widget::ScriptEditor;

fn build_test_script() -> Script {
    Script::new_with(|vec| {
        vec.push(ScriptInstruction::NoOp);
        vec.push(ScriptInstruction::Wait(10));
        vec.push(ScriptInstruction::NoOp);
        vec.push(ScriptInstruction::NoOp);
        vec.push(ScriptInstruction::NoOp);
        vec.push(ScriptInstruction::Wait(1));
        vec.push(ScriptInstruction::PluginCommand("ShowMessage fa_map_flavor.message_366".into()));
        vec.push(ScriptInstruction::ControlSelfSwitch(SelfSwitch::A, true));
    })
}

fn build_ui() -> impl Widget<Script> {
    Scroll::new(ScriptEditor::new())
        .vertical()
}

fn main() -> Result<(), PlatformError> {
    AppLauncher::with_window(WindowDesc::new(build_ui())
        .window_size((640.0, 820.0))
        .resizable(true)
    ).use_simple_logger().launch(build_test_script())?;
    Ok(())
}