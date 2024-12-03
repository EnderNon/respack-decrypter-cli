use image::ImageBuffer;
use zune_png::{zune_core::options::DecoderOptions, PngDecoder};

fn main() {
    let data = include_bytes!("../inventory.png");

    let mut decoder = PngDecoder::new_with_options(data, DecoderOptions::new_cmd());

    decoder.decode_headers().unwrap();
    let dim = decoder.get_dimensions().unwrap();
    let col = decoder.get_colorspace().unwrap();
    let data = decoder.decode_raw().unwrap();

    // TODO: figure out propper color checks rn we assume data is RGBA

    let out: ImageBuffer<image::Rgba<u8>, Vec<u8>> =
        ImageBuffer::from_raw(dim.0 as u32, dim.1 as u32, data).unwrap();

    out.save("out.png").unwrap();
}
