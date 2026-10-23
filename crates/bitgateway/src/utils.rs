pub fn format_flux(bytes: u64) -> String {
    const UNITS: [&str; 5] = ["B", "KB", "MB", "GB", "TB"];
    let mut value = bytes as f64;
    let mut unit_index = 0usize;

    while value >= 1024.0 && unit_index + 1 < UNITS.len() {
        value /= 1024.0;
        unit_index += 1;
    }

    if unit_index == 0 {
        format!("{}{}", bytes, UNITS[unit_index])
    } else {
        let formatted = format!("{value:.2}");

        trim_decimal(&formatted, UNITS[unit_index])
    }
}

pub fn humanize_duration(seconds: i64) -> String {
    let normalized_seconds = seconds.max(0);
    let days = normalized_seconds / 86_400;
    let hours = normalized_seconds % 86_400 / 3_600;
    let minutes = normalized_seconds % 3_600 / 60;

    format!("{days}天{hours}小时{minutes}分钟")
}

pub fn format_balance(balance: f64) -> String {
    let formatted = format!("{balance:.2}");

    formatted
        .trim_end_matches('0')
        .trim_end_matches('.')
        .to_string()
}

fn trim_decimal(value: &str, unit: &str) -> String {
    let trimmed = value.trim_end_matches('0').trim_end_matches('.');

    format!("{trimmed}{unit}")
}
