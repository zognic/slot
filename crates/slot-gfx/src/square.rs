use crate::layout::Canvas;
use crate::pipeline::{SRC_H, SRC_W};
use crate::surface::OUT_H;

/// A Game Boy picture drawn straight onto a square panel (RG Cube), past the 3:2 canvas.
#[derive(Clone, Copy, PartialEq, Eq, Debug)]
pub struct SquareGame {
    pub picture: (u32, u32),
    pub stretch: bool,
}

pub fn square(panel: (u32, u32)) -> bool {
    panel.0 > 0 && panel.0 == panel.1
}

/// Where the picture lands on the panel, in GL viewport coordinates (origin bottom left).
/// Actual keeps the picture's own aspect and fills the panel's width; stretch fills it all.
pub fn square_rect(panel: (u32, u32), game: SquareGame) -> (i32, i32, i32, i32) {
    if game.stretch {
        return (0, 0, panel.0 as i32, panel.1 as i32);
    }
    let (pw, ph) = (game.picture.0.max(1) as f32, game.picture.1.max(1) as f32);
    let scale = (panel.0 as f32 / pw).min(panel.1 as f32 / ph);
    let w = (pw * scale).round() as i32;
    let h = (ph * scale).round() as i32;
    ((panel.0 as i32 - w) / 2, (panel.1 as i32 - h) / 2, w, h)
}

/// The sprite canvas over that rect: OUT_H tall, as wide as the rect's aspect asks.
pub fn square_canvas(panel: (u32, u32), game: SquareGame) -> Canvas {
    let (_, _, w, h) = square_rect(panel, game);
    Canvas {
        w: (OUT_H as f32 * w as f32 / h.max(1) as f32).round(),
        h: OUT_H as f32,
    }
}

/// The picture's texels inside the SRC_W × SRC_H game texture, as [x, y, w, h].
pub fn square_source(game: SquareGame) -> [f32; 4] {
    let (w, h) = (game.picture.0.min(SRC_W), game.picture.1.min(SRC_H));
    let (x, y) = ((SRC_W - w) / 2, (SRC_H - h) / 2);
    [
        x as f32 / SRC_W as f32,
        y as f32 / SRC_H as f32,
        w as f32 / SRC_W as f32,
        h as f32 / SRC_H as f32,
    ]
}
