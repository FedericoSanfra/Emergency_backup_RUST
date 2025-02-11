#![cfg_attr(feature = "windows_subsystem", windows_subsystem = "windows")]

use std::fs::File;
use std::thread;
use std::time::Duration;
use sysinfo::{CpuExt, Pid, ProcessExt, System, SystemExt};
use std::io::Write;
use std::path::Path;
use std::env;
use std::io::BufReader;
use rodio::{OutputStream, Sink, source::Source};
use winit::event_loop::EventLoop;

mod mouse_listener;
pub mod backup;
mod config_boot;
mod find_usb;
mod utils;

// Funzione per rilevare dinamicamente le dimensioni dello schermo
fn get_screen_dimensions() -> (f64, f64) {
    let event_loop = EventLoop::new();
    if let Some(primary_monitor) = event_loop.primary_monitor() {
        let screen_size = primary_monitor.size();
        println!("Risoluzione dello schermo rilevata: {}x{}", screen_size.width, screen_size.height);
        return (screen_size.width as f64, screen_size.height as f64);
    } else {
        eprintln!("Impossibile rilevare le dimensioni dello schermo. Utilizzo i valori di fallback.");
        (1920.0, 1080.0) // Valori di fallback
    }
}


fn main() {
    
    // Ottieni le dimensioni dello schermo una sola volta
    let (screen_width, screen_height) = get_screen_dimensions();

    config_boot::is_windows_startup_configured();
    // Thread per monitorare l'utilizzo della CPU

    // Ottieni il PID del processo corrente e convertilo in `Pid`
    let current_pid = Pid::from(std::process::id() as usize);
    let mut config_error = false;
    let mut usb_error = false;
    // Thread per monitorare l'utilizzo della CPU utilizzata dal programma
    let _cpu_usage = thread::spawn(move || {
        let mut system = System::new_all();
        let cpu_count = system.cpus().len(); // Numero di core logici
        let exe_path = env::current_exe().expect("Impossibile ottenere il percorso dell'eseguibile");
        let project_folder_path = exe_path
            .parent() // target/debug/
            .unwrap()
            .parent() // target/
            .unwrap()
            .parent() // Group14/
            .unwrap();
        let cpu_usage_path = project_folder_path.join("cpu_usage.txt");

        // Crea il file di log
        if let Ok(mut file) = File::create(cpu_usage_path) {
            thread::sleep(Duration::from_secs(2)); //partire con informazioni sufficienti per il calcolo differenziale del consumo CPU
            loop {
                // Aggiorna lo stato del sistema per il processo corrente
                system.refresh_process(current_pid);

                // Trova il processo corrente
                if let Some(process) = system.process(current_pid) {
                    let raw_cpu_usage = process.cpu_usage();
                    let normalized_cpu_usage = raw_cpu_usage / cpu_count as f32;
                    writeln!(file, "CPU Usage (raw): {:.2}%, Normalized: {:.2}%", raw_cpu_usage, normalized_cpu_usage).expect("Writing file unsuccesful");
                } else {
                    writeln!(file, "Processo non trovato.").unwrap();
                }

                // Attendi 2 minuti prima di rilevare di nuovo
                thread::sleep(Duration::from_secs(120));
            }
        }
    });

    // Ciclo principale
    loop {

        // Ascoltatore del mouse  STEP 1
        mouse_listener::listener(screen_width, screen_height);

        // Carica la configurazione  STEP 2
        let config = match config_boot::config_boot() {
            Ok(config) => config,
            Err(e) => {
                eprintln!("Errore nella configurazione: {:?}. Ritento in 5 secondi...", e);
                if !config_error {
                    utils::play_sound("config_error.mp3");
                    config_error=true;
                }
                thread::sleep(Duration::from_secs(5));
                continue; // Riprova il ciclo
            }
        }; //rileva files

        let filtered_bytes=utils::get_folder_size(&config.source_path, &config.file_types).expect("Errore nel calcolare lo spazio occupato dalla source folder.");
        println!("bytes filtrati: {:?}", &filtered_bytes);


        // Ciclo per trovare la chiavetta USB  STEP 3
        let destination = loop {
            let result = find_usb::find_usb_disks(filtered_bytes).unwrap_or_else(|| "".to_string()); //passiamo i bytes richiesti come parametro
            // a find usb per fare il check

            if !result.is_empty() {
                break result; // Esce dal ciclo se la chiavetta viene trovata
            } else {
                println!("Disk not found. Retrying in 5 seconds...");
                if !usb_error {
                    utils::play_sound("usb_error.mp3");
                    usb_error=true;
                }
                thread::sleep(Duration::from_secs(5)); // Attende prima di riprovare
            }
        };

        // Backup completato
        println!("Disk found:  {}", destination);

        // Riproduce il suono di conferma
        utils::play_sound("confirmation_sound.mp3");

        backup::execute(config, &destination).expect("Errore durante il backup");


        // Messaggio di completamento
        println!("Backup completato. Rilancio il processo...");
    }
}