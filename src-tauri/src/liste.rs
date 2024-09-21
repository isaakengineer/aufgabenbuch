use std::fs;
use std::path::Path;
use std::fs::File;
use std::io::Write;
use std::sync::Mutex;

use tauri::{AppHandle, Emitter, Listener, Builder, Manager};

use tauri_plugin_dialog::{DialogExt, MessageDialogKind};

use sqlx::{sqlite::{SqliteConnection, SqlitePoolOptions}, Pool, Sqlite, FromRow};
use tauri_plugin_fs::FilePath;

#[derive(Default)]
pub struct AppData {
    pub db_path: String,
    pub pool: Option<Pool<Sqlite>>,
}

fn datenbankpfad_fuellen(app: AppHandle, pfad: String) {
    let data = app.state::<Mutex<AppData>>();
    let mut data = data.lock().unwrap();
    data.db_path = pfad;
}
fn datenbankpfad_leeren(app: AppHandle) {
    let data = app.state::<Mutex<AppData>>();
    let mut data = data.lock().unwrap();
    data.db_path = format!("");
}
fn pool_fuellen(app: AppHandle, pool: Pool<Sqlite>) {
    let data = app.state::<Mutex<AppData>>();
    let mut data = data.lock().unwrap();
    data.pool = Some(pool);
}




// match File::create(&dateipfad) {
//     Ok(mut file) => {
//         let pfad = dateipfad.to_str().unwrap().to_string();
//         datenbankpfad_fuellen(app.clone(), pfad.clone());
//         datenbank_aktivieren_oder_herstellen(app.clone(), pfad.clone()).await;
        
//         app.emit("file-gewaehlt", true).unwrap();
//         return Ok(format!("File created at ..."));
//     },
//     Err(e) => return Err(format!("Failed to create file: {:?}", e)),
// }

// match File::create(&pfad_mit_endung) {
//     Ok(mut file) => {
//         let data = app.state::<Mutex<AppData>>();
//         let db_path = pfad_mit_endung.to_str().unwrap().to_string();
//         {
//             let mut data = data.lock().unwrap();
//             data.db_path = db_path.clone();
            
//         }
//         datenbank_aktivieren_oder_herstellen(app.clone(), db_path).await;
//             app.emit("file-gewaehlt", true).unwrap();
//         return Ok(format!("File created at ..."));
//     },
//     Err(e) => return Err(format!("Failed to create file: {:?}", e)),
// }

fn neue_dateipfad_pruefen(pfad: FilePath) -> Result<String, String> {
    
    let dateipfad = pfad.into_path().unwrap(); // nur für Desktop wegen TAURI

    let message = format!("Eine Datei mit diesem Namen existiert bereits. Wir empfehlen Ihnen es nicht zu überschreiben, bitte.");

    if dateipfad.exists() {
        println!("File exists: {:?}", &dateipfad.to_str());
        return Err(message);
    } else {
        match dateipfad.extension() {
            Some(extension) if extension == "ab" || extension == "aufgabenbuch" => {
                return Ok(dateipfad.to_str().unwrap().to_string());
            },
            Some(_) => return Err(format!("Dateipfad hat eine ungültige Dateiendung.")),
            None => {
                let mut pfad_mit_endung = dateipfad.clone();
                pfad_mit_endung.set_extension("ab");
                if pfad_mit_endung.exists() {
                    println!("File exists: {:?}", &pfad_mit_endung.to_str());
                    return Err(message);
                } else {
                    return Ok(pfad_mit_endung.to_str().unwrap().to_string());
                }
            },
        }
    }
}

#[tauri::command]
pub async fn file_erstellen(app: AppHandle) -> Result<String, String> {
    let app_handle = app.clone();
    if let Some(neue_pfad) = app
        .dialog()
        .file()
        .add_filter("My Filter", &["ab", "aufgabenbuch"])
        .blocking_save_file()
    {
        match neue_dateipfad_pruefen(neue_pfad) {
            Ok(pfad) => {
                // return Ok(dateipfad.to_string());
                match File::create(&pfad) {
                    Ok(mut file) => {

                        datenbankpfad_fuellen(app.clone(), pfad.clone());
                        datenbank_aktivieren_oder_herstellen(app.clone(), pfad.clone()).await;
                        
                        app.emit("file-gewaehlt", true).unwrap();
                        return Ok(format!("File created at ..."));
                    },
                    Err(e) => return Err(format!("Failed to create file: {:?}", e)),
                }
            },
            Err(e) => {
                app.clone().dialog()
                    .message(&e)
                    .kind(MessageDialogKind::Info)
                    .title("Ausfall")
                    .ok_button_label("OK")
                    .show(|result| match result {
                        true => (),
                        false => ()// do something,
                    });
                    return Err(e);
            },
        }
    } else {
        Err(String::from("No file path selected"))
    }
}

