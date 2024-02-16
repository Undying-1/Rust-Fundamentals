enum FileSize {
    Bytes(u64),
    Kilobytes(f64),
    Megabytes(f64),
    Gigabytes(f64),
}

fn format_size(size: u64) -> String {
    let filesize = match size {
        0..=999 => FileSize::Bytes(size),
        1000..=999_999 => FileSize::Kilobytes(size as f64 / 1000.0),
        1_000_000..=999_999_999 => FileSize::Megabytes(size as f64 / 1_000_000.0),
        _ => FileSize::Gigabytes(size as f64 / 1_000_000_000.0),
    };

    match filesize {
        FileSize::Bytes(bytes) => format!("{} bytes", bytes),
        FileSize::Kilobytes(kb) => format!("{:.2} KB", kb),
        FileSize::Megabytes(mb) => format!("{:.2} MB", mb),
        FileSize::Gigabytes(gb) => format!("{:.2} GB", gb),
    }
}



#[derive(Debug)]
struct Sizes {
    bytes: String,
    kilobytes: String,
    megabytes: String,
    gigabytes: String,
}


impl Sizes {
    fn get_sizes(data: String) -> Self{
        let temp: Vec<&str> = data.split_whitespace().collect();
        let size =  match temp[1]{
            "bytes" => temp[0].parse::<f64>().unwrap(),
            "KB" => temp[0].parse::<f64>().unwrap() * 1000.0,
            "MB" => temp[0].parse::<f64>().unwrap() * 1000.0 * 1000.0,
            "GB" => temp[0].parse::<f64>().unwrap() * 1000.0 * 1000.0 * 1000.0,
            _ => {
                println!("{}", temp[1]);
                panic!("Here is an error")
            },
        };

        Sizes{
            bytes: format!("{} bytes", size),
            kilobytes: format!("{:.2} kilobytes", size / 1000.0),
            megabytes: format!("{:.2} megabytes", size / 1000.0 / 1000.0),
            gigabytes: format!("{:.2} gigabytes", size / 1000.0 / 1000.0 / 1000.0),
        }
    }
}


fn main() {
    let result = format_size(688);
    println!("{}", result);
    let res = Sizes::get_sizes(result);
    println!("{:?}", res);
}