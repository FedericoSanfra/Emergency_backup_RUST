use std::fs;
use std::fs::File;
use std::path::{Path, PathBuf};
use std::time::Instant;
use walkdir::WalkDir;
use std::io::Write;
use crate::config_boot::Config;

/// Copia un file o directory dalla sorgente alla destinazione.
pub fn execute(config: Config, destination: &String) -> std::io::Result<()> {
    let source = Path::new(&config.source_path);
    let destination = Path::new(destination);

    // Verifica che il percorso di sorgente esista.
    if !source.exists() {
        eprintln!("Il percorso sorgente non esiste!");
        return Ok(());
    }

    // Esegui il backup.
    match backup(source, destination, &config) {
        Ok(_) => println!("Backup completato con successo!"),
        Err(e) => eprintln!("Errore durante il backup: {}", e),
    }

    Ok(())
}

/// Funzione di backup che copia solo i file accettati in base alla configurazione.
fn backup(source: &Path, destination: &Path, config: &Config) -> std::io::Result<()> {
    let mut total_size: u64 = 0;
    let start_time = Instant::now(); // Conteggio del tempo trascorso
    for entry in WalkDir::new(source) {
        let entry = entry?;
        let entry_path = entry.path();


        // Controllo se è un file e se il tipo è accettato
        if entry_path.is_file() {
            //println!("is file");
            if is_valid_file(entry_path, config) {
                //println!("è valido");
                let relative_path = entry_path.strip_prefix(source);
                let dest_path = destination.join(relative_path.unwrap());

                // Copia il file nella destinazione
                total_size += entry.metadata()?.len();
                fs::copy(entry_path, &dest_path);
            }
        } else if entry_path.is_dir() {
            // Se è una directory, crea la directory nella destinazione
            let relative_path = entry_path.strip_prefix(source);
            let dest_path = destination.join(relative_path.unwrap());
            fs::create_dir_all(&dest_path);
        }
    }

    let elapsed_time = start_time.elapsed();

    // Crea un file di log nella cartella di destinazione con la dimensione totale.
    let log_path = destination.join("backup_log.txt");
    let mut log_file = File::create(log_path)?;
    writeln!(log_file, "Backup completato correttamente!")?;
    writeln!(log_file, "Dimensione totale dei file copiati: {} bytes", total_size)?;
    writeln!(log_file, "Tempo totale di backup: {:?}", elapsed_time)?;

    Ok(())
}

/// Funzione per determinare se un file è valido in base alla configurazione.
fn is_valid_file(file_path: &Path, config: &Config) -> bool {
    // Se non ci sono tipi di file specificati, accetta tutti i file
    if config.file_types.is_empty() {
        return true;
    }

    // Verifica se l'estensione del file è nella lista dei tipi accettati
    let file_extension = file_path.extension().and_then(|e| e.to_str()).unwrap_or("");
    for pattern in &config.file_types {
       // println!("file extension {:?} and pattern {:?}", file_extension, pattern);
        if match_file_extension(file_extension, pattern) {

            return true;
        }
    }

    // Se nessuna estensione corrisponde, il file non è valido
    false
}

/// Funzione per verificare se un file corrisponde a un pattern di estensione (es. "*.txt")
fn match_file_extension(extension: &str, pattern: &str) -> bool {
    // Confronta l'estensione con il pattern (es. "*.txt")
    if pattern.starts_with('*') {
        return extension.ends_with(&pattern[2..]); // es. "*.txt" -> "txt"
    }

    // Se il pattern non è un wildcard, confronta direttamente
    extension == pattern
}
