use std::{fs::File, io::BufWriter};

use num::Complex;
use png::{BitDepth, ColorType, Encoder};

const SCREEN_WIDTH: u32 = 1920 * 3;
const SCREEN_HEIGHT: u32 = 1080 * 3;
const MAX_ITERATIONS: usize = 2000;
const MAX_OUTPUT_NORM: f64 = 100.0;
const JULIA_CONSTANT: Complex<f64> = Complex::new(-0.512511498387847167, 0.521295573094847167);

fn main() -> Result<(), Box<dyn std::error::Error>> {
    // R, G, B for each pixel
    let mut image_data: Vec<u8> =
        Vec::with_capacity(SCREEN_WIDTH as usize * SCREEN_HEIGHT as usize * 3);

    let width = SCREEN_WIDTH as i64;
    let height = SCREEN_HEIGHT as i64;
    for imag in num::range(-height / 2, height / 2) {
        for real in num::range(-width / 2, width / 2) {
            let n = Complex::new(
                (real as f64) / ((width / 2) as f64),
                (imag as f64) / ((height / 2) as f64),
            );
            let iterations = iterate(n, JULIA_CONSTANT);
            image_data.extend_from_slice(&[255 - iterations.unwrap_or(0).min(255) as u8]);
        }
    }

    let mut encoder = Encoder::new(
        BufWriter::new(File::create("output.png")?),
        SCREEN_WIDTH,
        SCREEN_HEIGHT,
    );
    encoder.set_color(ColorType::Grayscale);
    encoder.set_depth(BitDepth::Eight);
    let mut writer = encoder.write_header().unwrap();
    writer.write_image_data(&image_data).unwrap();

    Ok(())
}

#[inline]
fn iterate(start: Complex<f64>, constant: Complex<f64>) -> Option<usize> {
    let mut output = start;
    let mut iterations = 0;
    while output.norm() < MAX_OUTPUT_NORM {
        output = output.powu(2) + constant;
        iterations += 1;

        if iterations > MAX_ITERATIONS {
            return None;
        }
    }

    Some(iterations)
}
