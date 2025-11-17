#[cfg(test)]
mod tests {
    use moseiik::main::compute_mosaic;
    use image::{
    imageops::{resize, FilterType::Nearest},
    GenericImage, GenericImageView, ImageReader, RgbImage,
};  
    use std::time::Instant;
    use std::{
    error::Error,
    fs,
    ops::Deref,
    sync::{Arc, Mutex},
};
    use moseiik::main::Options;
    
#[test]
#[cfg(any(target_arch = "x86", target_arch = "x86_64"))]
fn test_x86() {
    let param = Options {image : "assets/kit.jpeg".to_string() ,
        output : "out.png".to_string(), 
        tiles : "assets/tiles-small/images".to_string(), 
        tile_size : 25,
        scaling : 1,
        remove_used : false,
        verbose : false,
        simd: true ,
        num_thread : 1,
    };

    /* Récupere l'image de réference */
    let tile_result = || -> Result<RgbImage, Box<dyn Error>> {
        Ok(ImageReader::open("assets/ground-truth-kit.jpeg")?.decode()?.into_rgb8())
        };
    let image_test = match tile_result() {
        Ok(t) => t,
        Err(_) => return,
        };

    /*Appel de la fonction */
    compute_mosaic(param);

    /*L'image genérée */
    let tile_result = || -> Result<RgbImage, Box<dyn Error>> {
        Ok(ImageReader::open("out.png")?.decode()?.into_rgb8())
        };
    let image_out = match tile_result() {
        Ok(t) => t,
        Err(_) => return,
        };

    for j in 0 .. image_test.height(){
        for i in 0 .. image_test.width() {
            assert_eq!(image_out.get_pixel(j, i),image_test.get_pixel(j, i),"Erreur valeur de pixel differente");
            };
        };
    }


#[test]
#[cfg(target_arch = "aarch64")]
fn test_aarch64() {
    let param = Options {image : "assets/kit.jpeg".to_string() ,
        output : "out.png".to_string(), 
        tiles : "assets/tiles-small/images".to_string(), 
        tile_size : 25,
        scaling : 1,
        remove_used : false,
        verbose : false,
        simd: true ,
        num_thread : 1,
        };
        
    /* Récupere l'image de réference */
    let tile_result = || -> Result<RgbImage, Box<dyn Error>> {
        Ok(ImageReader::open("assets/ground-truth-kit.jpeg")?.decode()?.into_rgb8())
        };

    let image_test = match tile_result() {
        Ok(t) => t,
        Err(_) => return,
        };

    /*Appel de la fonction */
    compute_mosaic(param);

    /*L'image genérée */
    let tile_result = || -> Result<RgbImage, Box<dyn Error>> {
        Ok(ImageReader::open("out.png")?.decode()?.into_rgb8())
        };
    let image_out = match tile_result() {
        Ok(t) => t,
        Err(_) => return,
        };

    for j in 0 .. image_test.height(){
        for i in 0 .. image_test.width() {
            assert_eq!(image_out.get_pixel(j, i),image_test.get_pixel(j, i),"Erreur valeur de pixel differente");
            };
        };
    }

#[test]
fn test_generic() {

    let param = Options {image : "assets/kit.jpeg".to_string() ,
        output : "out.png".to_string(), 
        tiles : "assets/tiles-small/images".to_string(), 
        tile_size : 25,
        scaling : 1,
        remove_used : false,
        verbose : false,
        simd: false ,
        num_thread : 1,
    };


    /* Récupere l'image de réference */
    let tile_result = || -> Result<RgbImage, Box<dyn Error>> {
         Ok(ImageReader::open("assets/ground-truth-kit.jpeg")?.decode()?.into_rgb8())
        };
    let image_test = match tile_result() {
        Ok(t) => t,
        Err(_) => return,
        };

    /*Appel de la fonction */
    compute_mosaic(param);

    /*L'image genérée */
    let tile_result = || -> Result<RgbImage, Box<dyn Error>> {
        Ok(ImageReader::open("out.png")?.decode()?.into_rgb8())
        };
    let image_out = match tile_result() {
        Ok(t) => t,
        Err(_) => return,
        };

    for j in 0 .. image_test.height(){
            for i in 0 .. image_test.width() {
                assert_eq!(image_out.get_pixel(j, i),image_test.get_pixel(j, i),"Erreur valeur de pixel differente");
            };
        };
    }
}
