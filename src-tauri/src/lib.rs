use tauri::{AppHandle, Emitter, Listener, Builder, Manager};
use tauri_plugin_dialog::{DialogExt, MessageDialogKind};
use std::sync::Mutex;
use sqlx::{sqlite::{SqliteConnection, SqlitePoolOptions}, Pool, Sqlite, FromRow};
use std::path::PathBuf;

mod aufgabe;
use aufgabe::{aufgabe_hinfuegen, aufgabe_aendern, list_alle, aufgabe_erledigen, list_erledigt};

mod liste;
use liste::{AppData, file_waehlen, datenbank_erstellen, file_erstellen, dateipfad_eingegeben};
 
#[cfg_attr(mobile, tauri::mobile_entry_point)]
#[tokio::main]
pub async fn run() {
    tauri::Builder::default()
        .plugin(tauri_plugin_dialog::init())
        .plugin(tauri_plugin_fs::init())
        .plugin(tauri_plugin_shell::init())
        .invoke_handler(tauri::generate_handler![

            file_erstellen,
            file_waehlen,
            datenbank_erstellen,
            dateipfad_eingegeben,

            aufgabe_hinfuegen,
            aufgabe_aendern,
            aufgabe_erledigen,

            list_alle,
            list_erledigt,
        ])
        .setup(|app| {

            // #[cfg(debug_assertions)] // only include this code on debug builds
            // {
            //   let window = app.get_webview_window("main").unwrap();
            //   window.open_devtools();
            //   window.close_devtools();
            // }

            app.manage(Mutex::new(AppData::default()));
            Ok(())
        })
        .run(tauri::generate_context!())
        .expect("error while running tauri application");
}
