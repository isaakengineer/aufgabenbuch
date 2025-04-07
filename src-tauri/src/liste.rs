use serde;
use std::fs;
use std::fs::File;
use std::io::Write;
use std::path::{Path, PathBuf};
use std::sync::Mutex;

use serde::Serialize;
use tauri::{AppHandle, Builder, Emitter, Listener, Manager};

use tauri_plugin_dialog::{DialogExt, MessageDialogKind};

use std::sync::Arc;
use tauri::async_runtime::block_on;
use tokio::sync::oneshot;

use sqlx::{
    sqlite::{SqliteConnection, SqlitePoolOptions},
    Error, FromRow, Pool, Sqlite,
};
use tauri_plugin_fs::FilePath;

const KURZE_ENDUNG: &'static str = "ab";
const LANGE_ENDUNG: &'static str = "aufgabebuch";
const PFAD_ENDUNGEN: [&'static str; 2] = [KURZE_ENDUNG, LANGE_ENDUNG];

const ERSTE_MIGRATIONS_DATEINAME: &'static str = "20240919152942_erstelle_aufgaben_tabelle";

#[derive(Default, Clone, Serialize)]
pub struct AppIdentitaet {
    pub dateipfad: Option<String>,
    pub name: Option<String>,
    pub endung: Option<String>,

    #[serde(skip_serializing)] // auslassen da es nicht nötig ist an Frontend mitzuschicken
    pub pool: Option<Pool<Sqlite>>,
}

// TODO:
// - Namen ändern
// - Error durch dialog anzeigen falls nicht DEV.MODE
// - alles queries rausnehmen
// - alle dialoge I11N

// ÜBER IDENTITÄT
fn datenbankpfad_fuellen(app: AppHandle, pfad: String) {
    // name ändern "identitaet_fuellen"
    if let Some((file_name, extension)) = dateipfad_auseinandernehmen(pfad.clone()) {
        let charakter = app.state::<Mutex<AppIdentitaet>>();
        let mut charakter = charakter.lock().unwrap();
        charakter.dateipfad = Some(pfad);
        charakter.name = Some(file_name.to_string());
        charakter.endung = Some(extension.to_string());
    } else {
        println!("Could not extract file name and extension");
    }
}

fn datenbankpfad_leeren(app: AppHandle) {
    // name ändern "identitaet_pfad_leeren"
    let data = app.state::<Mutex<AppIdentitaet>>();
    let mut data = data.lock().unwrap();
    data.dateipfad = None;
}

fn pool_fuellen(app: AppHandle, pool: Pool<Sqlite>) {
    // name ändern "identitaet_pool_fuellen"
    let data = app.state::<Mutex<AppIdentitaet>>();
    let mut data = data.lock().unwrap();
    data.pool = Some(pool);
}

fn identitaet_ausgeben(app: AppHandle) -> AppIdentitaet {
    let charakter = app.state::<Mutex<AppIdentitaet>>();
    let charakter = charakter.lock().unwrap();
    charakter.clone()
}

// ÜBER DATEI PFAD
fn dateipfad_auseinandernehmen(pfad: String) -> Option<(String, String)> {
    // vielleicht die checks sollten hier sein!
    let path: PathBuf = PathBuf::from(pfad);
    if let Some(file_name) = path.file_stem().and_then(|name| name.to_str()) {
        if let Some(extension) = path.extension().and_then(|ext| ext.to_str()) {
            return Some((file_name.to_string(), extension.to_string()));
        }
    }
    return None;
}

fn neue_dateipfad_pruefen(pfad: FilePath) -> Result<String, String> {
    let dateipfad = pfad.into_path().unwrap(); // nur für Desktop wegen TAURI

    let message = format!("Eine Datei mit diesem Namen existiert bereits. Wir empfehlen Ihnen es nicht zu überschreiben, bitte.");
    let message_2 = format!("Eigegebene Dateipfad hat eine ungültige Dateiendung.");

    if dateipfad.exists() {
        println!("File exists: {:?}", &dateipfad.to_str());
        return Err(message);
    } else {
        match dateipfad.extension() {
            Some(extension) if PFAD_ENDUNGEN.contains(&extension.to_str().unwrap_or("")) => {
                return Ok(dateipfad.to_str().unwrap().to_string());
            }
            Some(_) => return Err(message_2),
            None => {
                let mut pfad_mit_endung = dateipfad.clone();
                pfad_mit_endung.set_extension(KURZE_ENDUNG);
                if pfad_mit_endung.exists() {
                    println!("File exists: {:?}", &pfad_mit_endung.to_str());
                    return Err(message);
                } else {
                    return Ok(pfad_mit_endung.to_str().unwrap().to_string());
                }
            }
        }
    }
}

// ÜBER DATENBANK
async fn has_migration_been_applied(
    pool: &Pool<Sqlite>,
    migration_name: &str,
) -> Result<bool, String> {
    let result = sqlx::query(
        "SELECT COUNT(*) as count
			FROM _sqlx_migrations
			WHERE description = ?1",
    )
    .bind(migration_name)
    .fetch_one(pool)
    .await;
    match result {
        Ok(_) => Ok(true),
        Err(e) => {
            return Err(format!("{:?}", e));
        }
    }
}

async fn datenbank_aktivieren_oder_herstellen(
    app: AppHandle,
    path: String,
) -> Result<String, String> {
    let pool = SqlitePoolOptions::new().connect(&path).await.unwrap();

    // Check if the database is empty
    let is_empty = match sqlx::query(
        "SELECT name FROM sqlite_master
			WHERE type='table'
			AND name NOT LIKE 'sqlite_%';",
    )
    .fetch_optional(&pool)
    .await
    {
        Ok(Some(_)) => false, // There are tables in the database
        Ok(None) => true,     // No tables found
        Err(e) => return Err(format!("Error checking database: {}", e)),
    };

    if is_empty {
        // Perform migrations if the database is empty
        sqlx::migrate!("./migrations").run(&pool).await.unwrap();
        pool_fuellen(app.clone(), pool);
    } else {
        println!("Datenbank ist nicht leer!");

        let applied = has_migration_been_applied(&pool, ERSTE_MIGRATIONS_DATEINAME).await;
        match applied {
            Ok(_) => {
                println!("daten bank ist richtig konfiguriert!");
                pool_fuellen(app.clone(), pool);
            }
            Err(e) => {
                println!("daten bank ist falsch");
                let message = format!("Falsche Datenbank würde benutzt!");
                datenbankpfad_leeren(app.clone());
                app.dialog()
                    .message(&message)
                    .kind(MessageDialogKind::Info)
                    .title("Ausfall")
                    // .ok_button_label("OK")
                    // .buttons(MessageDialogButtons::OkCustom("Absolutely"))
                    .show(|result| match result {
                        true => (),
                        false => (), // do something,
                    });
                return Err(message);
            }
        }
    }

    let identitaet = identitaet_ausgeben(app.clone());
    app.emit("file-gewaehlt", identitaet).unwrap();
    Ok(format!(""))

    // return match identitaet {
    // 	Ok(i) => {
    // 		app.clone().emit("file-gewaehlt", i).unwrap();
    // 		Ok(format!(""))
    // 	}
    // 	Err(e) => {
    // 		app.clone()
    // 			.dialog()
    // 			.message(&e)
    // 			.kind(MessageDialogKind::Info)
    // 			.title("Ausfall")
    // 			.ok_button_label("OK")
    // 			.show(|result| match result {
    // 				true => (),
    // 				false => (), // do something,
    // 			});
    // 		return Err(e);
    // 	}
    // };
}

// OPTIONEN: drag-drop, open, new
#[tauri::command]
pub async fn dateipfad_eingegeben(app: AppHandle, pfad: String) -> Result<AppIdentitaet, String> {
    // TODO: noch weitere Checks hinzufügen, falls die "pfad" manipuliert worden sein sollte!
    // 	ähnlich wie "neue_dateipfad_pruefen" aber die erste Check soll anderes rum sein
    // 		Message = "this app cannot access the path provided!"
    // TODO: name ändern "dateipfad_einsetzen"

    let res = datenbank_aktivieren_oder_herstellen(app.clone(), pfad.clone()).await;
    match res {
        Ok(_) => {
            datenbankpfad_fuellen(app.clone(), pfad.clone());
            let identitaet = identitaet_ausgeben(app.clone());
            app.emit("file-gewaehlt", identitaet.clone()).unwrap();
            Ok(identitaet)
        }
        Err(e) => return Err(format!("Failed to create file: {:?}", e)),
    }
}
#[tauri::command]
pub async fn identitaet_bauen(app: AppHandle) -> Result<AppIdentitaet, String> {
    let charakter = app.state::<Mutex<AppIdentitaet>>();
    let pfad = charakter.lock().unwrap().dateipfad.clone();
    match pfad {
        Some(pfad) => {
            println!("pfad existiert: {}", pfad);
            let res = datenbank_aktivieren_oder_herstellen(app.clone(), pfad.clone()).await;
            match res {
                Ok(_) => {
                    datenbankpfad_fuellen(app.clone(), pfad.clone());
                    let identitaet = identitaet_ausgeben(app.clone());
                    app.emit("file-gewaehlt", identitaet.clone()).unwrap();
                    Ok(identitaet)
                }
                Err(e) => Err(format!("Failed to create file: {:?}", e)),
            }
        }
        None => Err(format!("Kein Dateipfad existiert.")),
    }
}

#[tauri::command]
pub async fn file_erstellen(app: AppHandle) -> Result<AppIdentitaet, String> {
    // TODO: name ändern  "dialog_neue_datei_erstellen"
    if let Some(neue_pfad) = app
        .dialog()
        .file()
        .add_filter("Aufgabenbuch", &PFAD_ENDUNGEN)
        .blocking_save_file()
    {
        match neue_dateipfad_pruefen(neue_pfad) {
            Ok(pfad) => {
                match File::create(&pfad) {
                    Ok(file) => {
                        let res =
                            datenbank_aktivieren_oder_herstellen(app.clone(), pfad.clone()).await;
                        match res {
                            Ok(_) => {
                                datenbankpfad_fuellen(app.clone(), pfad.clone());
                                let identitaet = identitaet_ausgeben(app.clone());
                                app.emit("file-gewaehlt", identitaet.clone()).unwrap();
                                Ok(identitaet)
                            }
                            Err(e) => return Err(format!("Failed to create file: {:?}", e)),
                        }

                        // match identitaet {
                        // 	Ok(i) => {
                        // 		app.clone().emit("file-gewaehlt", i.clone()).unwrap();
                        // 		Ok(i)
                        // 	},
                        // 	Err(e) => Err(format!("{:?}", e))
                        // }
                        // return Ok(identitaet);
                    }
                    Err(e) => return Err(format!("Failed to create file: {:?}", e)),
                }
            }
            Err(e) => {
                app.dialog()
                    .message(&e)
                    .kind(MessageDialogKind::Info)
                    .title("Ausfall")
                    // .buttons(MessageDialogButtons::OkCustom("Absolutely"))
                    .show(|result| match result {
                        true => (),
                        false => (), // do something,
                    });
                return Err(e);
            }
        }
    } else {
        Err(String::from("No file path selected"))
    }
}

#[tauri::command]
pub async fn file_waehlen(app: AppHandle) -> Result<AppIdentitaet, String> {
    // TODO: name ändern "dialog_existirende_datei_oeffnen"

    // Create a channel to communicate between the callback and the async function
    let (tx, rx) = oneshot::channel();

    app.clone()
        .dialog()
        .file()
        .add_filter("Aufgabenbuch", &PFAD_ENDUNGEN)
        .pick_file(move |file_path| {
            if let Some(file_path) = file_path {
                println!("Selected file: {}", file_path.to_string());

                // {
                // 	datenbankpfad_fuellen(app_handle.clone(), file_path.clone().to_string());
                // }

                tx.send(file_path.to_string()).unwrap();
            } else {
                println!("No file selected");
                // Send an empty string to indicate no file was selected
                tx.send(String::new()).unwrap();
            }
        });

    // Wait for the file path from the callback
    let file_path = rx.await.unwrap();

    if !file_path.is_empty() {
        let res = datenbank_aktivieren_oder_herstellen(app.clone(), file_path.clone()).await;
        match res {
            Ok(_) => {
                datenbankpfad_fuellen(app.clone(), file_path.clone());
                let identitaet = identitaet_ausgeben(app.clone());
                app.emit("file-gewaehlt", identitaet.clone()).unwrap();
                Ok(identitaet)
            }
            Err(e) => return Err(format!("Failed to create file: {:?}", e)),
        }
    } else {
        Err("No file selected".to_string())
    }

    // datenbank_aktivieren_oder_herstellen(app.clone(), file_path).await;

    // println!("datenbank noch nicht aktiviert!");
    // Ok(format!("lass uns mal sehen"))
}

#[tauri::command]
pub async fn schliessen(app: AppHandle) {
    let data = app.state::<Mutex<AppIdentitaet>>();
    let db = data.lock().unwrap().pool.clone();
    match db {
        Some(db) => {
            db.close().await;
        }
        None => {}
    }
    app.exit(0);
}
// TODO: rausnehmen falls extra
//
// #[tauri::command]
// pub async fn datenbank_erstellen(app: AppHandle) -> String {

// 	let data = app.state::<Mutex<AppIdentitaet>>();
// 	let dateipfad = data.lock().unwrap().dateipfad.clone();

// 	println!("dateipfad während Aktivierung: {:?}", &dateipfad);

// 	datenbank_aktivieren_oder_herstellen(app.clone(), dateipfad.unwrap()).await;

// 	format!("DB Path:")
// }
