pub const PKG_NAME: &'static str = env!("CARGO_PKG_NAME");
pub const STR_UNDEFINED: &str = "STR_UNDEFINED";
pub const USIZE_UNDEFINED: usize = usize::MAX;
// コピー複数行の最後が空文字の場合の代替文字　WSLで改行を最後に設定出来ない対応
pub const COPY_END: &str = "COPY_END";
