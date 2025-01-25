use crate::commands::utils::stringify;
use crate::settings::Settings;

#[tauri::command]
pub async fn get_settings(address: String) -> Result<Settings, String> {
    let url = format!("http://{}/settings", address);

    println!("Connecting to {}", url);

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
    let url = format!("http://{}/settings", address);

    println!("Connecting to {}", url);

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
    let url = format!("http://{}/disaster_info", address);

    println!("Connecting to {}", url);

    reqwest::Client::new()
        .post(url)
        .json(&info)
        .send()
        .await
        .map_err(stringify)?;

    Ok(())
}

#[tauri::command]
pub async fn clear_disaster_info(address: String) -> Result<(), String> {
    let url = format!("http://{}/disaster_info/clear", address);

    println!("Connecting to {}", url);

    reqwest::get(url).await.map_err(stringify)?;

    Ok(())
}

#[tauri::command]
pub async fn scroll(address: String, name: String) -> Result<(), String> {
    let url = format!("http://{}/scroll/{}", address, name);

    println!("Connecting to {}", url);

    reqwest::get(url).await.map_err(stringify)?;

    Ok(())
}

#[tauri::command]
pub async fn get_widgets(address: String) -> Result<Vec<String>, String> {
    let url = format!("http://{}/widgets", address);

    println!("Connecting to {}", url);

    let res = reqwest::get(url)
        .await
        .map_err(stringify)?
        .json::<Vec<String>>()
        .await
        .map_err(stringify)?;

    Ok(res)
}
