extern crate hsl;
extern crate image;
use hsl::HSL;
use image::{GenericImageView, ImageBuffer, Pixel, Rgb};
use std::env;
use std::path::Path;

fn main() {
  let mut args: Vec<String> = env::args().collect();
  let file_names = args.split_off(1);

  let first_image = image::open(&file_names[0]).unwrap();
  let (max_width, max_height) = first_image.dimensions();
  let pixel_count = max_width * max_height;
  let mut hsl_pixels = Vec::with_capacity(pixel_count as usize);
  for _ in 0..pixel_count {
    hsl_pixels.push(HSL::default());
  }

  let file_count = file_names.len();

  for (i, file) in file_names.into_iter().enumerate() {
    println!("{}/{} [{}]", i + 1, file_count, file);
    let img = image::open(file).unwrap();
    let (own_width, own_height) = img.dimensions();
    let (width, height) = (own_width.max(max_width), own_height.max(max_height));
    for y in 0..height {
      for x in 0..width {
        let i = ((y * width) + x) as usize;
        let pixel = img.get_pixel(x, y).to_rgb().data;
        let new_hsl = HSL::from_rgb(&pixel);
        let mut hsl = hsl_pixels[i];
        hsl.h += new_hsl.h;
        hsl.s += new_hsl.s;
        hsl.l += new_hsl.l;
        hsl_pixels[i] = hsl;
      }
    }
  }

  let f = file_count as f64;
  for hsl in hsl_pixels.iter_mut() {
    hsl.h /= f;
    hsl.s /= f;
    hsl.l /= f;
  }

  let averaged_rgb_pixels: Vec<Rgb<u8>> = hsl_pixels
    .iter()
    .map(|hsl| HSL::to_rgb(&hsl))
    .map(|(r, g, b)| Rgb { data: [r, g, b] })
    .collect();

  let averaged_image = ImageBuffer::from_fn(max_width, max_height, |x, y| {
    let i = ((y * max_width) + x) as usize;
    averaged_rgb_pixels[i]
  });

  averaged_image.save(Path::new("./out.jpg")).unwrap();
}
