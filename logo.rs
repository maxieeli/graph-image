/// https://github.com/image-rs/image#generating-fractals

fn main() {
    let imgx = 800;
    let imgy = 800;
  
    let scalex = 3.0 / imgx as f32;
    let scaley = 3.0 / imgy as f32;
  
    // Create a new ImgBuf with width: imgx and height: imgy
    let mut imgbuf = image::ImageBuffer::new(imgx, imgy);
  
    // Iterate over the coordinates and pixels of the image
    for (x, y, pixel) in imgbuf.enumerate_pixels_mut() {
        let r = (0.3 * x as f32) as u8;
        let b = (0.3 * y as f32) as u8;
        *pixel = image::Rgba([r, 0, b, 255]);
    }
  
    let center_x = imgx as f32 / 2.0;
    let center_y = imgy as f32 / 2.0;
  
    // A redundant loop to demonstrate reading image data
    for x in 0..imgx {
        for y in 0..imgy {
            let cx = y as f32 * scalex - 1.5;
            let cy = x as f32 * scaley - 1.5;
    
            let c = num_complex::Complex::new(-0.4, 0.6);
            let mut z = num_complex::Complex::new(cx, cy);
    
            let mut i = 0;
            while i < 255 && z.norm() <= 2.0 {
                z = z * z + c;
                i += 1;
            }
  
            let pixel = imgbuf.get_pixel_mut(x, y);
            let image::Rgba(data) = *pixel;
            *pixel = image::Rgba([
                data[0],
                i as u8,
                data[2],
                if f32::hypot(x as f32 - center_x, y as f32 - center_y) < 360.0 {
                    255
                } else {
                    0
                },
            ]);
        }
    }
}
