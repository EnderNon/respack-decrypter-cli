use clap::Parser;
use glob::glob;
use image::ImageBuffer;
use std::io::{BufReader, Read};
use std::{
    fs::{metadata, File},
    path::{Path, PathBuf},
};
use zune_png::{zune_core::options::DecoderOptions, PngDecoder};

#[derive(Parser, Debug)]
#[command(version, about, long_about = None)]
struct Args {
    /// Input path
    ///
    /// Include a file path or a directory path. Do not use any asterisks or wildcards.
    /// If the path includes spaces you may find it useful to use speechmarks (" ") around the path.
    #[arg(short, long)]
    path: PathBuf,

    //    /// Output path
    //    #[arg(short, long,default_value="")]
    //    out: PathBuf,
    /// Debug mode
    ///
    /// Either "true" or "false"
    #[arg(short, long, default_value_t = false)]
    debug: bool,
}

fn main() {
    // argument parsing yep
    let Args: Args = Args::parse();
    let pathfr = Args.path.clone();
    let debugmode = Args.debug;

    let mut filelist: Vec<PathBuf> = Vec::new();

    let pathtype = metadata(&pathfr).unwrap();
    if pathtype.is_file() {
        filelist.push(PathBuf::from(format!("{:?}", pathtype)));
        println!("input path FILE");
    } else if pathtype.is_dir() {
        println!("input path DIR");
        // parse file list if it's a dir
        for entry in glob(&format!("{}/**/*.png", pathfr.clone().display()))
            .expect("Failed to read glob pattern (you should panic)")
        {
            match entry {
                Ok(path) => {
                    // println!("{:?}", path.display());
                    filelist.push(path.display().to_string().parse().unwrap());
                }
                Err(e) => {
                    println!("Shell globbing error");
                }
            }
        }
    } else {
        panic!("that input is neither file nor dir and thats scary...")
    }

    //    if debugmode {
    //        for i in filelist.clone() {
    //            println!("List of files: {}",i.display())
    //        }
    //    }

    for thatpath in filelist {
        encoder(thatpath, debugmode)
    }
}

fn encoder(filepath: PathBuf, modeDebug: bool) {
    let file = File::open(&filepath).unwrap();
    let mut data = BufReader::new(file);
    let mut buf = Vec::new();
    data.read_to_end(&mut buf).unwrap();

    let mut decoder = PngDecoder::new_with_options(buf, DecoderOptions::new_cmd());
    decoder.decode_headers().unwrap();
    let dim = decoder.get_dimensions().unwrap();
    //dbg!(dim);
    let col = decoder.get_colorspace().unwrap();
    //dbg!(col);
    let data = decoder.decode_raw().unwrap();
    //dbg!(&data);
    let depth = decoder.get_depth().unwrap();

    if modeDebug {
        println!("Filepath: {:?}", filepath);
        println!("Dimensions: {:?}", dim);
        println!("Colorspace: {:?}", col);
        println!("BitDepth: {:?}", depth);
    }

    // TODO: figure out propper color checks rn we assume data is RGBA

    let out: ImageBuffer<image::Rgba<u8>, Vec<u8>> =
        ImageBuffer::from_raw(dim.0 as u32, dim.1 as u32, data).unwrap();

    out.save(filepath.clone()).unwrap();
    println!("wrote file {:?}\n",filepath)
}
