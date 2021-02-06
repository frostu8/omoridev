//! Common widgets used in Omoridev.

use druid::{Rect, Affine, FontFamily, Point};
use druid::piet::{TextLayoutBuilder, Text as _};
use druid::widget::prelude::*;
use druid::theme;

use crate::event::script::Script;

/// RPGMaker MV script editor.
pub struct ScriptEditor {
    entry_size: f64,
    text_size: f64,
    border_width: f64,

    selected: usize,
}

impl ScriptEditor {
    pub fn new() -> ScriptEditor {
        ScriptEditor {
            entry_size: 30.0,
            text_size: 17.0,
            border_width: 1.0,

            selected: 0,
        }
    }

    pub fn with_entry_size(mut self, entry_size: f64) -> ScriptEditor {
        self.entry_size = entry_size;
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

    fn text_margin(&self) -> f64 {
        (self.entry_size - self.text_size) / 2.0
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
                let i = (event.pos.y / self.entry_size) as usize;

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
    }

    fn update(
        &mut self,
        ctx: &mut UpdateCtx<'_, '_>,
        old_data: &Script,
        data: &Script,
        env: &Env,
    ) {
        // process data model updates
    }

    fn layout(
        &mut self,
        ctx: &mut LayoutCtx<'_, '_>,
        bc: &BoxConstraints,
        data: &Script,
        env: &Env,
    ) -> Size {
        // process layout changes
        bc.constrain(Size::new(
            // for now just use this
            bc.max().width,
            data.contents.len() as f64 * self.entry_size,
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

        let color_light = env.get(theme::BACKGROUND_LIGHT);
        let color_dark = env.get(theme::BACKGROUND_DARK);
        let color_selected = env.get(theme::PRIMARY_LIGHT);

        for (i, s) in data.contents.iter().enumerate() {
            ctx.with_save(|ctx| {
                ctx.transform(Affine::translate((0.0, i as f64 * self.entry_size)));

                // draw bg
                let bg_rect = Rect::new(0.0, 0.0, size.width, self.entry_size);

                ctx.fill(bg_rect,
                    if i % 2 == 0 {
                        &color_light
                    } else {
                        &color_dark
                    }
                );

                // render script instruction text
                let text_layout = ctx.text().new_text_layout(s.to_string())
                    .font(FontFamily::MONOSPACE, self.text_size)
                    .text_color(env.get(theme::FOREGROUND_LIGHT))
                    .build().unwrap();

                ctx.draw_text(&text_layout, Point::new(self.text_margin(), self.text_margin()));

                // draw border if selected
                if self.selected == i {
                    ctx.stroke(bg_rect.inset(-self.border_width), &color_selected, self.border_width);
                }
            })
        }
    }
}