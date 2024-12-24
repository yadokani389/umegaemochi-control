#[derive(Debug, Clone, serde::Serialize, serde::Deserialize)]
pub struct Settings {
    weather_city_id: String,
    atcoder_id: String,
}
