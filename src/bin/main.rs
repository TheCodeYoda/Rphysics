use piston_window::*;
// use std::f64::consts::PI;
use rand::*;
use std::env;

use rphysics::circle::*;
use rphysics::collison::*;
use rphysics::engine::*;
use rphysics::screen::*;

/// randomly spawns Non overlapping circles in the Screen
fn get_circle(list: &mut Vec<Circle>, screen: &Screen) -> Option<Circle> {
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

        let mut circ = Circle::new(
            rng.gen_range(0.0, width),
            rng.gen_range(0.0, height),
            rng.gen_range(min_radius, max_radius),
            rng.gen_range(min_vel, max_vel),
            rng.gen_range(min_vel, max_vel),
        );
        let mut flag = 1;
        for sample in list.iter_mut() {
            if circ.is_colliding(sample) {
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
    // let circ_1 = Circle::new(50.0, 50.0, 50.0, 100.0, 0.0);
    // let circ_2 = Circle::new(462.0, 50.0, 50.0, -100.0, 0.0);
    // engine.object_list = vec![circ_1, circ_2];

    // --------------------------random testing -------------------------------------
    for _i in 0..n {
        if let Some(circ) = get_circle(&mut engine.object_list, &engine.screen) {
            engine.object_list.push(circ);
        }
    }
}

fn update(engine: &mut Engine, dt: f64) {
    engine.resolve_collisons(dt);
    engine.resolve_wall_collisions(dt);
    engine.update_pos(dt);
}

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

    // info to draw line on mouse clicks
    let mut start_point: [f64; 2] = [0.0, 0.0];
    let mut end_point: [f64; 2] = [0.0, 0.0];
    let mut state = 0;
    let mut prev_state = 0;

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
                    line_from_to(
                        [0.0, 0.0, 0.0, 1.0],
                        1.0,
                        [circ.x(), circ.y()],
                        [circ.x() + circ.r(), circ.y()],
                        c.transform,
                        g,
                    );
                    if state == 1 {
                        println!("{:?} {:?}", start_point, end_point);
                        line_from_to(
                            [0.0, 1.0, 0.0, 1.0],
                            2.0,
                            start_point,
                            end_point,
                            c.transform,
                            g,
                        );
                    }
                }
            });
        }

        // mouse_cursor_args -> returns position of mouse
        if let Event::Input(input) = &event {
            if let Input::Button(button_args) = input {
                if let Button::Mouse(key) = button_args.button {
                    // println!("Key event: {:?} {:?} ", key, button_args.state);
                    if key == MouseButton::Left {
                        prev_state = state;
                        if button_args.state == ButtonState::Press {
                            state = 1;
                        } else {
                            state = 0;
                        }
                    }
                }
            }
        }

        if let Some(pos) = event.mouse_cursor_args() {
            if state == 1 && prev_state == 0 {
                start_point = pos;
            }

            if state == 0 && prev_state == 1 {
                end_point = pos;
            }
        }

        // this is for updation of movement of shapes
        if let Some(u) = event.update_args() {
            // update position according to speed after every unit of time in simulation
            // u->update object ;;;; u.dt ----> time elapsed in simulation
            update(&mut eng, u.dt);
        }
    }
}
