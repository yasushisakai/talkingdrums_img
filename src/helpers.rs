use image;

pub fn make_sample_image(file_name: &str) {

    const WIDTH: u32 = 150;
    const HEIGHT: u32 = 100;

    let mut imgbuf = image::ImageBuffer::new(WIDTH, HEIGHT);

    for (x, y, pixel) in imgbuf.enumerate_pixels_mut() {
        let i: u8 = ((x + y * WIDTH) % 255) as u8;
        
        *pixel = image::Rgb([i, i, i]);
    }
    image::ImageRgb8(imgbuf).save(file_name).unwrap();
    println!("created sample image")
}
