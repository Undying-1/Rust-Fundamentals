use std::io::Bytes;


enum FileSize {
    Bytes(u64),
    Kilobytes(u64),
    Megabytes(u64),
    Gigabytes(u64),
}

impl FileSize{
    fn format_size(&self) -> String {
        match self {
            FileSize::Bytes(bytes) => format!("{:.2} bytes", bytes),
            FileSize::Kilobytes(kb) => format!("{:.2} KB", *kb as f64 / 1000.0),
            FileSize::Megabytes(mb) => format!("{:.2} MB", *mb as f64 / 1000.0),
            FileSize::Gigabytes(gb) => format!("{:.2} GB", *gb as f64 / 1000.0),
        }
    }
}





fn main() {
    let size = 4_100_00;
    let filesize = match size as f64 / 1000.0 {
        0.0..=999.9 => FileSize::Bytes(size),
        1000.0..=999_999.9 => FileSize::Kilobytes(size / 1000),
        1_000_000.0..=999_999_999.9 => FileSize::Megabytes(size / 1_000_000),
        _ => FileSize::Gigabytes(size / 1_000_000_000),
    };

    println!("File size: {}", filesize.format_size());
}
