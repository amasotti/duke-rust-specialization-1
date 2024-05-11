const KB_RATIO: f64 = 1024.0;
const MB_RATIO: f64 = 1048576.0;
const GB_RATIO: f64 = 1073741824.0;
const TB_RATIO: f64 = 1099511627776.0;

enum FileSize {
    Bytes(u64),
    Kilobytes(f64),
    Megabytes(f64),
    Gigabytes(f64),
    Terabytes(f64),
}

fn format_size(size: u64) -> String {
    let filesize = match size {
        0..=999 => FileSize::Bytes(size),
        1000..=999_999 => FileSize::Kilobytes(size as f64 / 1000.0),
        1_000_000..=999_999_999 => FileSize::Megabytes(size as f64 / 1_000_000.0),
        1_000_000_000..=999_999_999_999 => FileSize::Gigabytes(size as f64 / 1_000_000_000.0),
        _ => FileSize::Terabytes(size as f64 / 1_000_000_000_000.0),
    };

    match filesize {
        FileSize::Bytes(bytes) => format!("{} bytes", bytes),
        FileSize::Kilobytes(kb) => format!("{:.2} KB", kb as f64 / 1000.0),
        FileSize::Megabytes(mb) => format!("{:.2} MB", mb as f64 / 1000.0),
        FileSize::Gigabytes(gb) => format!("{:.2} GB", gb as f64 / 1000.0),
        FileSize::Terabytes(tb) => format!("{:.2} TB", tb as f64 / 1000.0),
    }
}


fn format_to_largest_unit(size: u64) -> String {
    let filesize = match size {
        0..=999 => FileSize::Bytes(size),
        1000..=999_999 => FileSize::Kilobytes(size as f64 / KB_RATIO),
        1_000_000..=999_999_999 => FileSize::Megabytes(size as f64 / MB_RATIO),
        1_000_000_000..=999_999_999_999 => FileSize::Gigabytes(size as f64 / GB_RATIO),
        _ => FileSize::Terabytes(size as f64 / TB_RATIO),
    };

    match filesize {
        FileSize::Bytes(bytes) => format!("{} bytes", bytes),
        FileSize::Kilobytes(kb) => format!("{:.2} KB", kb as f64),
        FileSize::Megabytes(mb) => format!("{:.2} MB", mb as f64),
        FileSize::Gigabytes(gb) => format!("{:.2} GB", gb as f64),
        FileSize::Terabytes(tb) => format!("{:.2} TB", tb as f64),
    }
}


pub fn lab2() {
    let result = format_size(6888837399);
    println!("{}", result);
    
    let bytes = 2500;
    println!("{}", format_to_largest_unit(bytes));
    
    let bytes = (1024*2)+512;
    println!("{}", format_to_largest_unit(bytes))
}
