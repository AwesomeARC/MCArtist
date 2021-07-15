use std::path::Path;
use std::error::Error;
use std::io::ErrorKind;
use image::{
    DynamicImage,
    imageops::FilterType
};

pub fn read_image(image_path: &Path, width: u32, height: u32)
    -> Result<DynamicImage, Box<dyn Error>> {

    if !image_path.exists() {

        eprintln!(
            "{}\n{}",
            "[ImageReader] Error: The specified path does not exist.",
            "Please make sure a valid image exists at the specified path and try again."
        );
        
        return Err(Box::from(
            std::io::Error::from( ErrorKind::NotFound )
        ));

    }

    let img = image::open(image_path)?
        .resize(width, height, FilterType::Gaussian)
        .fliph();

    Ok(img)

}
