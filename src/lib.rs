pub mod pixel_density_lut;
pub mod text_art;

#[cfg(test)]
mod tests {
    use std::fs;

    use image::{imageops::FilterType, io::Reader};
    use rusttype::{Font, Scale};

    use crate::{
        pixel_density_lut::{PixelDensityLut, PixelDensityLutBuilder},
        text_art::{string_to_image, TextArtStringEncoder},
    };

    #[test]
    fn test_index() {
        let brightness = 50.0;
        let color = 255.0;
        let total_pixels = 100.0;
        dbg!((brightness / total_pixels) * color);
    }

    #[test]
    fn test_pixel_density_lut() {
        const CHARS: &str = " !\"#$%&'()*+,-./0123456789:;<=>?@ABCDEFGHIJKLMNOPQRSTUVWXYZ[\\]^_`abcdefghijklmnopqrstuvwxyz{|}~";
        const FONT_BYTES: &[u8] = include_bytes!("../files/RobotoMono-Regular.ttf");
        const SCALE: f32 = 12.0;
        let font = Font::try_from_bytes(FONT_BYTES).unwrap();
        let scale = Scale::uniform(SCALE);
        let lut = PixelDensityLut::new(CHARS, &font, scale);
        println!("{:?}", &lut);
        println!("{}", '\u{1FB4D}');
    }

    #[test]
    fn test_text_art_encoder() {
        let pixel_density_lut = PixelDensityLut::default();
        let image = Reader::open("files/input/test-pattern.webp")
            .unwrap()
            .decode()
            .unwrap()
            .resize(300, 300, FilterType::Lanczos3);
        let text_art_encoder = TextArtStringEncoder::new(pixel_density_lut);
        let string = text_art_encoder.encode_alternating(&image);
        fs::write("files/output/test-pattern.txt", &string).unwrap();
    }

    #[test]
    fn test_image_to_string_image() {
        let scale = Scale::uniform(16.0);
        const FONT_BYTES: &[u8] = include_bytes!("../files/RobotoMono-Regular.ttf");
        let font = Font::try_from_bytes(FONT_BYTES).unwrap();
        let pixel_density_lut = PixelDensityLutBuilder::new()
            .with_font(font.clone())
            .with_scale(scale.clone())
            .build();
        let image = Reader::open("files/input/test-pattern.webp")
            .unwrap()
            .decode()
            .unwrap()
            .resize(300, 300, FilterType::Lanczos3);
        let text_art_encoder = TextArtStringEncoder::new(pixel_density_lut);
        let string = text_art_encoder.encode_alternating(&image);
        let image = string_to_image(font, scale, &string);
        let _ = image.save("files/output/test-pattern.jpg");
    }
}
