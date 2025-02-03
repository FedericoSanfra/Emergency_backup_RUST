use serde::Deserialize;
use std::{fs, path::Path, error::Error};
use winreg::enums::*;
use winreg::RegKey;
use std::env;
use std::io::{self, Write};
use std::fs::OpenOptions;
use chrono::Local;

#[derive(Deserialize, Debug)]
pub struct Config {
    pub source_path: String,
    pub file_types: Vec<String>
}

fn load_config(path: &str) -> Result<Config, Box<dyn Error>> {
    // Ottieni il percorso dell'eseguibile
    let exe_path = env::current_exe().expect("Impossibile ottenere il percorso dell'eseguibile");
    // Calcola la root del progetto risalendo dalla directory dell'eseguibile
    let project_root = exe_path
        .parent() // target/debug/
        .unwrap()
        .parent() // target/
        .unwrap()
        .parent() // Group14/
        .unwrap();

    // Costruisci il percorso completo del file di configurazione
    let config_path = project_root.join(path);

    // Verifica se il file di configurazione esiste
    if !config_path.exists() {
        return Err(format!("Il file di configurazione non esiste: {}", config_path.display()).into());
    }

    // Leggi il contenuto del file di configurazione
    let config_data = fs::read_to_string(&config_path)?;
    // Parse del file TOML nella struttura `Config`
    let config: Config = toml::from_str(&config_data)?;


    Ok(config)
}

fn setup_windows_startup() -> io::Result<()> {
    let hkcu = RegKey::predef(HKEY_CURRENT_USER);
    let run_key = hkcu.open_subkey_with_flags("Software\\Microsoft\\Windows\\CurrentVersion\\Run", KEY_WRITE)?;

    let exe_path = env::current_exe()?.to_str()
    .ok_or(io::Error::new(io::ErrorKind::InvalidData, "Invalid path"))?
    .to_string();
    println!("Adding to registry: {}", exe_path);
    
    println!("Sono dentro setup");
    run_key.set_value("MyRustBackupApp", &exe_path)
        .map_err(|e| {
            eprintln!("Failed to write to registry: {:?}", e);
            e
        })
}

fn is_startup_configured() -> bool {
    let hkcu = RegKey::predef(HKEY_CURRENT_USER);
    match hkcu.open_subkey("Software\\MyRustBackupApp") {
        Ok(subkey) => subkey.get_value::<String, _>("StartupConfigured").is_ok(),
        Err(_) => false,
    }
}

fn mark_startup_configured() -> io::Result<()> {
    let hkcu = RegKey::predef(HKEY_CURRENT_USER);
    let (key, _) = hkcu.create_subkey("Software\\MyRustBackupApp")?;
    key.set_value("StartupConfigured", &"true")?;
    Ok(())
}

pub fn is_windows_startup_configured () {
    if !is_startup_configured() {
        if let Err(e) = setup_windows_startup() {
            eprintln!("Errore nella configurazione dell'avvio automatico: {:?}", e);
        } else {
            println!("Avvio automatico configurato con successo.");
            mark_startup_configured().expect("Errore nel marcare la configurazione dell'avvio.");
        }
        println!("Sono dentro startup");
    }
}


pub fn config_boot() -> Result<Config, Box<dyn Error>> {
    
    match load_config("config.toml") {
        Ok(config) => {
            println!("Configurazione caricata correttamente: {:?}", config);
            Ok(config)
        },
        Err(e) => {
            eprintln!("Errore nel caricamento della configurazione: {}", e);
            Err(e)
        },
    }
    
}