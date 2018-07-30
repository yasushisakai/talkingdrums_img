extern crate csv;
extern crate image;

use image::{DynamicImage, GenericImage};
use std::env;
use std::fs::File;
use std::time::{SystemTime, UNIX_EPOCH};

mod helpers;

fn main() {

    // find the latest image
    let mut img = match image::open("current.png") {
        Ok(img) => img,
        Err (_e)=> {
            // find the original image
            match image::open("original.png") {
                Ok(img) => img,
                // if not make a sample image
                Err(_e) => {
                    helpers::make_sample_image("original.png");
                    image::open("original.png").unwrap()
                }
            }
        }
    };
    
    let (width, height) = img.dimensions();

    let mut args = env::args();

    let cmd = args.nth(1).unwrap();

    // FIXME: turn this into a match?
    
    if cmd == "size" {
        print!("{},{}", width, height);
    }

    if cmd == "get" {
        args.next().map(|value| {
            let index = value.parse::<u32>().unwrap();
            let coord = index_to_x_y(&img, index);
            let color = img.get_pixel(coord.0, coord.1).data[0];
            println!("{}", color)
        });
    }

    if cmd == "update" {
        let file = File::open("pixels.csv").unwrap();
        let mut reader = csv::Reader::from_reader(file);

        for result in reader.records() {
            let record = result.unwrap();
            let index = record[1].parse::<u32>().unwrap();
            let coord= index_to_x_y(&img, index);
            let value = record[2].parse::<u8>().unwrap();
            let pixel = image::Rgba([value, value, value, 255]);

            img.put_pixel(coord.0, coord.1, pixel);
        }
        let since_epoch = SystemTime::now().duration_since(UNIX_EPOCH).unwrap();
        img.save("current.png").unwrap();

        // saving a backup file
        let file_name = format!("archive_{}.png", since_epoch.as_secs());
        img.save(file_name).unwrap(); 
    }

}


struct Coordinates (u32, u32);

fn index_to_x_y (img: &DynamicImage, i: u32) -> Coordinates {
    let (width, height) = img.dimensions();
    let x = i % width; 
    let y = (i / width) / height;
    Coordinates(x, y)
}
