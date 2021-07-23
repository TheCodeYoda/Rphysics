use piston_window::*;
use std::env;
extern crate nalgebra_glm as glm;
use glm::*;

use rphysics::circle::*;
use rphysics::engine::*;
use rphysics::screen::*;

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

    let screen = Screen::new(1280.0, 720.0);

    // initializing piston window
    let mut window: PistonWindow =
        WindowSettings::new("Circles!", [screen.width(), screen.height()])
            .build()
            .unwrap();

    // engine object
    let mut eng = Engine::new(e, screen);

    // get object list
    eng.get_circles(n);

    if grav_state == "on" {
        eng.gravity_on();
    }

    // info to draw line on mouse clicks
    let mut start_point: [f64; 2] = [0.0, 0.0];
    let mut curr_pos: [f64; 2] = [0.0, 0.0];
    let mut state = 0;
    let mut prev_state = 0;

    // render loop
    while let Some(event) = window.next() {
        // this is for rendering
        if event.render_args().is_some() {
            window.draw_2d(&event, |c, g, _| {
                // background color
                clear([0.5, 0.5, 0.5, 1.0], g);
                for circ in &eng.object_list {
                    let transform = c
                        .transform
                        .trans(circ.x(), circ.y())
                        .rot_deg(circ.theta())
                        .trans(-(circ.x()), -(circ.y()));

                    ellipse(circ.color(), circ.readjust(), transform, g);
                    line_from_to(
                        [0.0, 0.0, 0.0, 1.0],
                        1.0,
                        [circ.x(), circ.y()],
                        [circ.x() + circ.r(), circ.y()],
                        transform,
                        g,
                    );

                    // draw impulse line
                    if state == 1 && prev_state == 1 {
                        line_from_to(
                            [0.0, 1.0, 0.0, 1.0],
                            2.0,
                            start_point,
                            curr_pos,
                            c.transform,
                            g,
                        );
                    }
                }
            });
        }

        // mouse_cursor_args -> returns position of mouse
        prev_state = state;
        if let Event::Input(input) = &event {
            if let Input::Button(button_args) = input {
                if let Button::Mouse(key) = button_args.button {
                    // println!("Key event: {:?} {:?} ", key, button_args.state);
                    if key == MouseButton::Left {
                        if button_args.state == ButtonState::Press {
                            state = 1;
                        } else {
                            state = 0;
                        }
                    }

                    let mut _selected_circ: Option<&mut Circle> = None;
                    if state == 1 && prev_state == 0 {
                        start_point = curr_pos;
                    } else if state == 0 && prev_state == 1 {
                        eng.mouse_impulse(
                            vec2(start_point[0], start_point[1]),
                            vec2(curr_pos[0], curr_pos[1]),
                        );
                    }
                }
            }
        }
        if let Some(pos) = event.mouse_cursor_args() {
            curr_pos = pos;
        }

        // this is for updation of movement of shapes
        if let Some(u) = event.update_args() {
            // update position according to speed after every unit of time in simulation
            // u->update object ;;;; u.dt ----> time elapsed in simulation
            update(&mut eng, u.dt);
        }
    }
}
