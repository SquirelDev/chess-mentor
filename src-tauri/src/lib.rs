mod puzzles;
mod openings;
mod engine;

// Learn more about Tauri commands at https://tauri.app/develop/calling-rust/
#[tauri::command]
fn greet(name: &str) -> String {
    format!("Hello, {}! You've been greeted from Rust!", name)
}

#[tauri::command]
fn get_random_puzzle(level: String) -> Result<puzzles::Puzzle, String> {
    puzzles::get_random_puzzle(level)
}

#[tauri::command]
fn get_all_openings() -> Result<Vec<openings::Opening>, String> {
    openings::get_all_openings()
}

#[tauri::command]
async fn get_engine_move(fen: String, depth: u8) -> Result<String, String> {
    engine::get_engine_move(fen, depth).await
}

#[cfg_attr(mobile, tauri::mobile_entry_point)]
pub fn run() {
    puzzles::initialize_puzzles();
    tauri::Builder::default()
        .plugin(tauri_plugin_opener::init())
        .invoke_handler(tauri::generate_handler![greet, get_random_puzzle, get_all_openings, get_engine_move])
        .run(tauri::generate_context!())
        .expect("error while running tauri application");
}
