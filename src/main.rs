extern crate piston_window;

use piston_window::*;

struct Coordinate {
    x: f64,
    y: f64,
}

// Algo based on http://www.soc.napier.ac.uk/~andrew/hilbert.html
fn hilbert(x0: f64, y0: f64, xi: f64, yi: f64, xj: f64, yj: f64, n: i32) -> Vec<Coordinate> {
    let mut coordinate_vector: Vec<Coordinate> = Vec::new();
    match n {
        n if (n <= 0) => {
            let x = x0 + (xi + yi) / 2.0;
            let y = y0 + (xj + yj) / 2.0;

            coordinate_vector.push(Coordinate { x: x, y: y });
            coordinate_vector
        }
        _ => {
            coordinate_vector.append(&mut hilbert(x0, y0, 
                                                  yi / 2.0,
                                                  yj / 2.0,
                                                  xi / 2.0,
                                                  xj / 2.0,
                                                  n - 1));
            coordinate_vector.append(&mut hilbert(x0 + xi / 2.0,
                                                  y0 + xj / 2.0,
                                                  xi / 2.0,
                                                  xj / 2.0,
                                                  yi / 2.0,
                                                  yj / 2.0,
                                                  n - 1));
            coordinate_vector.append(&mut hilbert(x0 + xi / 2.0 + yi / 2.0,
                                                  y0 + xj / 2.0 + yj / 2.0,
                                                  xi / 2.0,
                                                  xj / 2.0,
                                                  yi / 2.0,
                                                  yj / 2.0,
                                                  n - 1));
            coordinate_vector.append(&mut hilbert(x0 + xi / 2.0 + yi,
                                                  y0 + xj / 2.0 + yj,
                                                  -yi / 2.0,
                                                  -yj / 2.0,
                                                  -xi / 2.0,
                                                  -xj / 2.0,
                                                  n - 1));
            coordinate_vector
        }
    }
}


fn create_hilbert_curve(coordinate_vector: &Vec<Coordinate>) -> Vec<[f64; 4]> {
    let mut line_vector = Vec::with_capacity(coordinate_vector.len());
    for i in 0..coordinate_vector.len() - 1 {
        line_vector.push([coordinate_vector[i].x,
                          coordinate_vector[i].y,
                          coordinate_vector[i + 1].x,
                          coordinate_vector[i + 1].y]);
    }
    line_vector
}

fn main() {
    let curve = hilbert(0.0, 0.0, 512.0, 0.0, 0.0, 512.0, 6);
    for point in curve.iter() {
        println!("{} {} 0", point.x, point.y);
    }

    let mut window: PistonWindow = WindowSettings::new("Hilbert!", [640, 480])
        .exit_on_esc(true)
        .build()
        .unwrap();
    while let Some(e) = window.next() {
        window.draw_2d(&e, |c, g| {
            clear([1.0; 4], g);

            for i in create_hilbert_curve(&curve) {
                line([0.0, 0.0, 0.0, 1.0], // black
                     2.0, // radius of line
                     i, // [x0, y0, x1,y1] coordinates of line
                     c.transform,
                     g);
            }
        });
    }

}
