use sqlx::{
    sqlite::{SqliteConnection, SqlitePoolOptions},
    FromRow, Pool, Sqlite,
};
use std::path::PathBuf;
use std::sync::Mutex;
use tauri::{AppHandle, Builder, Emitter, Listener, Manager};
use tauri_plugin_dialog::{DialogExt, MessageDialogKind};

mod aufgabe;
use aufgabe::{
	aufgabe_aendern,
	aufgabe_erledigen, aufgabe_wieder_aktivieren,
	aufgabe_hinfuegen,
	aufgabe_priorisieren,
	gruppen_alle, list_alle, list_erledigt,
	prioritaetenliste, list_jetzige,
};

mod liste;
use liste::{dateipfad_eingegeben, datenbank_erstellen, file_erstellen, file_waehlen, AppData};

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
	    aufgabe_wieder_aktivieren,

			aufgabe_priorisieren,

	    list_alle,
	    list_jetzige,
	    list_erledigt,
	    gruppen_alle,
	    prioritaetenliste,
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
