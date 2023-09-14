mod utils;
pub mod collagecreator;

use std::{fs, io};
use collagecreator::{recreate, Trial};
use image::{open, RgbaImage, ImageError};

fn get_image(path: &str) -> Result<RgbaImage, ImageError> {
    Ok(open(path)?.into_rgba8())
}

fn get_paths_in_dir(dir: &str) -> Result<Vec<String>, io::Error> {
    let paths = fs::read_dir(dir)?.filter_map(|entry| {
        let path = entry.ok()?.path();
        if path.is_file() {
            path.to_str().map(|s| s.to_owned())
        } else {
            None
        }
    }).collect();

    Ok(paths)
}


fn main() -> Result<(), Box<dyn std::error::Error>> {
    let target_path = "./target.png";
    let output_path = "./output.png";
    let sources_dir = "./sources.png";
    let source_paths = get_paths_in_dir(sources_dir)?;

    // load images
    let target = get_image(target_path)?;
    let mut output = if fs::metadata(output_path).is_ok() {
        get_image(target_path)?
    } else {
        let (w, h) = target.dimensions();
        RgbaImage::new(w, h)
    };

    let mut sources = Vec::new();
    for source_path in source_paths {
        sources.push(get_image(&source_path)?)
    }

    recreate(target, sources, &mut output, Trial {
        init_attempts: 200,
        survivors: 20,
        offspring: 10,
        generations: 5,
        offspring_mutation_modifier: 0.8
    });
    
    Ok(())
}
