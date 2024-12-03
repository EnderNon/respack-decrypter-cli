use image::ImageBuffer;
use zune_png::{zune_core::options::DecoderOptions, PngDecoder};
use clap::Parser;
use glob::glob;
use std::fs::metadata;
use std::path::{Path, PathBuf};

#[derive(Parser, Debug)]
#[command(version, about, long_about = None)]
struct Args {
    /// Input path
    #[arg(short, long)]
    path: Vec<PathBuf>,

    /// Output path
    #[arg(short, long)]
    out: Option<PathBuf>,

    /// Debug mode
    #[arg(short, long, default_value_t=false)]
    debug: bool,

}

fn main() {

    // argument parsing yep
    let Args: Args = Args::parse();
    let pathfr = Args.path.clone();
    let outfr = match Args.out {
        Some(realoutpath) => {
            realoutpath
        },
        None => {
            Args.path
        }
    };
    let realdebugmode = Args.debug;

    let mut filelist: Vec<String> = Vec::new();


    let pathtype = metadata(&pathfr).unwrap();
    if pathtype.is_file() {

    }
//    else if pathtype.is_dir() {
//        // parse file list if it's a dir
//        for entry in glob(&format!("{}/**/*.png", pathfr.clone().display())).expect("Failed to read glob pattern (you should panic)") {
//            match entry {
//                Ok(path) => {
//                    // println!("{:?}", path.display());
//                    filelist.push(path.display().to_string());
//                },
//                Err(e) => {
//                    println!("Shell globbing error");
//                }
//            }
//        }
//    }
//    else {
//        panic!("")
//    }
//


    if realdebugmode {
        for i in filelist {
            println!("{i}")
        }
    }


    let data = include_bytes!("../inventory.png");




    let mut decoder = PngDecoder::new_with_options(data, DecoderOptions::new_cmd());

    decoder.decode_headers().unwrap();
    let dim = decoder.get_dimensions().unwrap();
    //dbg!(dim);
    let col = decoder.get_colorspace().unwrap();
    //dbg!(col);
    let data = decoder.decode_raw().unwrap();
    //dbg!(&data);
    let depth = decoder.get_depth().unwrap();

    // TODO: figure out propper color checks rn we assume data is RGBA

    let out: ImageBuffer<image::Rgba<u8>, Vec<u8>> =
        ImageBuffer::from_raw(dim.0 as u32, dim.1 as u32, data).unwrap();

    out.save("out.png").unwrap();
}