use std::sync::{Arc};
use tauri::async_runtime::block_on;
use tokio::sync::oneshot;

#[tauri::command]
pub async fn file_waehlen(app: AppHandle) -> Result<String, String> {
    let app_handle = app.clone();
    
    // Create a channel to communicate between the callback and the async function
    let (tx, rx) = oneshot::channel();

    app.dialog().file().pick_file(move |file_path| {
        if let Some(file_path) = file_path {
            println!("Selected file: {}", file_path.to_string());

            {
                let data = app_handle.state::<Mutex<AppData>>();
                let mut data = data.lock().unwrap();
                data.db_path = file_path.to_string();
            }

            

            // Send the file path through the channel
            tx.send(file_path.to_string()).unwrap();
        } else {
            println!("No file selected");
            // Send an empty string to indicate no file was selected
            tx.send(String::new()).unwrap();
        }
    });

    // Wait for the file path from the callback
    let file_path = rx.await.unwrap();

    if file_path.is_empty() {
        return Ok("No file selected".to_string());
    }

    datenbank_aktivieren_oder_herstellen(app.clone(), file_path).await;
    
    println!("datenbank noch nicht aktiviert!");
    Ok(format!("lass uns mal sehen"))
}

use sqlx::Error;

async fn has_migration_been_applied(pool: &Pool<Sqlite>, migration_name: &str) -> Result<bool, String> {
    let result = sqlx::query(
        "SELECT COUNT(*) as count FROM _sqlx_migrations WHERE description = ?1")
        .bind(migration_name)
        .fetch_one(pool)
        .await;
    
    match(result) {
        Ok(_) => Ok(true),
        Err(e) => {
			return Err(format!("{:?}", e));
		},
    }
   
}

async fn datenbank_aktivieren_oder_herstellen(app: AppHandle, path: String) -> Result<String, String> {
    let pool = SqlitePoolOptions::new()
        .connect(&path).await.unwrap();

    // Check if the database is empty
    let is_empty = match sqlx::query("SELECT name FROM sqlite_master WHERE type='table' AND name NOT LIKE 'sqlite_%';")
        .fetch_optional(&pool).await {
        Ok(Some(_)) => false, // There are tables in the database
        Ok(None) => true,     // No tables found
        Err(e) => return Err(format!("Error checking database: {}", e)),
    };

    if is_empty {
        // Perform migrations if the database is empty
        sqlx::migrate!("./migrations").run(&pool).await.unwrap();
    } else {
        println!("Datenbank ist nicht leer!");
        let migration_name = "20240919152942_erstelle_aufgaben_tabelle";
        let applied = has_migration_been_applied(&pool, migration_name).await;
    
        match applied {
            Ok(_) => {
                println!("daten bank ist richtig konfiguriert!");
                pool_fuellen(app.clone(), pool);
            },
            Err(e) => {
                println!("daten bank ist falsch");
                let message = format!("Falsche Datenbank würde benutzt!");
                datenbankpfad_leeren(app.clone());
                app.clone().dialog()
                    .message(&message)
                    .kind(MessageDialogKind::Info)
                    .title("Ausfall")
                    .ok_button_label("OK")
                    .show(|result| match result {
                        true => (),
                        false => ()// do something,
                    });
                    return Err(message);
            }
        }
        
    }



    
    app.clone().emit("file-gewaehlt", true).unwrap();

    Ok(format!(""))
}
#[tauri::command]
pub async fn datenbank_erstellen(app: AppHandle) -> String {

    let data = app.state::<Mutex<AppData>>();
    let db_path = data.lock().unwrap().db_path.clone();


    println!("db_path während Aktivierung: {}", &db_path);

    datenbank_aktivieren_oder_herstellen(app.clone(), db_path).await;
    
    format!("DB Path:")
}