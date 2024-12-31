use sqlx::{
		sqlite::{SqliteConnection, SqlitePoolOptions},
		FromRow, Pool, Sqlite,
};
use std::path::PathBuf;
use std::sync::Mutex;
use tauri::{AppHandle, Builder, Emitter, Listener, Manager};
use tauri_plugin_dialog::{DialogExt, MessageDialogKind};
use tauri_plugin_cli::CliExt;

mod aufgabe;
use aufgabe::{
	aufgabe_aendern,
	aufgabe_erledigen, aufgabe_wieder_aktivieren,
	aufgabe_hinfuegen,
	aufgabe_priorisieren,
	gruppen_alle, list_alle, list_erledigt,
	prioritaetenliste, list_jetzige,
	aufgaben_positionieren,
};

mod liste;
use liste::{dateipfad_eingegeben,
	// datenbank_erstellen,
	identitaet_weitergeben,
	schliessen,
	file_erstellen, file_waehlen, AppIdentitaet};

use tokio::sync::oneshot;
use tokio::runtime::Handle;

#[cfg_attr(mobile, tauri::mobile_entry_point)]
#[tokio::main]
pub async fn run() {
	tauri::Builder::default()
		.plugin(tauri_plugin_cli::init())
		.plugin(tauri_plugin_dialog::init())
		.plugin(tauri_plugin_fs::init())
		.plugin(tauri_plugin_shell::init())
		.invoke_handler(tauri::generate_handler![
			file_erstellen,
			file_waehlen,

			// datenbank_erstellen,
			identitaet_weitergeben,
			dateipfad_eingegeben,

			aufgabe_hinfuegen,
			aufgabe_aendern,

			aufgabe_erledigen,
			aufgabe_wieder_aktivieren,

			aufgabe_priorisieren,
			aufgaben_positionieren,

			list_alle,
			list_jetzige,
			list_erledigt,
			gruppen_alle,
			prioritaetenliste,
			schliessen
		])
		.setup(|app| {
			// #[cfg(debug_assertions)] // only include this code on debug builds
			// {
			//	 let window = app.get_webview_window("main").unwrap();
			//	 window.open_devtools();
			//	 window.close_devtools();
			// }

			app.manage(Mutex::new(AppIdentitaet::default()));

			let cli_eingaben = app.cli().matches().unwrap();
			let eingabe = cli_eingaben.args.get("pfad").unwrap().value.clone();
			print!("{:?}", eingabe);
			let args: Vec<_> = std::env::args().collect(); // get all arguements passed to app
			println!("{:?}", args);
			let mut m = format!("");

			if eingabe.is_boolean() && eingabe.as_bool().unwrap() == false {
				m = format!("kein Dateipfad beim Start eingegeben.");
			} else {
				let eingegebene_pfad = eingabe.as_str();
				match eingegebene_pfad {
					Some(pfad) => {
						let pfad_buf = std::path::PathBuf::from(pfad);
						if pfad_buf.is_dir() {
							m = format!("Fehler: eingegebene Dateipfad ist ein Fach!");
						} else {
							let charakter = app.state::<Mutex<AppIdentitaet>>();
							let mut charakter = charakter.lock().unwrap();
							charakter.dateipfad = Some(pfad_buf.display().to_string().clone());
							m = format!("Dateipfad würde eingenommen.");
						}
					},
					None => {}
				}
			}
			println!("{}",m);
			// app.listen("closeRequested", |e| {
			// 	println!("lass die Apps einfach schließen.");

			//	 tokio::task::block_in_place(move || {
			//		 let data = data.lock().unwrap();
			// 		Handle::current().block_on(async move {
			// 			close_application(data).await;
			// 		});

			//	 });

		 //	});

			Ok(())
		})
		.run(tauri::generate_context!())
		.expect("error while running tauri application");
}
