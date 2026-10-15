use slot_gfx::{
    square, square_canvas, square_rect, square_source, Canvas, Layout, SquareGame, OUT_H,
};

const CUBE: (u32, u32) = (720, 720);

const GB: SquareGame = SquareGame {
    picture: (160, 144),
    stretch: false,
};

#[test]
fn only_a_square_panel_is_square() {
    assert!(square(CUBE));
    assert!(!square((640, 480)));
    assert!(!square((720, 480)));
    assert!(!square((0, 0)));
}

#[test]
fn a_game_boy_picture_fills_the_cube_width_at_ten_by_nine() {
    assert_eq!(square_rect(CUBE, GB), (0, 36, 720, 648));
}

#[test]
fn stretch_fills_the_whole_cube() {
    let game = SquareGame {
        stretch: true,
        ..GB
    };
    assert_eq!(square_rect(CUBE, game), (0, 0, 720, 720));
}

#[test]
fn the_hud_canvas_keeps_the_rect_aspect() {
    let canvas = square_canvas(CUBE, GB);
    assert_eq!(canvas.h, OUT_H as f32);
    assert_eq!(canvas.w, 533.0);
}

#[test]
fn the_source_is_the_game_boy_picture_inside_the_gba_texture() {
    assert_eq!(
        square_source(GB),
        [40.0 / 240.0, 8.0 / 160.0, 160.0 / 240.0, 144.0 / 160.0]
    );
}

#[test]
fn the_menus_draw_on_a_square_canvas_at_panel_resolution() {
    let canvas = Canvas::of(Layout::Native, CUBE);
    assert_eq!((canvas.w, canvas.h), (480.0, 480.0));
}

#[test]
fn a_framed_layout_on_the_cube_keeps_the_regular_canvas() {
    assert_eq!(Canvas::of(Layout::Framed, CUBE), Canvas::FRAMED);
}
