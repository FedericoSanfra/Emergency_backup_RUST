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

// Funzione per riprodurre un suono di conferma
fn play_confirmation_sound() {
    let (_stream, stream_handle) = rodio::OutputStream::try_default().unwrap();
    let sink = rodio::Sink::try_new(&stream_handle).unwrap();
    let exe_path = env::current_exe().expect("Impossibile ottenere il percorso dell'eseguibile");
    let project_root = exe_path
        .parent() // target/debug/
        .unwrap()
        .parent() // target/
        .unwrap()
        .parent() // Group14/
        .unwrap();
    let file = std::io::BufReader::new(File::open(project_root.join("confirmation_sound.mp3")).expect("File audio non trovato"));
    let source = rodio::Decoder::new(file).unwrap();

    // Imposta il volume tramite il sink
    sink.set_volume(0.1); // Imposta il volume a 10%

    sink.append(source);
    sink.sleep_until_end();
}

fn main() {
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

    // Ottieni le dimensioni dello schermo una sola volta
    let (screen_width, screen_height) = get_screen_dimensions();

    config_boot::is_windows_startup_configured();
    // Thread per monitorare l'utilizzo della CPU

    // Ottieni il PID del processo corrente e convertilo in `Pid`
    let current_pid = Pid::from(std::process::id() as usize);

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
                thread::sleep(Duration::from_secs(5));
                continue; // Riprova il ciclo
            }
        }; //rileva files

        // Ciclo per trovare la chiavetta USB  STEP 3
        let destination = loop {
            let result = find_usb::find_usb_disks().unwrap_or_else(|| "".to_string());

            if !result.is_empty() {
                break result; // Esce dal ciclo se la chiavetta viene trovata
            } else {
                println!("Disk not found. Retrying in 5 seconds...");
                thread::sleep(Duration::from_secs(5)); // Attende prima di riprovare
            }
        };

        // Backup completato
        println!("Disk found:  {}", destination);

        // Riproduce il suono di conferma
        play_confirmation_sound();

        backup::execute(config, &destination).expect("Errore durante il backup");


        // Messaggio di completamento
        println!("Backup completato. Rilancio il processo...");
    }
}