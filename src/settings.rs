#[derive(Debug)]
pub struct Settings {
    pub(crate) min_ms: u32,
    pub(crate) max_ms: u32,
    pub(crate) chance_threshold_percent: f32,
}