use std::path::Path;

pub fn human_size(bytes: u64) -> String {
    let units = ["B", "KB", "MB", "GB"];
    let mut size = bytes as f64;
    let mut i = 0;
    while size >= 1024.0 && i < units.len() - 1 {
        size /= 1024.0;
        i += 1;
    }
    format!("{size:.1} {}", units[i])
}

pub fn level(value: f64, warn: f64, hot: f64) -> &'static str {
    if value < warn {
        "ok"
    } else if value < hot {
        "warn"
    } else {
        "hot"
    }
}

pub fn ext_color(name: &str) -> &'static str {
    let ext = Path::new(name)
        .extension()
        .and_then(|e| e.to_str())
        .map(|e| e.to_lowercase())
        .unwrap_or_default();

    match ext.as_str() {
        "pdf" => "#f2777a",
        "doc" | "docx" => "#6fa8ff",
        "md" => "#66d9ef",
        "txt" => "#b0b8c2",
        "xls" | "xlsx" | "csv" => "#7ee08a",
        "zip" | "tar" | "gz" | "7z" => "#f0c674",
        "png" | "jpg" | "jpeg" | "gif" | "svg" => "#d68cf2",
        _ => "#e6e9ee",
    }
}
