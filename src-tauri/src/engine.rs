use tauri::AppHandle;
use tauri_plugin_shell::ShellExt;
use tauri_plugin_shell::process::CommandEvent;
use tauri::async_runtime::spawn;
use tokio::sync::Mutex;
use std::sync::Arc;

pub async fn get_engine_move(app: AppHandle, fen: String, depth: u8) -> Result<String, String> {
    let shell = app.shell();
    let (mut rx, child) = shell.sidecar("stockfish-sidecar")
        .map_err(|e| format!("Failed to create sidecar command: {}", e))?
        .spawn()
        .map_err(|e| format!("Failed to spawn sidecar: {}", e))?;

    // Wrap child in Arc<Mutex<Option<...>>> to allow taking ownership
    let child_arc = Arc::new(Mutex::new(Some(child)));
    let child_clone_for_task = Arc::clone(&child_arc);

    let best_move = Arc::new(Mutex::new(None));
    let result_best_move = Arc::clone(&best_move);

    spawn(async move {
        while let Some(event) = rx.recv().await {
            if let CommandEvent::Stdout(line_bytes) = event {
                let line = String::from_utf8_lossy(&line_bytes);
                if line.starts_with("bestmove") {
                    let parts: Vec<&str> = line.split_whitespace().collect();
                    if parts.len() >= 2 {
                        let mut locked_move = result_best_move.lock().await;
                        *locked_move = Some(parts[1].to_string());

                        // Take ownership of the child process to kill it
                        let mut child_opt_guard = child_clone_for_task.lock().await;
                        if let Some(child_to_kill) = child_opt_guard.take() {
                            if let Err(e) = child_to_kill.kill() {
                                eprintln!("Failed to kill stockfish process: {}", e);
                            }
                        }
                        break;
                    }
                }
            }
        }
    });

    {
        let mut child_guard = child_arc.lock().await;
        if let Some(child) = child_guard.as_mut() {
            child.write(format!("position fen {}\n", fen).as_bytes()).map_err(|e| e.to_string())?;
            child.write(format!("go depth {}\n", depth).as_bytes()).map_err(|e| e.to_string())?;
        }
    }

    // Wait for the best move to be found, with a timeout
    for _ in 0..200 { // Timeout after 20 seconds (200 * 100ms)
        let locked_move = best_move.lock().await;
        if let Some(the_move) = locked_move.as_ref() {
            return Ok(the_move.clone());
        }
        drop(locked_move);
        tokio::time::sleep(tokio::time::Duration::from_millis(100)).await;
    }

    // If we timeout, we should still try to kill the process
    let mut child_opt_guard = child_arc.lock().await;
    if let Some(child_to_kill) = child_opt_guard.take() {
        if let Err(e) = child_to_kill.kill() {
            eprintln!("Failed to kill stockfish process on timeout: {}", e);
        }
    }

    Err("Engine timed out or failed to find a move.".to_string())
}
