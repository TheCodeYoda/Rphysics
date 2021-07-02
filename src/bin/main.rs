use piston_window::*;
use rphysics::circle::*;
// use rand::*;

// const HEIGHT: f64 = 720.0;
// const WIDTH: f64 = 1280.0;

// struct Bubble {
//     speed: f64,
//     x: f64,
//     y: f64,
//     r: f64,
// }

// impl Bubble {
//     fn new(num: Option<f64>) -> Bubble {
//         let r: f64 = (random::<f64>() * WIDTH / 8.0) + 5.0;
//         let mut b: Bubble = Bubble {
//             speed: (random::<f64>() * (WIDTH / 8.0)) + 10.0,
//             y: random::<f64>() * (HEIGHT + r),
//             x: random::<f64>() * WIDTH,
//             r: r,
//         };
//         if let Some(y) = num {
//             b.speed = 0.0;
//             b.y = y;
//         }
//         return b;
//     }
// }

fn get_circles() -> Vec<Circle> {
    let circ_1 = Circle::new(0.0, 400.0, 100.0);
    let circ_2 = Circle::new(400.0, 400.0, 100.0);
    return vec![circ_1, circ_2];
}

fn main() {
    let mut window: PistonWindow = WindowSettings::new("Hello World!", [512; 2])
        .build()
        .unwrap();
    // render loop
    while let Some(e) = window.next() {
        window.draw_2d(&e, |c, g, _| {
            clear([0.5, 0.5, 0.5, 1.0], g);
            let circ_list = get_circles();
            for circ in circ_list {
                ellipse(
                    [1.0, 0.0, 0.0, 1.0],                     // red color
                    [circ.x(), circ.y(), circ.r(), circ.r()], // rectangle  [x,y,width,height]
                    c.transform,
                    g,
                );
            }
        });
    }
}
