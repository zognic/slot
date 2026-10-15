use std::time::Duration;

use slot::frontend::Frontend;
use slot_gfx::{Compositor, HeadlessSurface};
use slot_input::{Btn, InputSource, Millis, RawEvent};
use slot_power::SimPlatform;

const FPS: f64 = 60.0;

#[derive(Default)]
struct Scripted(Vec<RawEvent>);

impl InputSource for Scripted {
    fn poll(&mut self, _now: Millis) -> Vec<RawEvent> {
        std::mem::take(&mut self.0)
    }
}

fn btn(name: &str) -> Btn {
    match name {
        "up" => Btn::Up,
        "down" => Btn::Down,
        "left" => Btn::Left,
        "right" => Btn::Right,
        "a" => Btn::A,
        "b" => Btn::B,
        "x" => Btn::X,
        "y" => Btn::Y,
        "l1" => Btn::L1,
        "r1" => Btn::R1,
        "start" => Btn::Start,
        "select" => Btn::Select,
        "menu" => Btn::Menu,
        other => panic!("unknown button {other}"),
    }
}

fn main() {
    let args: Vec<String> = std::env::args().skip(1).collect();
    let [root, out, w, h, script] = args.as_slice() else {
        eprintln!("usage: shot ROOT OUT_DIR W H 'wait 60; tap a; shot name; ...'");
        std::process::exit(2);
    };
    let window: (u32, u32) = (w.parse().unwrap(), h.parse().unwrap());
    let surface = HeadlessSurface::new().expect("headless GL");
    let mut compositor = Compositor::new(&surface).expect("compositor");
    let mut frontend = Frontend::boot(Box::new(
        SimPlatform::at(root.into()).stopped_at(1_790_784_000),
    ));
    frontend.upload_faces(&mut compositor);
    frontend.upload_bezel(&mut compositor, window);
    frontend.drive_emulator();
    let mut input = Scripted::default();
    let mut frame = 0u64;
    let present = Duration::from_secs_f64(1.0 / FPS);
    let mut run = |input: &mut Scripted, frontend: &mut Frontend, c: &mut Compositor| {
        frame += 1;
        let now = (frame as f64 * 1000.0 / FPS) as Millis;
        frontend.advance_at(input, now, (1.0 / FPS) as f32);
        frontend.step_emulator(present, Duration::from_millis(50));
        frontend.render(c, window);
    };
    for step in script.split(';') {
        let words: Vec<&str> = step.split_whitespace().collect();
        match words.as_slice() {
            [] => {}
            ["wait", n] => {
                for _ in 0..n.parse::<u32>().unwrap() {
                    run(&mut input, &mut frontend, &mut compositor);
                }
            }
            ["tap", b] => {
                input.0.push(RawEvent::Down(btn(b)));
                for _ in 0..4 {
                    run(&mut input, &mut frontend, &mut compositor);
                }
                input.0.push(RawEvent::Up(btn(b)));
                run(&mut input, &mut frontend, &mut compositor);
            }
            ["shot", name] => {
                let px = compositor.read_window(window);
                let path = format!("{out}/{name}.png");
                let file = std::fs::File::create(&path).expect("create png");
                let mut enc = png::Encoder::new(file, window.0, window.1);
                enc.set_color(png::ColorType::Rgba);
                enc.set_depth(png::BitDepth::Eight);
                enc.write_header().unwrap().write_image_data(&px).unwrap();
                eprintln!(
                    "shot: {path} platform {:?} square {:?} layout {:?}",
                    frontend.app().playing_platform(),
                    frontend.app().square_game(),
                    frontend.app().layout()
                );
            }
            _ => panic!("cannot read {step:?}"),
        }
    }
}
