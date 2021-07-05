use piston_window::*;
// use std::f64::consts::PI;
use rand::*;

use rphysics::circle::*;
use rphysics::collison::*;
use rphysics::gravity::*;
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

fn get_circles(screen: &Screen) -> Vec<Circle> {
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
    // let circ_1 = Circle::new(50.0,50.0,50.0,60.0,0.0);
    // let circ_2 = Circle::new(256.0+25.0,462.0,50.0,0.0,-60.0);
    // return vec![circ_1];

    // --------------------------random testing -------------------------------------
    let mut list: Vec<Circle> = Vec::new();
    let n = 20;
    for _i in 0..n {
        if let Some(circ) = get_circle(&list, &screen) {
            list.push(circ);
        }
    }
    return list;
}

fn update(circ_list: &mut Vec<Circle>, dt: f64, screen: &Screen) {
    let mut grav = Gravity::new();
    let e = 1.0;
    grav.off();
    for circ in circ_list {
        circ.update_pos(dt, &Gravity::new(), &screen);
        circ.check_bounds(screen, e);
    }
}

fn check_collisions(circ_list: &mut Vec<Circle>) {
    let n = circ_list.len();
    let e = 1.0;
    // let mut res = Vec::new();
    for i in 0..n {
        for j in i + 1..n {
            if let Some(p) = circ_list[i].is_colliding(&circ_list[j]) {
                // resolve static collison for ith and jth circle
                circ_list[i].point = p.0;
                circ_list[j].point = p.1;
                // collide and readjust velocit for ith and jth sphere
                let vel = circ_list[i].collide(&circ_list[j], e);
                circ_list[i].v = vel.0;
                circ_list[j].v = vel.1;
            }
        }
    }
}
// ellipse(x,y,halfwidth,halfheight)

fn main() {
    let screen = Screen::new(512.0, 512.0);

    // initializing piston window
    let mut window: PistonWindow =
        WindowSettings::new("Circles!", [screen.width(), screen.height()])
            .build()
            .unwrap();

    // list of circles to render
    let mut circ_list = get_circles(&screen);

    // render loop
    while let Some(e) = window.next() {
        // this is for rendering
        if let Some(_) = e.render_args() {
            window.draw_2d(&e, |c, g, _| {
                // background color
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
