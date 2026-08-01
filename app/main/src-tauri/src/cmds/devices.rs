use tauri::State;
use core_lib::services::device_manager::DeviceManager;

#[tauri::command]
pub async fn list_devices(
    user_id: String,
    manager: State<'_, DeviceManager>,
) -> Result<Vec<serde_json::Value>, String> {
    let devices = manager
        .list_user_devices(&user_id)
        .await
        .map_err(|e| e.to_string())?;
    
    Ok(devices.iter().map(|d| serde_json::to_value(d).unwrap()).collect())
}

#[tauri::command]
pub async fn trust_device(
    user_id: String,
    device_id: String,
    manager: State<'_, DeviceManager>,
) -> Result<(), String> {
    manager
        .trust_device(&user_id, &device_id)
        .await
        .map_err(|e| e.to_string())
}

#[tauri::command]
pub async fn get_device_contacts(
    user_id: String,
    manager: State<'_, DeviceManager>,
) -> Result<Vec<serde_json::Value>, String> {
    let contacts = manager
        .get_device_contacts(&user_id)
        .await
        .map_err(|e| e.to_string())?;
    
    Ok(contacts.iter().map(|c| serde_json::to_value(c).unwrap()).collect())
}
