use tauri::{AppHandle, Emitter, Listener};
use tauri_plugin_dialog::{DialogExt, MessageDialogKind};

// Learn more about Tauri commands at https://tauri.app/v1/guides/features/command


use std::path::PathBuf;

fn file_waehlen(app: AppHandle) {
    println!("File waehlen");
    app.dialog().file().pick_file(move |file_path| {
        if let Some(file_path) = file_path {
            println!("Selected file: {}", file_path.to_string());
            app.emit("file-gewaehlt", true).unwrap();
        } else {
            println!("No file selected");
        }
    });
}

#[tauri::command] // JUST SOME INITIAL TESTS with dialog.message AND ipc
fn greet(app: AppHandle, name: String) -> String {
    let message = format!("Hello, {}! You've been greeted from Rust!", name);
//     // let file_path = app.dialog().file().blocking_pick_file();

//     // format!("Hello, {}! You've been greeted from Rust!", name);
//     // format!("{} has selected the file", name)

//     // app.dialog()
//     //     .message("Tauri is Awesome")
//     //     .kind(MessageDialogKind::Info)
//     //     .title("Information")
//     //     .ok_button_label("Absolutely")
//     //     .show(|result| match result {
//     //         true => (), // do something,
//     //         false => () // do something,
//     //     });
//     // let message = app.dialog().file().pick_file(move |file_path| {
//     //     match file_path {
//     //         None => "No file selected".to_string(),
//     //         Some(file_path) => "hi there".to_string(),
//     //     }
//     // });
//     let message = {
//         let rt = Runtime::new().unwrap();
//         let result = rt.block_on(async {
//             app.dialog().file().pick_file(move |file_path| {
//                 match file_path {
//                     None => "No file selected".to_string(),
//                     Some(file_path) => "hi there".to_string(),
//                 }
//             }).await
//         });

//         result.unwrap_or("Error selecting file".to_string())
//     };

    message
}

#[cfg_attr(mobile, tauri::mobile_entry_point)]
pub fn run() {
    tauri::Builder::default()
        .plugin(tauri_plugin_dialog::init())
        .plugin(tauri_plugin_fs::init())
        .plugin(tauri_plugin_shell::init())
        .invoke_handler(tauri::generate_handler![greet])
        .setup(|app| {
            // let handle = app.handle().clone();
            let handle_clone = app.handle().clone();
            app.listen("file-waehlen", move |event| {
                if let Ok(flag) = serde_json::from_str::<bool>(event.payload()) {
                    file_waehlen(handle_clone.clone());
                    println!("Flag: {}", flag.to_string());
                    
                    handle_clone.unlisten(event.id());
                }
            });
            Ok(())
        })
        .run(tauri::generate_context!())
        .expect("error while running tauri application");
}
