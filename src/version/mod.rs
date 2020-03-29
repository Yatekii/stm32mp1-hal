mod data;

pub fn version() -> &'static str {
    unsafe { ::core::str::from_utf8_unchecked(&data::VERSION_TEXT).trim() }
}
