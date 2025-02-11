use std::env;
use std::fs::File;
use std::path::Path;
use walkdir::WalkDir;

pub fn get_folder_size(folder_path: &String, file_types: &Vec<String>) -> std::io::Result<u64> {
    let mut total_size = 0;
    let path_source = Path::new(folder_path);

    for entry in WalkDir::new(path_source) {
        let entry = entry?;
        let metadata = entry.metadata()?;
        let file_path = entry.path();

        if metadata.is_file() {
            if file_types.is_empty() || file_path.extension()
                .and_then(|e| e.to_str())
                .map_or(false, |ext| file_types.iter().any(|pattern| match_file_extension(ext, pattern)))
            {
                total_size += metadata.len();
            }
        }
    }

    Ok(total_size)
}

/// Funzione per verificare se un file corrisponde a un pattern di estensione (es. "*.txt")
pub fn match_file_extension(extension: &str, pattern: &str) -> bool {
    // Confronta l'estensione con il pattern (es. "*.txt")
    if pattern.starts_with('*') {
        return extension.ends_with(&pattern[2..]); // es. "*.txt" -> "txt"
    }

    // Se il pattern non è un wildcard, confronta direttamente
    extension == pattern
}

// Funzione per riprodurre un suono di conferma
pub(crate) fn play_sound(path_file: &str) {
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
    let file = std::io::BufReader::new(File::open(project_root.join(path_file)).expect("File audio non trovato"));
    let source = rodio::Decoder::new(file).unwrap();

    // Imposta il volume tramite il sink
    sink.set_volume(0.1); // Imposta il volume a 10%

    sink.append(source);
    sink.sleep_until_end();
}