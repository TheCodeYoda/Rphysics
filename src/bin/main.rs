use piston_window::*;
use rphysics::circle::*;
use rphysics::collison::*;
use rphysics::screen::*;

fn get_circles() -> Vec<Circle> {
    let circ_1 = Circle::new(50.0, 50.0, 50.0, 60.0, 60.0);
    let circ_2 = Circle::new(462.0, 462.0, 50.0, -60.0, -60.0);
    return vec![circ_1, circ_2];
}

fn update(circ_list: &mut Vec<Circle>, dt: f64, screen: &Screen) {
    for circ in circ_list {
        circ.update_pos(dt);
        circ.check_bounds(screen.width(), screen.height());
    }
}

fn check_collisions(circ_list: &mut Vec<Circle>) {
    let n = circ_list.len();
    for i in 0..n {
        for j in i + 1..n {
            if is_colliding(&circ_list[i], &circ_list[j]) {
                let arr = collide(&circ_list[i], &circ_list[j]);
                circ_list[i].vel_x = arr[0];
                circ_list[i].vel_y = arr[1];
                circ_list[j].vel_x = arr[2];
                circ_list[j].vel_y = arr[3];
            }
        }
    }
}
// ellipse(x,y,halfwidth,halfheight)

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
                    ellipse(circ.color(), circ.readjust(), c.transform, g);
                }
            });
        }
        // this is for updation of movement of shapes
        if let Some(u) = e.update_args() {
            // update position according to speed after every unit of time in simulation
            // u->update object ;;;; u.dt ----> time elapsed in simulation
            update(&mut circ_list, u.dt, &screen);
            check_collisions(&mut circ_list);
        }
    }
}
