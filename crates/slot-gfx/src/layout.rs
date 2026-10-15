use crate::bezel::framed;
use crate::draw::Draw;
use crate::square::square;
use crate::surface::{OUT_H, OUT_W};

#[derive(Clone, Copy, PartialEq, Eq, Debug)]
pub enum Layout {
    Native,
    Framed,
}

#[derive(Clone, Copy, PartialEq, Debug)]
pub struct Canvas {
    pub w: f32,
    pub h: f32,
}

impl Canvas {
    pub const FRAMED: Canvas = Canvas {
        w: OUT_W as f32,
        h: OUT_H as f32,
    };

    pub fn of(layout: Layout, panel: (u32, u32)) -> Canvas {
        match layout {
            Layout::Native if square(panel) => Canvas {
                w: panel.0.max(OUT_H) as f32,
                h: panel.1.max(OUT_H) as f32,
            },
            Layout::Native if framed(panel) => Canvas {
                w: (OUT_H * panel.0 / panel.1) as f32,
                h: OUT_H as f32,
            },
            _ => Canvas::FRAMED,
        }
    }

    pub fn centred(self, w: f32) -> f32 {
        (self.w - w) / 2.0
    }

    pub fn window(self, tex_w: f32) -> [f32; 4] {
        if tex_w <= self.w {
            return [0.0, 0.0, 1.0, 1.0];
        }
        [(tex_w - self.w) / 2.0 / tex_w, 0.0, self.w / tex_w, 1.0]
    }

    /// The part of an OUT_H-tall ground texture that covers the whole canvas without stretching.
    pub fn cover(self, tex_w: f32) -> [f32; 4] {
        let shown = self.w * OUT_H as f32 / self.h;
        if tex_w <= shown {
            return [0.0, 0.0, 1.0, 1.0];
        }
        [(tex_w - shown) / 2.0 / tex_w, 0.0, shown / tex_w, 1.0]
    }

    /// The height a square panel's canvas has past the OUT_H the layouts are drawn for.
    /// Bottom-anchored pieces drop by all of it, centred ones by half.
    pub fn spare(self) -> f32 {
        (self.h - OUT_H as f32).max(0.0)
    }

    pub fn mid(self) -> f32 {
        self.spare() / 2.0
    }
}

/// Moves already pushed draws down by `dy`, to place an OUT_H layout on a taller canvas.
pub fn lower(items: &mut [Draw], dy: f32) {
    if dy == 0.0 {
        return;
    }
    for item in items {
        match item {
            Draw::Rect { y, .. }
            | Draw::Tex { y, .. }
            | Draw::Part { y, .. }
            | Draw::Turned { y, .. } => *y += dy,
            Draw::Game | Draw::Shot { .. } => {}
        }
    }
}
