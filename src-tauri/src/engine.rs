use tauri::AppHandle;

pub async fn get_engine_move(app: AppHandle, _fen: String, _depth: u8) -> Result<String, String> {
    let resource_path = app.path().resource_dir()
        .ok_or_else(|| "Could not resolve resource directory.".to_string())?;

    let dbg_msg = format!(
        "Tauri resource directory is: {:?}. The engine should be in an 'engine' subdirectory inside this path, and be named correctly (e.g., stockfish-aarch64-apple-darwin).",
        resource_path
    );

    // Return this debug message as an error so the user can see it.
    Err(dbg_msg)
}
