use piston_window::*;
// use std::f64::consts::PI;
use rand::*;
use std::env;

use rphysics::circle::*;
use rphysics::collison::*;
use rphysics::engine::*;
use rphysics::screen::*;

/// randomly spawns Non overlapping circles in the Screen
fn get_circle(list: &Vec<Circle>, screen: &Screen) -> Option<Circle> {
    let mut tries = 100000;
    while tries > 0 {
        // rand handle
        let mut rng = rand::thread_rng();

        let min_vel = 600.0;
        let max_vel = 700.0;
        let min_radius = 25.0;
        let max_radius = 30.0;
        let width = screen.width();
        let height = screen.height();

        let circ = Circle::new(
            rng.gen_range(0.0, width),
            rng.gen_range(0.0, height),
            rng.gen_range(min_radius, max_radius),
            rng.gen_range(min_vel, max_vel),
            rng.gen_range(min_vel, max_vel),
        );
        let mut flag = 1;
        for sample in list {
            if let Some(_) = circ.is_colliding(&sample) {
                flag = 0;
                break;
            }
        }
        if flag == 1 {
            return Some(circ);
        }
        tries -= 1;
    }
    return None;
}

fn get_circles(engine: &mut Engine, n: u32) {
    // let w = 512.0;
    // let h = 512.0;
    // let v = 100.0;
    // let circ_1 = Circle::new(256.0, 0.0, 50.0, 0.0, v);
    // let x = w/2.0 -((w/2.0)*(PI/3.0).sin());
    // let y = ((w/2.0*(PI/3.0).cos())) + h/2.0;
    // let circ_2 = Circle::new(x, y, 50.0,v*(PI/6.0).cos(), -v*(PI/6.0).sin());
    // let x = w/2.0 +((w/2.0)*(PI/3.0).sin());
    // let circ_3 = Circle::new(x, y, 50.0, -v*(PI/6.0).cos(), -v*(PI/6.0).sin());
    // return vec![circ_1, circ_2,circ_3];

    // ------------------custom testing ----------------------------
    // let circ_1 = Circle::new(50.0, 50.0, 50.0, 0.0, 0.0);
    // let circ_2 = Circle::new(50.0, 462.0, 50.0, 0.0, 0.0);
    // engine.object_list = vec![circ_1, circ_2];

    // --------------------------random testing -------------------------------------
    for _i in 0..n {
        if let Some(circ) = get_circle(&engine.object_list, &engine.screen) {
            engine.object_list.push(circ);
        }
    }
}

fn update(engine: &mut Engine, dt: f64) {
    engine.update_pos(dt);
    engine.resolve_collisons();
}

// fn check_collisions(engine: &mut Engine) {
//     let n = engine.object_list.len();
//     // let mut res = Vec::new();
//     for i in 0..n {
//         for j in i + 1..n {
//             if let Some(p) = engine.object_list[i].is_colliding(&engine.object_list[j]) {
//                 // resolve static collison for ith and jth circle
//                 engine.object_list[i].point = p.0;
//                 engine.object_list[j].point = p.1;
//                 // collide and readjust velocit for ith and jth sphere
//                 let vel = engine.object_list[i].collide(&engine.object_list[j], engine.e());
//                 engine.object_list[i].v = vel.0;
//                 engine.object_list[j].v = vel.1;
//             }
//         }
//     }
// }
// ellipse(x,y,halfwidth,halfheight)

fn main() {
    // commandline args
    // [gravity] [no.of.circles] [e]
    let args: Vec<String> = env::args().collect();
    // println!("{:?}", (args));
    // gravity state
    let grav_state = &args[1];
    // no.of circles
    let n: u32 = args[2].parse().unwrap();
    // coeffecient of restitution
    let e: f64 = args[3].parse().unwrap();

    println!("{:?}", (grav_state, n, e));

    let screen = Screen::new(512.0, 512.0);

    // initializing piston window
    let mut window: PistonWindow =
        WindowSettings::new("Circles!", [screen.width(), screen.height()])
            .build()
            .unwrap();

    // list of circles to render
    // let mut circ_list = get_circles(&screen, n);

    // engine object
    let mut eng = Engine::new(e, screen);

    // get object list
    get_circles(&mut eng, n);

    if grav_state == "on" {
        eng.gravity_on();
    }

    // render loop
    while let Some(event) = window.next() {
        // this is for rendering
        if let Some(_) = event.render_args() {
            window.draw_2d(&event, |c, g, _| {
                // background color
                clear([0.5, 0.5, 0.5, 1.0], g);
                for circ in &eng.object_list {
                    // let cir = ellipse::circle(circ.x(), circ.y(), circ.r());
                    ellipse(circ.color(), circ.readjust(), c.transform, g);
                }
            });
        }
        // this is for updation of movement of shapes
        if let Some(u) = event.update_args() {
            // update position according to speed after every unit of time in simulation
            // u->update object ;;;; u.dt ----> time elapsed in simulation
            update(&mut eng, u.dt);
        }
    }
}
