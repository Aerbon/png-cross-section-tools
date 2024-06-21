use std::fs::File;
use rayon::prelude::*;

const SOLID_THRESHOLD: u8 = 0x40;
// const IMAGE_PATH: &str = "samples/p1.png";
const IMAGE_PATH: &str = "samples/4096x4096_circle_3999.png";

// const PIXEL_LENGTH_MM: f64 = 0.1;
const PIXEL_LENGTH_MM: f64 = 2f64 / 3999f64;
// const PIXEL_LENGTH_MM: f64 = 35f64 / (486 - 21) as f64;

fn main() {
    // Decode PNG
    let decoder = png::Decoder::new(
        File::open(IMAGE_PATH).unwrap()
    );
    let mut reader = decoder.read_info().unwrap();
    let mut buf = vec![0; reader.output_buffer_size()];
    let info = reader.next_frame(&mut buf).unwrap();
    let bytes = &buf[..info.buffer_size()];
    let (w, h) = (info.width, info.height);
    // Debug
    println!("Image Width: {} px\nImage Height: {} px",w,h);
    // Calculate CoS
    let (
        area,
        mut center_y,
        mut center_x
    ) =
    (0..(h as usize)).into_par_iter().map(|y| {
        let offset = y * w as usize;
        let mut sum = 0i64;
        let mut xweight = 0i64;
        for x in 0..w {
            let offset = (x as usize + offset) * 3;
            if bytes[offset] >= SOLID_THRESHOLD {
                sum += 1;
                xweight += 1 + 2 * x as i64;
            }
        }
        let yweight = sum * (1 + 2 * y as i64);
        (sum, xweight, yweight)
    }).reduce(
        ||{(0, 0, 0)},
        |(sum, xweight, yweight), (isum, ixweight, iyweight)| {
        (
            sum + isum,
            xweight + ixweight,
            yweight + iyweight
        )
    });
    assert!(area > 0, "Image is empty!");
    println!("Surface Area: {} px", area);
    center_x = center_x / (area * 2);
    center_y = center_y / (area * 2);
    println!("Centroid: ({}, {}) px", center_x, center_y);
    // Calculate moments of inertia
    let (
        moment_y,
        moment_x
    ) =
    (0..(h as usize)).into_par_iter().map(|y: usize| {
        let offset = y * w as usize;
        let mut sum = 0i64;
        let mut xweight = 0i64;
        for x in 0..w {
            let offset = (x as usize + offset) * 3;
            if bytes[offset] >= SOLID_THRESHOLD {
                sum += 1;
                xweight += (x as i64 - center_x).pow(2);
            }
        }
        let yweight = sum * (y as i64 - center_y).pow(2);
        (xweight, yweight)
    }).reduce(
        ||{(0, 0)},
        |(xweight, yweight), (ixweight, iyweight)| {
        (
            xweight + ixweight,
            yweight + iyweight
        )
    });
    // Conversion from pixels^4 to mm^4.
    let moment_x_mm = moment_x as f64 * PIXEL_LENGTH_MM.powi(4);
    let moment_y_mm = moment_y as f64 * PIXEL_LENGTH_MM.powi(4);
    println!(
        "Area Moments Of Inertia:\nIx = {:.4} mm^4\nIy = {:.4} mm^4",
        moment_x_mm, moment_y_mm
    );
}
