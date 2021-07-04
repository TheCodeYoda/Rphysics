use piston_window::*;
//use std::f64::consts::PI;
use rand::*;

use rphysics::circle::*;
use rphysics::collison::*;
use rphysics::screen::*;


fn get_circle(list:&Vec<Circle>) -> Option<Circle> {
    let mut tries = 100000;
    while tries>0 {
          let mut rng = rand::thread_rng();
          let circ = Circle::new(rng.gen_range(50.0,462.0),rng.gen_range(50.0,462.0),rng.gen_range(25.0,60.0),rng.gen_range(-70.0,70.0),rng.gen_range(-70.0,70.0));
          let mut flag = 1;
          for sample in list {
              if is_colliding(&circ,&sample) {
                  flag = 0;
                  break;
              }
            }
         if flag==1 {
             return Some(circ);
         }
        tries -= 1;
     }
    return None;
}

fn get_circles() -> Vec<Circle> {
    // let w = 512.0;
    // let h = 512.0;
    // let v = 60.0;
    // let circ_1 = Circle::new(256.0, 0.0, 50.0, 0.0, 60.0);
    // let x = w/2.0 -((w/2.0)*(PI/3.0).sin());
    // let y = ((w/2.0*(PI/3.0).cos())) + h/2.0;
    // let circ_2 = Circle::new(x, y, 50.0,v*(PI/6.0).cos(), -v*(PI/6.0).sin());
    // let x = w/2.0 +((w/2.0)*(PI/3.0).sin());
    // let circ_3 = Circle::new(x, y, 50.0, -v*(PI/6.0).cos(), -v*(PI/6.0).sin());
    // return vec![circ_1, circ_2,circ_3];
    // let circ_1 = Circle::new(256.0-25.0,50.0,50.0,0.0,60.0);
    // let circ_2 = Circle::new(256.0+25.0,462.0,50.0,0.0,-60.0);
    // return vec![circ_1, circ_2];
    let mut list:Vec<Circle> = Vec::new();
    let n = 7;
    for _i in 0..n {
        if let Some(circ) = get_circle(&list) {
            list.push(circ);
        }
    }
    return list;
}

fn update(circ_list: &mut Vec<Circle>, dt: f64, screen: &Screen) {
    for circ in circ_list {
        circ.update_pos(dt);
        circ.check_bounds(screen.width(), screen.height());
    }
}

fn check_collisions(circ_list: &mut Vec<Circle>) {
    let n = circ_list.len();
    // let mut res = Vec::new();
    for i in 0..n {
        for j in i + 1..n {
            if is_colliding(&circ_list[i], &circ_list[j]) {
                // let (a, b) = circ_list.split_at_mut(i); // Returns (&mut [1], &mut [2, 3])
                let arr = collide(&circ_list[i], &circ_list[j]);
                circ_list[i].v[0] = arr.0.0;
                circ_list[i].v[1] = arr.0.1;
                circ_list[j].v[0] = arr.1.0;
                circ_list[j].v[1] = arr.1.1;
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
