use piston_window::*;
use rand::*;

const HEIGHT: f64 = 720.0;
const WIDTH: f64 = 1280.0;

struct Bubble {
    speed: f64,
    x: f64,
    y: f64,
    r: f64,
}

impl Bubble {
    fn new(num: Option<f64>) -> Bubble {
        let r: f64 = (random::<f64>() * WIDTH / 8.0) + 5.0;
        let mut b: Bubble = Bubble {
            speed: (random::<f64>() * (WIDTH / 8.0)) + 10.0,
            y: random::<f64>() * (HEIGHT + r),
            x: random::<f64>() * WIDTH,
            r: r,
        };
        if let Some(y) = num {
            b.speed = 0.0;
            b.y = y;
        }
        return b;
    }
}

fn get_bubbles() -> Vec<Bubble> {
    let mut bubbles = Vec::new();
    let n = (random::<u64>() % 15) + 10;
    for _ in 0..n {
        // bottom bubbles
        bubbles.push(Bubble::new(Some(HEIGHT)));
        bubbles.push(Bubble::new(Some(0.0)));
        bubbles.push(Bubble::new(None));
    }
    bubbles
}

fn main() {
    let bub = [1.0, 97.0 / 255.0, 0.0, 1.0];
    let bg = [104.0 / 255.0, 221.0 / 255.0, 19.0 / 255.0, 1.0];

    let mut bubbles: Vec<Bubble> = get_bubbles();
    // creating a piston window
    let mut window: PistonWindow = WindowSettings::new("lava lamp", [WIDTH, HEIGHT])
        .exit_on_esc(true)
        .build()
        .unwrap();

    let mut events = window.events;
    // event loop
    while let Some(e) = events.next(&mut window) {
        if let Some(_) = e.render_args() {
            let bubbs = &bubbles;
            window.draw_2d(&e, |c, g, _| {
                clear(bg, g);
                for b in bubbs {
                    ellipse(
                        bub,
                        [b.x - b.r, b.y - b.r, b.r * 2.0, b.r * 2.0],
                        c.transform,
                        g,
                    );
                }
            });
            ()
        }
        if let Some(u) = e.update_args() {
            let bubbs = &mut bubbles;
            for bub in bubbs {
                bub.y -= bub.speed * u.dt; // dt = unit of time
                if bub.y + bub.r <= 0.0 {
                    bub.y = HEIGHT + bub.r;
                }
            }
        }
    }
}
