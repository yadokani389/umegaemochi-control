#[derive(Debug, Clone, serde::Serialize, serde::Deserialize)]
pub struct Settings {
    weather_city_id: String,
    atcoder_id: String,
    widget_interval: u64,
    using_widgets: Vec<String>,
    auto_fullscreen: bool,
    using_sports_news: Vec<String>,
    auto_hide_cursor: bool,
    nightmode_range: NightmodeRange,
}

#[derive(Debug, Copy, Clone, PartialEq, serde::Serialize, serde::Deserialize)]
struct NightmodeRange {
    start: chrono::NaiveTime,
    end: chrono::NaiveTime,
}
