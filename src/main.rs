use druid::{AppLauncher, WindowDesc, Widget, PlatformError};
use druid::widget::{Scroll};

use omoridev::event::script::{Script, ScriptInstruction, SelfSwitch};
use omoridev::widget::ScriptEditor;

fn build_test_script() -> Script {
    let mut script = Script::new();

    script.contents.push(ScriptInstruction::NoOp);
    script.contents.push(ScriptInstruction::Wait(10));
    script.contents.push(ScriptInstruction::NoOp);
    script.contents.push(ScriptInstruction::NoOp);
    script.contents.push(ScriptInstruction::NoOp);
    script.contents.push(ScriptInstruction::Wait(1));
    script.contents.push(ScriptInstruction::PluginCommand("ShowMessage fa_map_flavor.message_366".into()));
    script.contents.push(ScriptInstruction::ControlSelfSwitch(SelfSwitch::A, true));

    script
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