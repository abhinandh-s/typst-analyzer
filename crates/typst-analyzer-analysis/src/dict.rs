pub static NONE: &str = "none";
pub static AUTO: &str = "auto";
// FIX: currenty putting ( ) will cause function param parsing fail
// pub static COLUMNS: (&str, &str) = ("columns", "(1fr, 1fr)");
pub static COLUMNS: (&str, &str) = ("columns", AUTO);
pub static ROWS: (&str, &str) = ("rows", AUTO);
pub static GUTTER: (&str, &str) = ("gutter", AUTO);
pub static COLUMN_GUTTER: (&str, &str) = ("column-gutter", AUTO);
pub static ROW_GUTTER: (&str, &str) = ("row-gutter", AUTO);
pub static FILL: (&str, &str) = ("fill", NONE);
pub static ALIGN: (&str, &str) = ("align", AUTO);
pub static STROKE: (&str, &str) = ("stroke", NONE);
pub static INSET: (&str, &str) = ("inset", "relative");
