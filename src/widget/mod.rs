//! Common widgets used in Omoridev.

use druid::{Rect, Affine, FontFamily, FontDescriptor, Point, Color, TextLayout};
use druid::widget::prelude::*;
use druid::theme;

use crate::event::script::{Script, ScriptEntry, ScriptInstruction};
use crate::util;

/// RPGMaker MV script editor.
pub struct ScriptEditor {
    text_size: f64,
    text_padding: f64,
    border_width: f64,

    selected: usize,
    entry_layouts: Vec<TextLayout<String>>,
}

impl ScriptEditor {
    pub fn new() -> ScriptEditor {
        ScriptEditor {
            text_padding: 5.0,
            text_size: 17.0,
            border_width: 1.0,

            selected: 0,
            entry_layouts: Vec::new(),
        }
    }

    pub fn with_text_padding(mut self, text_padding: f64) -> ScriptEditor {
        self.text_padding = text_padding;
        self
    }

    pub fn with_text_size(mut self, text_size: f64) -> ScriptEditor {
        self.text_size = text_size;
        self
    }

    pub fn with_border_width(mut self, border_width: f64) -> ScriptEditor {
        self.border_width = border_width;
        self
    }

    fn reconstruct_text_layouts(&mut self, script: &Script, env: &Env) {
        // clear layouts
        self.entry_layouts.clear();

        // build text layouts
        for (i, entry) in script.contents().iter().enumerate() {
            let mut text_layout = TextLayout::<String>::from_text(entry.to_string());

            text_layout.set_font(FontDescriptor::new(FontFamily::MONOSPACE)
                .with_size(self.text_size));
            text_layout.set_text_color(script_highlight_color(&entry, &bg_color(i, env)));

            self.entry_layouts.push(text_layout);
        }
    }

    fn vertical_bounds(&self, i: usize) -> f64 {
        let mut bounds: f64 = 0.0;

        for text_layout in self.entry_layouts.iter().take(i) {
            bounds += self.entry_size(&text_layout);
        }

        bounds
    }

    fn find_entry_physical(&self, y: f64) -> usize {
        // if the y is greater than all of the vertical bounds, then quit early
        if y > self.vertical_bounds(self.entry_layouts.len()) {
            self.entry_layouts.len()
        } else {
            (0..self.entry_layouts.len())
                .map(|i| (i, self.vertical_bounds(i)))
                .find(|(_, b)| *b > y)
                .map(|(i, _)| i)
                .unwrap_or(self.entry_layouts.len()) - 1
        }
    }

    fn entry_size(&self, text_layout: &TextLayout<String>) -> f64 {
        text_layout.layout_metrics().size.height + self.text_padding * 2.0
    }
}

impl Widget<Script> for ScriptEditor {
    fn event(
        &mut self,
        ctx: &mut EventCtx<'_, '_>,
        event: &Event,
        data: &mut Script,
        env: &Env,
    ) {
        // process events and input
        match event {
            Event::MouseDown(event) => {
                // we do not have to worry about x or width because
                // each instruction is the same width
                let i = self.find_entry_physical(event.pos.y);

                // simply set the selected to i, since the empty void should
                // deselect, which it does
                self.selected = i;

                // request repaint
                ctx.request_paint();
            },
            _ => (),
        }
    }

    fn lifecycle(
        &mut self,
        ctx: &mut LifeCycleCtx<'_, '_>,
        event: &LifeCycle,
        data: &Script,
        env: &Env,
    ) {
        // process lifecycle events from druid
        match event {
            LifeCycle::WidgetAdded => self.reconstruct_text_layouts(data, env),
            _ => (),
        }
    }

    fn update(
        &mut self,
        ctx: &mut UpdateCtx<'_, '_>,
        old_data: &Script,
        data: &Script,
        env: &Env,
    ) {
        if old_data.contents() != data.contents() {
            // process data model updates
            self.reconstruct_text_layouts(data, env);
            ctx.request_layout();
        }
    }

    fn layout(
        &mut self,
        ctx: &mut LayoutCtx<'_, '_>,
        bc: &BoxConstraints,
        data: &Script,
        env: &Env,
    ) -> Size {
        // rebuild text layouts
        for layout in self.entry_layouts.iter_mut() {
            layout.rebuild_if_needed(ctx.text(), env);
        }

        // process layout changes
        bc.constrain(Size::new(
            // for now just use this
            bc.max().width,
            self.vertical_bounds(self.entry_layouts.len()),
        ))
    }

    fn paint(
        &mut self,
        ctx: &mut PaintCtx<'_, '_, '_>, 
        data: &Script, 
        env: &Env,
    ) {
        // paint the widget
        let size = ctx.size();

        let color_selected = env.get(theme::PRIMARY_LIGHT);

        for (i, layout) in self.entry_layouts.iter().enumerate() {
            ctx.with_save(|ctx| {
                ctx.transform(Affine::translate((0.0, self.vertical_bounds(i))));

                // draw bg
                let bg_rect = Rect::new(0.0, 0.0, size.width, self.entry_size(&layout));

                ctx.fill(bg_rect, &bg_color(i, env));

                layout.draw(ctx, Point::new(self.text_padding, self.text_padding));

                // draw border if selected
                if self.selected == i {
                    ctx.stroke(bg_rect.inset(-self.border_width), &color_selected, self.border_width);
                }
            })
        }
    }
}

/// Gets the best highlight color of an instruction based on its background.
pub fn script_highlight_color(entry: &ScriptEntry, bg: &Color) -> Color {
    if util::is_dark(bg) { script_highlight_color_dark(entry) }
    else { script_highlight_color_light(entry) }
}

/// Gets the highlight color of an instruction. For dark themes.
pub fn script_highlight_color_dark(entry: &ScriptEntry) -> Color {
    match entry {
        ScriptInstruction::NoOp => Color::rgb8(255, 255, 255),
        ScriptInstruction::Wait(_) => Color::rgb8(247, 32, 32),
        ScriptInstruction::ControlSelfSwitch(_, _) => Color::rgb8(247, 32, 32),
        ScriptInstruction::PluginCommand(_) => Color::rgb8(167, 92, 237),
        ScriptInstruction::Script(_) => Color::rgb8(119, 52, 235),
    }
}

/// Gets the highlight color of an instruction. For light themes.
pub fn script_highlight_color_light(_entry: &ScriptEntry) -> Color {
    unimplemented!("stub because light themes stink!");
}

fn bg_color(i: usize, env: &Env) -> Color {
    if i % 2 == 0 {
        env.get(theme::BACKGROUND_LIGHT)
    } else {
        env.get(theme::BACKGROUND_DARK)
    }
}
