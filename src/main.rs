mod vector3;
mod color;
use crate::vector3::Color;
use crate::color::write_color;
fn main() {
    
    let img_width = 256;
    let img_height = 256;

    eprintln!("Generating image..");
    println!("P3\n{} {} \n255\n", img_width, img_height);

    for i in (0..img_height).rev() {
        for j in 0..img_width {
            let pixel_color = Color::new(i as f64 / (img_width-1) as f64, j as f64 / (img_height-1) as f64, 0.25);
            write_color(&pixel_color);
        }
    }

    eprintln!("Image generation done, file has been created");

}
