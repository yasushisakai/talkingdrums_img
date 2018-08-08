extern crate csv;
extern crate image;

use image::{DynamicImage, GenericImage};
use std::env;
use std::io::prelude::*;
use std::fs::File;
use std::time::{SystemTime, UNIX_EPOCH};

mod helpers;

fn main() {

    // find the latest image
    let mut img = match image::open("current.png") {
        Ok(img) => img,
        Err (_e)=> {
            // find the original image
            match image::open("image.png") {
                Ok(img) => img,
                // if not make a sample image
                Err(_e) => {
                    helpers::make_sample_image("image.png");
                    image::open("image.png").unwrap()
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

    if cmd == "cheatsheet" {
        let mut color_table = ColorTable::new();

        let mut byte: u8 = 0;
        let mut count = 0;
        let mut bytes: Vec<u8> = Vec::new();
        for x in 0..width {
            for y in 0..height {
                let c = img.get_pixel(x, y).data;
                let c: Color = Color::new(c[0], c[1], c[2]);
                let i = color_table.push_if_unique(c).unwrap_or_else(||(color_table.colors.len() as u8));
                    
                if count % 2 == 1 {
                    let new_byte = i | byte << 4;
                    // println!("{:04b}+{:04b}={:08b}", byte, i, new_byte);
                    bytes.push(new_byte); 
                } else {
                    byte = i;
                }
                count += 1;
                // println!("{}", i);
            }
        }

        let mut overall = color_table.file_header();
        overall.extend_from_slice(bytes.as_ref());

        match File::create("image.pixels"){
            Ok(mut buf)=> {
            buf.write(overall.as_ref()).unwrap(); 
            }
            Err(e)=>{println!("error: {}",e)}
        }


    }
}



struct ColorTable{
    colors:Vec<Color>
}

impl ColorTable{
    pub fn new() -> ColorTable{
        ColorTable{colors:Vec::new()}
    }

    pub fn has(&self, new_color: &Color) -> Option<u8> {

        // FIXME: enumerate
        let mut cnt = 0;
        for c in &self.colors {
            if new_color == c {
                return Some(cnt)
            }
            cnt += 1;
        };
        None
    }

    pub fn push(&mut self, new_color: Color) {
        self.colors.push(new_color)
    }

    pub fn push_if_unique(&mut self, new_color: Color) -> Option<u8>{
        let is_unique = self.has(&new_color);
        
        match is_unique{
            Some(i) => Some(i),
            None  => {
                // println!("found new color {:?}", &new_color);
                self.push(new_color);
                None
            }
        }
    }

    pub fn file_header(&self) -> Vec<u8> {
        let mut color_vec:Vec<u8> = Vec::new();
        
        let new_colors = self.colors.clone();
        for c in new_colors {
            color_vec.extend_from_slice(&c.to_u8().as_ref());
        }
        color_vec
    } 

}

#[derive(Debug, Eq, PartialEq, Copy, Clone)]
struct Color(u8,u8,u8);

impl Color {
    pub fn new(r: u8, g:u8, b:u8) -> Color {
        Color(r, g, b)
    }

    pub fn to_u8(&self) -> [u8;3] {
        [self.0, self.1, self.2]
    }
}

struct Coordinates (u32, u32);

fn index_to_x_y (img: &DynamicImage, i: u32) -> Coordinates {
    let (width, height) = img.dimensions();
    let x = i % width; 
    let y = (i / width) / height;
    Coordinates(x, y)
}
