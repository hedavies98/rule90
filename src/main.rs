use bmp::{px, Image, Pixel};
use rand::{thread_rng, Rng};
use std::io::{self, Write};

// Width and height of the final image
const WIDTH: usize = 8000;
const HEIGHT: u32 = 4000;

fn main() {
    let mut configuration = initialise_configuration();

    let mut img = Image::new(WIDTH as u32, HEIGHT);

    // (from colour, to colour)
    let red = (207_f64, 0_f64);
    let green = (2_f64, 54_f64);
    let blue = (108_f64, 163_f64);

    for y in 1..HEIGHT {
        let y_progress: f64 = y as f64 / HEIGHT as f64;
        for x in 1..WIDTH {
            if configuration[x] {
                img.set_pixel(
                    x as u32,
                    y,
                    px!(
                        red.0 * y_progress + red.1 * (1_f64 - y_progress),
                        green.0 * y_progress + green.1 * (1_f64 - y_progress),
                        blue.0 * y_progress + blue.1 * (1_f64 - y_progress)
                    ),
                );
            } else {
                img.set_pixel(x as u32, y, px!(0, 0, 0));
            }
        }
        configuration = next_iteration(configuration);
    }

    let _ = img.save("img.bmp");
}

fn initialise_configuration() -> [bool; WIDTH] {
    let mut rng = thread_rng();
    let mut configuration = [false; WIDTH];
    print!("Please enter probability of a white cell on row 1: ");
    io::stdout().flush().unwrap();

    let mut probability = String::new();
    io::stdin()
        .read_line(&mut probability)
        .expect("Failed to read line!");
    let probability: f64 = probability.trim().parse().expect("Please enter a number");

    for i in 1..WIDTH {
        configuration[i] = rng.gen_bool(probability);
    }

    configuration
}

fn next_iteration(configuration: [bool; WIDTH]) -> [bool; WIDTH] {
    let mut next_configuration = [false; WIDTH];

    for i in 0..configuration.len() {
        if i == 0 {
            next_configuration[i] = false ^ configuration[i + 1];
        } else if i >= configuration.len() - 1 {
            next_configuration[i] = configuration[i - 1] ^ false;
        } else {
            next_configuration[i] = configuration[i - 1] ^ configuration[i + 1];
        }
    }

    next_configuration
}
