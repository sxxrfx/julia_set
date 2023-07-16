#![allow(dead_code)]
use clap::Parser;
use image::io::Reader as ImageReader;
use image::{ImageBuffer, ImageFormat, Rgb};
use indicatif::ParallelProgressIterator;
use indicatif::ProgressIterator;
use num::Complex;
use rayon::prelude::*;
use std::fs::File;

#[derive(Parser)]
#[command(about, long_about=None)]
struct Args {
    #[clap(long, short = 'w', default_value_t = 400)]
    width: u32,
    #[clap(long, short = 'h', default_value_t = 300)]
    height: u32,
    #[clap(long, short = 'n', default_value_t = 100)]
    no_of_images: u32,
    #[clap(long, short = 's', default_value_t = 0.001)]
    step: f64,
    #[clap(long, short = 'x', default_value_t=-0.87)]
    x_initial: f64,
    #[clap(long, short = 'y', default_value_t = 0.1)]
    y_initial: f64,
}

fn main() {
    let Args {
        width,
        height,
        no_of_images,
        step,
        x_initial,
        y_initial,
    } = Args::parse();

    println!("Rendering imgs...");
    let _ = (0..no_of_images)
        .into_par_iter()
        .progress()
        .map(|i| {
            let img = render_julia(width, height, x_initial + step * i as f64, y_initial);
            let _ = img.save_with_format(format!("./imgs/img{}.png", i), ImageFormat::Png);
        })
        .count();

    let mut image = File::create("output.gif").unwrap();
    let mut encoder = gif::Encoder::new(&mut image, width as u16, height as u16, &[]).unwrap();
    encoder.set_repeat(gif::Repeat::Infinite).unwrap();

    println!("Converting imgs to gif...");
    let _ = (0..no_of_images)
        .progress()
        .map(|i| {
            let img = ImageReader::open(format!("./imgs/img{}.png", i))
                .unwrap()
                .decode()
                .unwrap();
            let frame = gif::Frame::from_rgb(width as u16, height as u16, img.as_bytes());
            encoder.write_frame(&frame).unwrap();
        })
        .count();
}

fn render_julia(width: u32, height: u32, re: f64, im: f64) -> ImageBuffer<Rgb<u8>, Vec<u8>> {
    let r = calculate_r(re, im);
    let c = Complex::new(re, im);
    let scale_x = 3.0 / width as f64;
    let scale_y = 3.0 / height as f64;
    ImageBuffer::from_fn(width, height, |x, y| {
        let cx = x as f64 * scale_x - 1.5;
        let cy = y as f64 * scale_y - 1.5;
        let value = julia(c, cx, cy, r);
        Rgb([value, value, value])
    })
}

fn calculate_r(cx: f64, cy: f64) -> f64 {
    let c = (cx * cx + cy * cy).sqrt();
    (1.0 + (1.0 + 4.0 * c).sqrt()) / 2.0
}

fn julia(c: Complex<f64>, x: f64, y: f64, r: f64) -> u8 {
    let mut z = Complex::new(x, y);

    for i in 0..255 {
        if z.norm() > r {
            return (255 - i) as u8;
        }
        z = z * z + c;
    }
    // 255
    0
}
