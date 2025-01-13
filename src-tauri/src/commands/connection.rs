use crate::commands::utils::stringify;
use crate::settings::Settings;

#[tauri::command]
pub async fn get_settings(address: String) -> Result<Settings, String> {
    println!("Connecting to server at {}", address);

    let url = format!("http://{}/settings", address);

    let settings = reqwest::get(url)
        .await
        .map_err(stringify)?
        .json::<Settings>()
        .await
        .map_err(stringify)?;

    println!("Settings: {:?}", settings);

    Ok(settings)
}

#[tauri::command]
pub async fn post_settings(address: String, settings: Settings) -> Result<(), String> {
    println!("Connecting to server at {}", address);

    let url = format!("http://{}/settings", address);

    reqwest::Client::new()
        .post(url)
        .json(&settings)
        .send()
        .await
        .map_err(stringify)?;

    Ok(())
}

#[derive(Debug, Clone, PartialEq, serde::Serialize, serde::Deserialize)]
pub struct DisasterInfo {
    title: String,
    description: String,
    warning: String,
    occurred: chrono::DateTime<chrono::Local>,
}

#[tauri::command]
pub async fn post_disaster_info(address: String, info: DisasterInfo) -> Result<(), String> {
    println!("Connecting to server at {}", address);

    let url = format!("http://{}/disaster_info", address);

    reqwest::Client::new()
        .post(url)
        .json(&info)
        .send()
        .await
        .map_err(stringify)?;

    Ok(())
}
