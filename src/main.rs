use bhtmp::*;
use image::{ImageBuffer, Rgb, RgbImage};

use std::fs::{self, DirEntry, FileType};
fn main() {

    let paths = fs::read_dir("HeightMap").unwrap();

    let paths: Vec<DirEntry> = paths.into_iter().map(|v| v.unwrap()).collect();
    let pc = paths.len();
    for pidx in 0..8 {
        let mut sections: Vec<(Bhtmp, String)> = vec![];
        for (n, i) in paths.iter().enumerate() {
            if i.path().ends_with(".md") {
                continue;
            }
            println!("processing: {}/{} : picidx {}/7", n, pc, pidx);

            let bytes = fs::read(i.path()).unwrap();

            let section = Bhtmp::new(bytes);
            section
                .to_image(pidx)
                .save(format!("./out/{}.png", i.file_name().to_str().unwrap()))
                .unwrap();

            sections.push((section, i.file_name().to_str().unwrap().to_string()));
        }

        for i in ["S", "U", "G"] {
            let mut image: RgbImage = ImageBuffer::new(1200, 1000);

            for (obj, filename) in &sections {
                if filename.starts_with(i) {
                } else {
                    continue;
                };
                let x = filename.to_string()[2..=3].to_string();
                let y = filename.to_string()[5..=6].to_string();

                let xoffset: usize = usize::from_str_radix(x.as_str(), 10).unwrap();
                let yoffset: usize = usize::from_str_radix(y.as_str(), 10).unwrap();

                for (i, rec) in obj.records.iter().enumerate() {
                    image.put_pixel(
                        (i % 100 + xoffset * 100) as u32,
                        (i / 100 + yoffset * 100) as u32,
                        Rgb([rec.raw[pidx], rec.raw[pidx], rec.raw[pidx]]),
                    )
                }
            }
            image.save(format!("full/{pidx}_{i}_full.png")).unwrap();
        }
    }
}
