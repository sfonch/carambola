use std::{env, io};
use image::ImageReader;
use linuxfb::Framebuffer;
use crossterm::{cursor::{Hide, Show}, event::{Event, KeyCode, read}, execute, terminal::{enable_raw_mode}};

// Note: Pixels are BGAX

fn main() {
    let framebuffer = Framebuffer::new("/dev/fb0").expect("Failed to create a framebuffer");
    let (screen_width, screen_height) = framebuffer.get_size();

    enable_raw_mode().unwrap();
    let mut stdout = io::stdout();
    execute!(stdout, Hide).unwrap();

    let mut data = framebuffer.map().unwrap();

    let img: Vec<u8> = ImageReader::open(format!("{}/hi.jpg", env::var("HOME").unwrap())) // This is for debug, so it will crash unless you have an image named "hi.jpg" in your home folder.
        .unwrap()
        .decode()
        .unwrap()
        .resize_exact(screen_width, screen_height, image::imageops::FilterType::Nearest)
        .to_rgb8()
        .pixels()
        .flat_map(|pix| [pix[2], pix[1], pix[0], 0])
        .collect();

    data.copy_from_slice(&img);

    loop {
        match read().unwrap() {
            Event::Key(key) => match key.code {
                KeyCode::Esc => {
                    execute!(stdout, Show).unwrap();
                    std::process::exit(1);
                },
                _ => {}
            }
            _ => {}
        }
    }
}
