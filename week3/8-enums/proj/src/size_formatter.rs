enum FileSize {
    Bytes(u64),
    KB(f64),
    MB(f64),
    GB(f64),
    TB(f64),
}

const KB_RATIO: f64 = 1024.0;
const MB_RATIO: f64 = 1048576.0;
const GB_RATIO: f64 = 1073741824.0;
const TB_RATIO: f64 = 1099511627776.0;

fn format_bytes(size: u64) -> String {
 let file_size = match size {
        0..=1024 => FileSize::Bytes(size),
        1025..=1048576 => FileSize::KB(size as f64 / KB_RATIO),
        1048577..=1073741824 => FileSize::MB(size as f64 / MB_RATIO),
        1073741825..=1099511627776 => FileSize::GB(size as f64 / GB_RATIO),
        _ => FileSize::TB(size as f64 / TB_RATIO),
    };
    
    match file_size {
        FileSize::Bytes(size) => format!("{} bytes", size),
        FileSize::KB(size) => format!("{:.2} KB", size),
        FileSize::MB(size) => format!("{:.2} MB", size),
        FileSize::GB(size) => format!("{:.2} GB", size),
        FileSize::TB(size) => format!("{:.2} TB", size),
    }
}

pub fn run() {
    let sizes = vec![0, 1024, 1048582, 1173741824, 1099671627776];
    for size in sizes {
        println!("File size: {}", format_bytes(size));
    }
}