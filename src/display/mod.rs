use chrono::{DateTime, FixedOffset, Offset, Utc};
use prettytable::{color, format, Attr, Cell, Row, Table};


/// Format a datetime to the user's timezone
pub fn format_datetime(dt: DateTime<Utc>, timezone: FixedOffset) -> String {
    let local_time = dt.with_timezone(&timezone);
    format!("{} ({:+})", 
        local_time.format("%Y-%m-%d %H:%M:%S"),
        timezone.fix().local_minus_utc() / 3600
    )
}

/// Create a styled cell with the given text and color
pub fn styled_cell(text: &str, color: color::Color) -> Cell {
    Cell::new(text)
        .with_style(Attr::Bold)
        .with_style(Attr::ForegroundColor(color))
}

/// Create a header cell
pub fn header_cell(text: &str) -> Cell {
    styled_cell(text, color::CYAN)
}

/// Create a table with the given headers
pub fn create_table(headers: &[&str]) -> Table {
    let mut table = Table::new();
    table.set_format(*format::consts::FORMAT_NO_BORDER_LINE_SEPARATOR);
    
    let header_row = Row::new(
        headers.iter()
            .map(|&h| header_cell(h))
            .collect()
    );
    table.set_titles(header_row);
    
    table
} 