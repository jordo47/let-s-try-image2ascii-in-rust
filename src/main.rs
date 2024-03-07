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
    println!("{},{}", image_info.width, image_info.height);
    print_type(&pixels);
    let mut _i = 0;
    let mut _ascii = String::new();
    for item in &pixels {
        if _i == image_info.width {
            _i = 0;
            _ascii += "\r\n"; 
        }
        if i32::from(*item) < 17 {
            _i += 1;
            _ascii += "."; 
            continue;
        }
        if i32::from(*item) < 34 { 
            _i += 1;
            _ascii += "`"; 
            continue;
        }
        if i32::from(*item) < 51 {
            _i += 1;
            _ascii += "-"; 
            continue;
        }
        if i32::from(*item) < 68 { 
            _i += 1;
            _ascii += "*";
            continue;
        }
        if i32::from(*item) < 85 {
            _i += 1;
            _ascii += "~";
            continue;
        }
        if i32::from(*item) < 102 {
            _i += 1;
            _ascii += "^";
            continue;
        }
        if i32::from(*item) < 119 {
            _i += 1;
            _ascii += "+";
            continue;
        }
        if i32::from(*item) < 136 {
            _i += 1;
            _ascii += "=";
            continue;
        }
        if i32::from(*item) < 153 {
            _i += 1;
            _ascii += "%";
            continue;
        }
        if i32::from(*item) < 170 {
            _i += 1;
            _ascii += "$";
            continue;
        }
        if i32::from(*item) < 187 {
            _i += 1;
            _ascii += ">";
            continue;
        }
        if i32::from(*item) < 204 { 
            _i += 1;
            _ascii += "<";
            continue;
        }
        if i32::from(*item) < 221 {
            _i += 1;
            _ascii += "&";
            continue;
        }
        if i32::from(*item) < 238 {
            _i += 1;
            _ascii += "@"; 
            continue;
        }
        _i += 1;
        _ascii += "#"; 
    }

    let _ = fs::write(ascii_path, _ascii);

}
