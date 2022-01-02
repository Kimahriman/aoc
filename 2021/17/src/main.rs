use std::fs;

fn main() {
    let x_range = 169..=206;
    let y_range = -108..=-68;

    // let x_range = 20..=30;
    // let y_range = -10..=-5;

    let mut x_cands = Vec::<u32>::new();

    for x in 1..=*x_range.end() {
        let mut vel = x;
        let mut pos = 0;
        while vel > 0 {
            pos += vel;
            vel -= 1;
            if x_range.contains(&pos) {
                println!("Found x candidate {}", x);
                x_cands.push(x);
                break;
            }
        }
    }
    let mut y_max = 0;
    let mut possibilities = 0;
    for x in x_cands.iter() {
        for y in *y_range.start()..10000 {
            let mut vel = (*x, y);
            let mut pos = (0, 0);
            let mut local_y_max = 0;
            while pos.0 <= *x_range.end() && pos.1 >= *y_range.start() {
                if pos.1 > local_y_max {
                    local_y_max = pos.1;
                }
                if x_range.contains(&pos.0) && y_range.contains(&pos.1) {
                    possibilities += 1;
                    println!("Found a match with max Y {}, {}: {} at {} {}", x, y, local_y_max, pos.0, pos.1);
                    if local_y_max >= y_max {
                        y_max = local_y_max;
                    }
                    break;
                }
                pos.0 += vel.0;
                pos.1 += vel.1;
                if vel.0 > 0 {
                    vel.0 -= 1;
                }
                vel.1 -= 1;
            }
        }
    }

    println!("Max Y: {}. Possibilities: {}", y_max, possibilities);
}
