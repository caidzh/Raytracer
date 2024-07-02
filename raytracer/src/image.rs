use std::path::Path;
use image::{GenericImageView,DynamicImage};

#[derive(Clone)]
pub struct RtwImage{
    pub image_width:i32,
    pub image_height:i32,
    pub data:Option<DynamicImage>,
}

impl Default for RtwImage{
    fn default() -> Self {
        RtwImage { image_width: 0, image_height: 0, data: None }
    }
}

impl RtwImage{
    pub fn new(image_filename:&str) -> Self{
        let filename=image_filename.to_string();
        let mut image=RtwImage::default();
        let search_paths = [
            ".",
            "images",
            "../images",
            "../..",
            "../../images",
            "../../../..",
            "../../../images",
        ];
        let mut found=false;
        for path in search_paths.iter() {
            let full_path = Path::new(path).join(&filename);
            if let Ok(img) = image::open(&full_path) {
                let temp_img=img.clone();
                image = RtwImage {
                    data: Some(temp_img),
                    image_width: img.width() as i32,
                    image_height: img.height() as i32,
                };
                found = true;
                break;
            }
        }
        if !found {
            eprintln!("ERROR: Could not load image file '{}'.", image_filename);
        }
        image
    }
    fn clamp(x:i32,low:i32,high:i32)->i32{
        if x<low{
            low
        }
        else if x<high{
            x
        }
        else{
            high-1
        }
    }
    pub fn pixel_data(&self,x:i32,y:i32)->[u8;3]{
        if let Some(img)=&self.data{
            let x_clamped=Self::clamp(x,0,self.image_width);
            let y_clamped=Self::clamp(y,0,self.image_height);
            let pixel=img.get_pixel(x_clamped as u32, y_clamped as u32);
            [pixel[0],pixel[1],pixel[2]]
        }
        else{
            [255,0,255]
        }
    }
}