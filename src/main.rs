extern crate raster;

use std::env;
use std::fs;
use std::path::Path;
use raster::{editor, PositionMode};

fn main() -> Result<(), String> {
    let mut args = env::args().skip(1);

    let infile = args.next()
                    .ok_or("No input filename provided".to_string())?;
    let outdir = args.next()
                    .ok_or("No output directory provided".to_string())?;
    let cols = args.next()
                    .ok_or("No column count provided".to_string())
                    .and_then(|x| x.parse::<i32>().map_err(|y| y.to_string()))?;
    let rows = args.next()
                    .ok_or("No row count provided".to_string())
                    .and_then(|x| x.parse::<i32>().map_err(|y| y.to_string()))?;

    let outdir = Path::new(&outdir);
    if !outdir.is_dir() {
        fs::create_dir(outdir)
            .map_err(|_| "Failed to create output directory".to_string())?;
    }

    let image = raster::open(&infile)
                .map_err(|_| "Failed to load image".to_string())?;

    let (fragw, fragh) = (image.width / cols, image.height / rows);
    println!("Source: {}x{}  Chunk: {}x{}  Count: {}", image.width, image.height, fragw, fragh, rows * cols);

    let outfext = Path::new(&infile).extension();

    for j in 0 .. rows {
        for i in 0 .. cols {
            let mut frag = image.clone();
            editor::crop(&mut frag, fragw, fragh, PositionMode::TopLeft, fragw * i, fragh * j)
                .map_err(|_| "Failed to crop image. This should not happen".to_string())?;

            let mut outpath = outdir.to_path_buf();
            outpath.push(&format!("chunk-{}-{}", i + 1, j + 1));

            if let Some(ext) = outfext {
                outpath.set_extension(ext);
            }

            raster::save(&frag, &outpath.to_string_lossy())
                .map_err(|_| "Failed to save image".to_string())?;
        }
    }

    Ok(())
}
