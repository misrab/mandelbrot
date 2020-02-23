extern crate num;
use num::Complex;

extern crate image;
use image::ColorType;
use image::png::PNGEncoder;
use std::fs::File;

use std::io::Result;

fn write_image(filename: &str, pixels: &[u8], bounds: (usize, usize)) 
    -> Result<()> {

  let output = File::create(filename)?;

  let encoder = PNGEncoder::new(output);
  encoder.encode(&pixels,
                 bounds.0 as u32,
                 bounds.1 as u32,
                 ColorType::Gray(8))?;

  Ok(())
}

fn render(pixels: &mut [u8],
          bounds: (usize, usize),
          upper_left: Complex<f64>,
          lower_right: Complex<f64>)
{
  assert!(pixels.len() == bounds.0 * bounds.1);

  for row in 0..bounds.1 {
    for col in 0..bounds.0 {
      let point = pixel_to_point(bounds, (col, row),
                                 upper_left, lower_right);

      pixels[row * bounds.0 + col] = 
          match escape_time(point, 255) {
            None => 0, // black if no escape
            Some(count) => 255 - count as u8
            // darker the longer it takes to escape
          }
    }
  }
}


fn pixel_to_point(bounds: (usize, usize),
                  pixel: (usize, usize),
                  upper_left: Complex<f64>,
                  lower_right: Complex<f64>)
   -> Complex<f64> 
{
  let (complex_w, complex_h) = (lower_right.re - upper_left.re,
                                upper_left.im - lower_right.im);

  Complex {
    re: upper_left.re + pixel.0 as f64 * complex_w / bounds.0 as f64,
    // multply by scaling factor to get complex units from pixel units
    im: upper_left.im - pixel.1 as f64 * complex_h / bounds.1 as f64
    // subtract since im goes higher, meaning lower in our image
    // since we're starting from top left to bottom right.
    // ! I think we could start from lower_left to upper_right and avoid
    // flipping the image vertically, which I think we're doing here.
  }
}

fn escape_time(c: Complex<f64>, limit: u32) -> Option<u32> {
  let mut z = Complex{ re: 0.0, im: 0.0 };

  for i in 0..limit {
    z = z*z + c;
    if z.norm_sqr() > 4.0 {
      return Some(i);
    }
  }
  
  // didn't escape
  None
}


fn main() {
    println!("Hello, world!");
    
    let bounds = (4000, 3000);
    let mut pixels = vec![0; bounds.0 * bounds.1];
    let upper_left = Complex{ re: -1.20, im: 0.35 };
    let lower_right = Complex{ re: -1.0, im: 0.20 };
    render(&mut pixels,
          bounds,
          upper_left,
          lower_right);
    write_image("moo.png", &pixels, bounds).expect("error writing PNG file");
}
