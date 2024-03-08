use zune_jpeg::JpegDecoder;
use std::env;
use std::fs;

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
    let wd2 = wd.clone();
    let image_path = wd + "\\src\\image.jpg";
    let ascii_path = wd2 + "\\src\\ascii.txt";

    let data = fs::read(image_path).unwrap();
    let mut decoder = JpegDecoder::new(&data);
    let pixels = decoder.decode().unwrap();
    let image_info = decoder.info().unwrap();

    let mut _ascii = String::new();
    let mut _i = 0;
    for i in (0..pixels.len()-1).step_by(usize::from(image_info.components)){
        let sum: i32 = match image_info.components {
            1 => i32::from(pixels[i]),
            2 => i32::from(pixels[i]) + i32::from(pixels[i+1]),
            3 => i32::from(pixels[i]) + i32::from(pixels[i+1]) + i32::from(pixels[i+2]),
            _ => panic!(),
        };
        
        if _i == image_info.width {
            _i = 0;
            _ascii += "\n"; 
        }
        if sum < 17 * i32::from(image_info.components) {
            _i += 1;
            _ascii += "."; 
            continue;
        }
        if sum < 34 * i32::from(image_info.components) { 
            _i += 1;
            _ascii += "`"; 
            continue;
        }
        if sum < 51 * i32::from(image_info.components) {
            _i += 1;
            _ascii += "-"; 
            continue;
        }
        if sum < 68 * i32::from(image_info.components) { 
            _i += 1;
            _ascii += "*";
            continue;
        }
        if sum < 85 * i32::from(image_info.components) {
            _i += 1;
            _ascii += "~";
            continue;
        }
        if sum < 102 * i32::from(image_info.components) {
            _i += 1;
            _ascii += "^";
            continue;
        }
        if sum < 119 * i32::from(image_info.components) {
            _i += 1;
            _ascii += "+";
            continue;
        }
        if sum < 136 * i32::from(image_info.components) {
            _i += 1;
            _ascii += "=";
            continue;
        }
        if sum < 153 * i32::from(image_info.components) {
            _i += 1;
            _ascii += "%";
            continue;
        }
        if sum < 170 * i32::from(image_info.components) {
            _i += 1;
            _ascii += "$";
            continue;
        }
        if sum < 187 * i32::from(image_info.components) {
            _i += 1;
            _ascii += ">";
            continue;
        }
        if sum < 204 * i32::from(image_info.components) { 
            _i += 1;
            _ascii += "<";
            continue;
        }
        if sum < 221 * i32::from(image_info.components) {
            _i += 1;
            _ascii += "&";
            continue;
        }
        if sum < 238 * i32::from(image_info.components) {
            _i += 1;
            _ascii += "@"; 
            continue;
        }
        _i += 1;
        _ascii += "#";

    }

    let _ = fs::write(ascii_path, _ascii);

}
