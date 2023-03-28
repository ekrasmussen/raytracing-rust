fn main() {
    
    let img_width = 256;
    let img_height = 256;

    eprintln!("Generating image..");
    println!("P3\n{} {} \n255\n", img_width, img_height);

    for i in (0..img_height).rev() {
        for j in 0..img_width {
            let r = i as f64 / (img_width-1) as f64;
            let g = j as f64 / (img_height-1) as f64;
            let b = 0.25;
        
            let ir = (255.999 * r) as i32;
            let ig = (255.999 * g) as i32;
            let ib = (255.999 * b) as i32;

            println!("{} {} {}\n", ir, ig, ib);
        }
    }

    eprintln!("Image generation done, file has been created");

}
