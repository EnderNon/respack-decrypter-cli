mod idk;
use clap::Parser;
use glob::glob;
use std::io::{Read};
use std::{
    fs::{metadata},
    path::{PathBuf},
};

#[derive(Parser, Debug)]
#[command(version, about, long_about = None)]
struct Args {
    /// Input path
    ///
    /// Include a file path or a directory path. Do not use any asterisks or wildcards.
    /// Do not use multiple paths in one command.
    /// If the path includes spaces you may find it useful to use speechmarks (" ") around the path.
    #[arg(short, long)]
    path: PathBuf,

    //    /// Output path
    //    #[arg(short, long,default_value="")]
    //    out: PathBuf,
    /// Optional Debug mode
    ///
    /// Either "true" or "false" in lowercase.
    #[arg(short, long, default_value_t = false)]
    debug: bool,
}

fn main() {
    // argument parsing yep
    let Args: Args = Args::parse();
    let mut pathfr = Args.path.clone();
    let debugmode = Args.debug;

    let mut filelist: Vec<PathBuf> = Vec::new();

    let pathtype = metadata(&pathfr).unwrap();
    if pathtype.is_file() {
        filelist.push(PathBuf::from(format!("{:?}", pathtype)));
        if debugmode {
            println!("input path type is FILE");
        }
    } else if pathtype.is_dir() {
        if debugmode {
            println!("input path type is DIR");
        }
        if pathfr.ends_with("/") {
            if debugmode {
                println!("Popped trailing / off the end")
            }
            pathfr.pop();
        }
        // parse file list if it's a dir
        for entry in glob(&format!("{}/**/*.png", pathfr.clone().display()))
            .expect("Failed to read glob pattern (you should panic)")
        {
            match entry {
                Ok(path) => {
                    // println!("{:?}", path.display());
                    filelist.push(
                        path.display().to_string().parse().unwrap()
                    );
                }
                Err(e) => {
                    eprintln!("Globbing error... uh oh...");
                }
            }
        }
    } else {
        eprint!("that input is neither file nor dir and thats scary... aaa")
    }

    //    if debugmode {
    //        for i in filelist.clone() {
    //            println!("List of files: {}",i.display())
    //        }
    //    }

    for thatpath in filelist {
        println!("Filename: {:?}",thatpath);
        idk::encoder(thatpath.to_owned(), thatpath)
    }
}