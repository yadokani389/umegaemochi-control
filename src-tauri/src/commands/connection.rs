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
