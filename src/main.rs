use image::png::PNGEncoder;
use image::ColorType;
use num::Complex;
use rayon::prelude::*;
use std::env;
use std::fs::File;
use std::str::FromStr;

fn escape_time(c: Complex<f64>, limit: usize) -> Option<usize> {
    let mut z = Complex { re: 0.0, im: 0.0 };
    for i in 0..limit {
        if z.norm_sqr() >= 4.0 {
            return Some(i);
        }
        z = z * z + c;
    }
    None
}

fn parse_pair<T: FromStr>(s: &str, separator: char) -> Option<(T, T)> {
    match s.find(separator) {
        None => None,
        Some(index) => match (T::from_str(&s[..index]), T::from_str(&s[index + 1..])) {
            (Ok(l), Ok(r)) => Some((l, r)),
            _ => None,
        },
    }
}

#[test]
fn test_parse_pair() {
    assert_eq!(parse_pair::<i32>("10,", 'x'), None);
    assert_eq!(parse_pair::<f64>("20.0x30.0", 'x'), Some((20.0, 30.0)));
    assert_eq!(parse_pair::<i32>("100,20", ','), Some((100, 20)));
    assert_eq!(parse_pair::<i32>("100,20", 'x'), None);
}

fn parse_complex(s: &str) -> Option<Complex<f64>> {
    match parse_pair(s, ',') {
        Some((re, im)) => Some(Complex { re, im }),
        None => None,
    }
}

#[test]
fn test_parse_complex() {
    assert_eq!(parse_complex("3.0,1.0"), Some(Complex { re: 3.0, im: 1.0 }));
    assert_eq!(parse_complex(",-1.5"), None);
}

fn pixel_to_point(
    bounds: (usize, usize),
    pixel: (usize, usize),
    upper_left: Complex<f64>,
    lower_right: Complex<f64>,
) -> Complex<f64> {
    let (width, height) = (
        lower_right.re - upper_left.re,
        upper_left.im - lower_right.im,
    );
    Complex {
        re: upper_left.re + pixel.0 as f64 * width / bounds.0 as f64,
        im: upper_left.im - pixel.1 as f64 * height / bounds.1 as f64,
    }
}

#[test]
fn test_pixel_to_point() {
    assert_eq!(
        pixel_to_point(
            (100, 200),
            (25, 175),
            Complex { re: -1.0, im: 1.0 },
            Complex { re: 1.0, im: -1.0 }
        ),
        Complex {
            re: -0.5,
            im: -0.75
        }
    );
}

fn write_image(
    filename: &str,
    pixels: &[u8],
    bounds: (usize, usize),
) -> Result<(), std::io::Error> {
    let output = File::create(filename)?;
    let encoder = PNGEncoder::new(output);
    encoder.encode(
        &pixels,
        bounds.0 as u32,
        bounds.1 as u32,
        ColorType::Gray(8),
    )?;
    Ok(())
}

fn main() {
    let args: Vec<String> = env::args().collect();
    if args.len() != 5 {
        eprintln!("Usage: {} FILE PIXELS, UPPERLEFT LOWERRIGHT", args[0]);
        eprintln!("Example {} model.png 1000x750 -1.20,0.35 -1,0.20", args[0]);
        std::process::exit(1);
    }
    let bounds = parse_pair::<usize>(&args[2], 'x').expect("error parsing image dimensions");
    let upper_left = parse_complex(&args[3]).expect("error parsing upper left");
    let lower_right = parse_complex(&args[4]).expect("error parsing lower right");
    let mut pixels = vec![0; bounds.0 * bounds.1];
    pixels
        .par_iter_mut()
        .enumerate()
        .for_each(|(index, value)| {
            let (row, col) = (index / bounds.0, index % bounds.0);
            let point = pixel_to_point(bounds, (col, row), upper_left, lower_right);
            *value = match escape_time(point, 255) {
                None => 0,
                Some(count) => 255 - count as u8,
            };
        });
    write_image(&args[1], &pixels, bounds).expect("error writing PNG file");
}
