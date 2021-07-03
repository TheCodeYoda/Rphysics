use piston_window::*;
use rphysics::circle::*;
use rphysics::screen::*;

fn get_circles() -> Vec<Circle> {
    let circ_1 = Circle::new(50.0, 50.0, 50.0, 60.0, 0.0);
    let circ_2 = Circle::new(462.0, 462.0, 50.0, -60.0, 0.0);
    return vec![circ_1, circ_2];
}

fn update(circ_list: &mut Vec<Circle>, dt: f64, screen: &Screen) {
    for circ in circ_list {
        circ.update_pos(dt);
        circ.check_bounds(screen.width(), screen.height());
    }
}

///gets center of the circle
// fn get_center(x: f64, y: f64) -> (f64, f64) {}

// ellipse(x,y,halfwidth,halfheight)  x,y are points where the ellips touch x and y axes

fn main() {
    let screen = Screen::new(512.0, 512.0);

    // initializing piston window
    let mut window: PistonWindow = WindowSettings::new("Circles!", [512; 2]).build().unwrap();

    // list of circles to render
    let mut circ_list = get_circles();

    // render loop
    while let Some(e) = window.next() {
        // this is for rendering
        if let Some(_) = e.render_args() {
            window.draw_2d(&e, |c, g, _| {
                clear([0.5, 0.5, 0.5, 1.0], g);
                for circ in &circ_list {
                    // let cir = ellipse::circle(circ.x(), circ.y(), circ.r());
                    ellipse(
                        [1.0, 0.0, 0.0, 1.0], // red color
                        [
                            circ.x() - circ.r(),
                            circ.y() - circ.r(),
                            2.0 * circ.r(),
                            2.0 * circ.r(),
                        ],
                        c.transform,
                        g,
                    );
                }
            });
        }
        // this is for updation of movement of shapes
        if let Some(u) = e.update_args() {
            // update position according to speed after every unit of time in simulation
            // u->update object ;;;; u.dt ----> time elapsed in simulation
            update(&mut circ_list, u.dt, &screen);
        }
    }
}
