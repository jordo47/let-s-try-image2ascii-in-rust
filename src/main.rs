use zune_jpeg::JpegDecoder;
use std::{env, fs::read};

fn print_type<T>(_: &T) {
    println!("{}", std::any::type_name::<T>())
}

fn get_working_dir() -> String{
    let res = env::current_dir();
    match res {
        Ok(path) => path.into_os_string().into_string().unwrap(),
        Err(_) => "FAILED".to_string()
    }
}

fn main() {
    let wd = get_working_dir().to_owned();
    let image_path = wd + "\\src\\image.jpg";
    println!("{}", image_path.trim());
    let data = read(image_path).unwrap();
    let mut decoder = JpegDecoder::new(&data);
    let pixels = decoder.decode();
    print_type(&pixels)
}
