use freetype::Bitmap;
use image::{Rgb, RgbImage};

use crate::canvas::path_info::PathInfo;
use crate::canvas::text_info::TextInfo;
use crate::device::Device;
use crate::errors::PDFResult;
use crate::geom::rectangle::Rectangle;

pub struct ImageDevice {
    x_res: f64,
    y_res: f64,
    image: RgbImage,
}

impl ImageDevice {
    pub fn new(x_res: f64, y_res: f64) -> Self {
        let image = RgbImage::new(1, 1);
        ImageDevice {
            x_res,
            y_res,
            image,
        }
    }
}
impl ImageDevice {
    fn draw_char(&mut self, x: u32, y: u32, bitmap: &Bitmap) {
        let width = bitmap.width() as u32;
        let height = bitmap.rows() as u32;
        let buffer = bitmap.buffer();
        for i in 0..height {
            for j in 0..width {
                let pixel = buffer[(i * width + j) as usize];
                if pixel == 0 {
                    continue;
                }
                let rgb = Rgb([0, 0, 0]);
                self.image
                    .put_pixel(x + j, self.image.height() - y + i, rgb);
            }
        }
    }
}

impl Device for ImageDevice {
    fn begain_page(&mut self, media: &Rectangle, _crop: &Rectangle) {
        let width = self.x_res / 72.0 * media.width();
        let height = self.y_res / 72.0 * media.height();

        self.image = RgbImage::from_fn(width as u32, height as u32, |_, _| {
            image::Rgb([255, 255, 255])
        });
    }

    fn end_page(&mut self) {
        self.image.save(format!("page-{}.png", 2)).unwrap()
    }

    fn show_text(&mut self, mut textinfo: TextInfo) -> PDFResult<()> {
        // bitmap, x, y for every character
        // TODO Encoding PDFString-> Character Encoding, multi bytes may be one character
        // TODO color
        let _unicode = textinfo.get_unicode();

        let character = textinfo.decoded_character();
        let (x, y) = textinfo.position();
        //println!("{}, x:{},y:{}, ", unicode, x, y);
        let bbox = textinfo.bbox();
        let sx = self.image.width() as f64 / bbox.width();
        let sy = self.image.width() as f64 / bbox.height();
        let mut x = x * sx;
        let y = y * sx;
        let scale = f64::sqrt((sx * sx + sy * sy) / 2.0);
        for code in character {
            let w = textinfo.get_character_width(code);
            let bitmap = textinfo.get_glyph(code, scale);
            self.draw_char(x as u32, (y + bitmap.rows() as f64) as u32, &bitmap);
            x += w * sx;
        }

        Ok(())
    }

    fn paint_path(&mut self, _pathinfo: PathInfo) -> PDFResult<()> {
        Ok(())
    }
}
