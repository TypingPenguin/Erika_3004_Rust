#[derive(Debug)]
pub struct Settings {
    pub(crate) justify: bool,
    pub(crate) characters_per_line: u32,
    pub(crate) min_ms: u32,
    pub(crate) max_ms: u32,
    pub(crate) chance_threshold_percent: f32,
}