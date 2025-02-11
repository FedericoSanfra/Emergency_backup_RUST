# **Progetto 2.1: Back-up di emergenza**

## **Descrizione del progetto**
Realizzazione di un'applicazione in linguaggio **Rust** per PC che consente di effettuare un back-up su un disco esterno (chiavetta USB) nel caso in cui lo schermo non sia agibile. L'interazione avviene tramite comandi convenzionali con il mouse.

---

## **Struttura del codice**
### **1. Moduli principali**
- `main.rs`: Punto di ingresso dell'applicazione.
- `mouse_listener.rs`: Gestione dei comandi tracciati con il mouse e interfaccia grafica di conferma.
- `backup.rs`: Logica di backup (sorgente, destinazione, filtri).
- `find_usb.rs`: Ricerca dispositivo USB.
- `config_boot.rs`: Gestione del file di configurazione.
- `utils.rs`: Metodi statici per operazioni e controlli in comune tra files.

### **2. Cartelle**
- **`src/`**:
    - `main.rs`
    - `mouse_listener.rs`
    - `backup.rs`
    - `find_usb.rs`
    - `config_boot.rs`
    - - `utils.rs`
- **`Group14/`**:
    - `cpu_usage.txt`: File di log del consumo di CPU.
    - `backup_log.txt`: Log finale creato al termine del backup nella cartella di destinazione.
    - `config.toml`: File di configurazione per la sorgente del backup.

---

## **USER MANUAL**
1. **Setup:**
    - download del progetto ed eseguirlo.
    - modificare il file config.toml inserendo il percorso della cartella sorgente del backup.
    - modificare il file config.tom inserendo i tipi di file da trasferire ( se il vettore è vuoto, verranno trasferiti tutti ).
    - inserire una chiavetta USB valida con spazio disponibile.
1. **Avvio dell'applicazione:**
    - L'applicazione viene avviata automaticamente durante il bootstrap del sistema operativo.
    - Rimane attiva in background consumando il minimo delle risorse.

2. **Riconoscimento del comando di attivazione:**
    - Tracciare un rettangolo con il mouse ai bordi dello schermo.
    - L'applicazione verifica il comando e attiva la l'interfaccia grafica di conferma che si chiuderà dopo 5 secondi in automatico.

3. **Conferma del backup:**
    - Confermare il backup tramite un secondo comando, ovvero un segno meno da un estremo all'altro dello schermo.
    - Loading del file di configurazione e ricerca unità USB. Se il loading del file di configurazione non va a buon fine (es. percorso source non valido), l'utente deve ripetere i comandi di attivazione sullo schermo.

4. **Esecuzione del backup:**
    - Se il file di configurazione è caricato correttamente, viene ricercata l'unità USB, in caso di unità non trovata o spazio disponibile non sufficiente, viene riprodotto un audio di errore.
    - Ogni 5 secondi viene rieseguita la ricerca della chiavetta. 
    - Viene quindi eseguita una copia dei file dalla sorgente specificata al disco esterno, filtrando in base ai parametri impostati.

5. **Creazione del log finale:**
    - Al termine del backup, viene creato un file di log sulla chiavetta USB con:
        - Dimensione complessiva dei file salvati.
        - Tempo di CPU utilizzato.

6. **Monitoraggio del consumo di CPU:**
    - Ogni 2 minuti, l'applicazione registra il consumo di CPU in un file di log nella cartella del progetto.

---

## **File di configurazione**
Formato del file :

```
[config.toml]
source_path = "Path\\sorgente\\"  # Percorso della cartella di origine
file_types = ["*.txt", "*.pdf", "jpg"]   # Tipi di estensioni, se vuoto, verranno accettati tutti
```
---

## **Funzionalità principali**
1. **Backup di emergenza tramite mouse:**
    - Attivazione del backup mediante un comando attivabile tracciando un rettangolo ai bordi dello schermo (dimensioni dello schermo rilevate dinamicamente).
    - Conferma del backup tramite un secondo comando con il mouse attivabile disegnano un segno meno agli estremi dello schermo.

2. **Configurazione del backup:**
    - Definizione della sorgente del backup nel file config.toml.
    - Possibilità di selezionare diverse tipologie di backup:
        - Backup dell'intera cartella.
        - Backup di file di un determinato tipo: non mettendo un'estensione specifica, il backup verrà effettuato per tutti i tipi di file.

3. **Interfaccia visiva di conferma:**
    - Finestra grafica per segnalare che il comando del rettangolo è stato rilevato correttamente (chiusura finestra automatica dopo 5 secondi).

4. **Ricerca ciclica per un dispositivo USB valido su cui effettuare backup:**
    - Il sistema procede con la ricerca di un dispositivo di storage valido per salvare i file, appena lo trova procede l'operazione
        (la ricerca parte ogni 5 secondi, per dare il tempo all'utente di connettere il dispositivo). Se non la trova, la prima volta riprodurrà un file audio di notifica errore.
    - Il controllo verificherà che ci sia abbastanza spazio nell'unità di destinazione: calcola lo spazio necessario valutando solo i file che rientrano nelle estensioni indicate in config.toml.

5. **Riproduzione audio di conferma:**
    - File audio riprodotto per segnalare che entrambi i comandi sono stati rilevati, in caso di inagibilità dello schermo.
    - File audio riprodotto per segnalare unità in caso di unità USB non trovata o file di configurazione non valido.

6. **Ottimizzazione delle risorse:**
    - Applicazione attiva in background con consumo minimo di CPU. Per ottenere ciò è stata utilizzata una condvar per
        comunicare tra i due thread, di cui uno, quello principale, gestisce l'interfaccia grafica.
    - Salvataggio del consumo di CPU utilizzata dal programma ogni **2 minuti** in un file di log, per il singolo core o normalizzata su tutti i thread logici.

7. **Log finale:**
    - Creazione di un file di log sulla chiavetta USB al termine del backup che include:
        - Dimensione totale dei file salvati.
        - Tempo di CPU impiegato per completare il backup.

8. **Avvio automatico:**
    - Installazione dell'applicazione durante il bootstrap del PC.

---

## **Requisiti del progetto**
- **Tecnologia utilizzata:** Linguaggio Rust.
- **Sistema operativo target:** Windows.

---

# **DETTAGLI TECNICI**

## Mouse Listener

Il progetto utilizza un listener per il mouse e un'interfaccia grafica per gestire un'applicazione che avvia un processo di backup in seguito a due comandi dell'utente: disegnare un rettangolo e disegnare un segno meno. L'applicazione mostra una finestra di conferma quando i comandi vengono rilevati.

### 1. Architettura e Struttura Generale

L'applicazione è strutturata attorno a tre componenti principali:
- **Un listener per il mouse** che rileva gli eventi di disegno.
- **Un'applicazione `druid` con interfaccia grafica** che visualizza una finestra di conferma e interagisce con l'utente.
- **Una logica di gestione dello stato** che sincronizza il flusso del programma.

### 2. Componente Mouse Listener

La logica di rilevamento è la seguente:

- **Rettangolo**: Se l'utente disegna una forma che si avvicina ai bordi dello schermo, viene interpretata come un rettangolo.
- **Segno meno**: Se l'utente disegna una linea orizzontale vicino al bordo superiore o inferiore dello schermo, viene riconosciuto come un segno meno.

La logica di rilevazione si basa su una serie di punti che vengono memorizzati durante il movimento del mouse (`points.push((x, y))`). La funzione `check_for_rectangle` verifica se i punti tracciati formano un rettangolo, mentre `check_minus_sign` verifica la presenza di un segno meno.

Dopo aver riconosciuto un rettangolo, il programma aggiorna lo stato e notifica gli altri thread utilizzando una variabile condizionale (`Condvar`). Il programma attende il segno meno per procedere.

### 3. Componente UI (Interfaccia Grafica)

L'interfaccia utente è creata utilizzando la libreria `druid`. La finestra mostra un'etichetta con le istruzioni per l'utente e un widget personalizzato chiamato `TimerWidget`. Questo widget gestisce un timer di 5 secondi dopo la connessione della finestra, dopodiché chiude automaticamente la finestra.

Il layout dell'interfaccia è composto da un'etichetta di testo (con le istruzioni per l'utente) e un widget del timer, il tutto disposto in un layout verticale (`Flex::column`).

### 4. Gestione dello Stato dell'Applicazione

L'applicazione tiene traccia dello stato del processo tramite una variabile condivisa, protetta da un mutex (`Arc<Mutex<i32>>`). Questo stato è aggiornato durante l'interazione con il mouse e determina quando il programma deve eseguire determinate azioni:

- **Stato 1**: L'utente sta aspettando di disegnare un rettangolo.
- **Stato 2**: Il rettangolo è stato disegnato, l'utente può ora disegnare il segno meno.
- **Stato 3**: Il segno meno è stato disegnato, l'operazione di backup è pronta per essere eseguita.

La variabile condizionale (`Condvar`) è utilizzata per sincronizzare i thread, permettendo al thread principale di attendere che l'utente completi le operazioni (disegnare il rettangolo e il segno meno) prima di procedere.

### 5. Componente di Gestione Finestra e Comandi

La finestra dell'applicazione è configurata utilizzando `druid::WindowDesc`, che definisce il layout iniziale e le dimensioni della finestra. La finestra è gestita tramite una delega (`AppDelegate`), che esegue operazioni personalizzate quando la finestra viene aggiunta o connessa. In particolare, quando la finestra è connessa, viene avviato un timer. Quando il timer scade, la finestra viene chiusa automaticamente (tramite `CLOSE_ALL_WINDOWS`).

### 6. Gestione del Flusso di Lavoro

Il flusso di lavoro dell'applicazione si sviluppa in tre fasi principali:
1. **Fase 1**: Il programma avvia il listener per il mouse e attende che l'utente disegni un rettangolo.
2. **Fase 2**: Dopo aver rilevato il rettangolo, il programma aspetta che l'utente disegni un segno meno.
3. **Fase 3**: Una volta che il segno meno è stato rilevato, l'applicazione è pronta per procedere con l'operazione di backup. La finestra grafica può essere chiusa automaticamente dal timer o manualmente dall'utente.

### 7. Sincronizzazione tra Thread e UI

- **Thread di ascolto del mouse**: Il thread che ascolta gli eventi del mouse viene avviato separatamente dal thread principale dell'applicazione, usando `thread::spawn`. La sincronizzazione tra il thread principale e quello di ascolto del mouse è realizzata tramite `Arc<Mutex<T>>` e `Condvar`. Questo permette al thread principale di attendere che l'utente completi le operazioni (disegnare un rettangolo e un segno meno) prima di avviare il processo di backup.
- **Interazione con la UI**: La finestra di `druid` interagisce con la logica tramite il `TimerWidget`, che innesca eventi basati sul timer (ad esempio, chiudere la finestra automaticamente).

##  Caricamento della Configurazione
Il progetto gestisce la configurazione e l'avvio automatico di un'applicazione su Windows. La configurazione viene letta da un file TOML e il programma si assicura che l'applicazione venga eseguita automaticamente all'avvio di Windows.
La configurazione dell'applicazione viene caricata da un file TOML. Il file contiene informazioni come il percorso di origine dei file e i tipi di file supportati per l'operazione di backup. La configurazione è rappresentata da una struttura di dati, che include:

- `source_path`: Il percorso di origine per i file da gestire.
- `file_types`: Un elenco di tipi di file supportati per l'operazione di backup.

Nel caso in cui il file non esista o ci siano errori nel caricamento, un audio di notifica errore viene riprodotto e l'utente dovrà rieseguire i comandi di attivazione una volta modificato il file di configurazione.

### 1. Configurazione dell'Avvio Automatico su Windows

L'applicazione può essere configurata per avviarsi automaticamente con Windows. Questo viene realizzato tramite la modifica del registro di sistema di Windows.

### 2. Aggiunta al Registro di Sistema

Il programma ottiene il percorso dell'eseguibile e lo aggiunge alla chiave del registro di sistema `HKEY_CURRENT_USER\Software\Microsoft\Windows\CurrentVersion\Run`. Questo garantisce che l'applicazione venga avviata automaticamente quando l'utente accede a Windows.

### 3. Verifica della Configurazione dell'Avvio

Viene verificato se l'avvio automatico è già configurato. Se l'avvio automatico non è stato configurato, il programma esegue la configurazione e segna la configurazione come completata nel registro di sistema.

### 4. Registro di Sistema

Il programma crea una chiave nel registro di sistema `Software\MyRustBackupApp` per segnare che l'avvio automatico è stato configurato. In seguito, una volta completata la configurazione, viene aggiornata la chiave con un valore che indica che la configurazione è stata eseguita correttamente.

### 3. Funzione di Avvio della Configurazione

Durante l'avvio dell'applicazione, viene caricata la configurazione dal file `config.toml`. Se il file viene caricato correttamente, viene restituita la configurazione. Se si verifica un errore durante il caricamento, viene restituito un errore che descrive il problema.

### 4. Dipendenze Utilizzate

Il codice fa uso delle seguenti librerie:

- **`serde`**: Utilizzata per la deserializzazione del file TOML in una struttura di dati Rust.
- **`winreg`**: Utilizzata per interagire con il registro di sistema di Windows, per configurare l'avvio automatico.
- **`toml`**: Utilizzata per il parsing dei file di configurazione in formato TOML.
- **`std`**: La libreria standard di Rust, utilizzata per le operazioni di file I/O, gestione dell'ambiente e manipolazione delle stringhe.

### 5. Flusso di Lavoro

1. All'avvio, il programma carica la configurazione dal file `config.toml`.
2. Verifica se l'applicazione è configurata per l'avvio automatico. Se non lo è, la configura nel registro di sistema di Windows.
3. Segna nel registro che l'avvio automatico è stato configurato.
4. Il programma esegue le operazioni di backup come specificato nella configurazione, utilizzando il percorso e i tipi di file forniti.

## Esecuzione Backup

Il modulo di backup copia file e directory dalla sorgente alla destinazione, basandosi su una configurazione che specifica i tipi di file da includere nel backup. Inoltre, il modulo genera un file di log che registra il successo dell'operazione, la dimensione totale dei file copiati e il tempo impiegato.

### 1. Funzione `execute`

La funzione `execute` è il punto di ingresso principale per eseguire il backup. Essa accetta due parametri:

- **`config`**: Una struttura di configurazione che contiene il percorso della sorgente e i tipi di file da includere nel backup.
- **`destination`**: Il percorso di destinazione dove i file saranno copiati.

### 2. Funzione `backup`

La funzione `backup` è responsabile dell'effettiva copia dei file dalla sorgente alla destinazione, tenendo conto della configurazione e dei tipi di file specificati. Essa accetta i seguenti parametri:

- **`source`**: Il percorso della directory di origine.
- **`destination`**: Il percorso della directory di destinazione.
- **`config`**: La configurazione che contiene i tipi di file accettati.

### 3. Flusso della funzione `backup`:

1. Utilizza la libreria `walkdir` per iterare attraverso la directory di origine, esplorando ricorsivamente tutte le sottodirectory.
2. Per ogni file trovato, verifica se il file è valido in base all'estensione e ai tipi di file specificati nella configurazione, tramite la funzione `is_valid_file`.
3. Se il file è valido, viene copiato nella destinazione, mantenendo la stessa struttura di directory.
4. Se l'elemento è una directory, viene creata una corrispondente directory nella destinazione.
5. Alla fine del backup, viene creato un file di log (`backup_log.txt`) nella destinazione, contenente:
    - Il messaggio di completamento del backup.
    - La dimensione totale dei file copiati.
    - Il tempo totale impiegato per il backup.

### 4. Funzione `is_valid_file`

La funzione `is_valid_file` verifica se un file deve essere incluso nel backup in base alla sua estensione. Essa accetta i seguenti parametri:

- **`file_path`**: Il percorso del file.
- **`config`**: La configurazione che contiene i tipi di file accettati.

### 5. Flusso della funzione `is_valid_file`:

1. Se la configurazione non specifica tipi di file (ovvero la lista `file_types` è vuota), il file è automaticamente considerato valido.
2. Se sono specificati tipi di file, viene confrontata l'estensione del file con i pattern presenti nella configurazione. Se c'è una corrispondenza, il file è considerato valido.

### 6. Funzione `match_file_extension`

La funzione `match_file_extension` confronta l'estensione di un file con un pattern di estensione fornito (es. `*.txt`).

### 7. Flusso della funzione `match_file_extension`:

1. Se il pattern inizia con un asterisco (`*`), viene verificato se l'estensione del file termina con il suffisso indicato dal pattern (ad esempio, `*.txt` verifica se l'estensione è `.txt`).
2. Se il pattern non è un wildcard, viene effettuato un confronto diretto tra l'estensione del file e il pattern.

### 8. Flusso di Lavoro

1. L'utente fornisce una configurazione (inclusi il percorso della sorgente e i tipi di file da includere nel backup).
2. La funzione `execute` avvia il processo di backup.
3. La funzione `backup` esplora la sorgente, copia i file validi e crea le directory corrispondenti nella destinazione.
4. Viene generato un file di log contenente informazioni sul backup.

## Loop nel Main

Nel file main.rs, viene gestito tutto il flusso di lavoro, l'inizializzazione di alcuni parametri e il loop principale che continuerà ad essere eseguito in background.

### 1. Riproduzione di Suoni di Notifica

La funzione `play_confirmation_sound()` utilizza la libreria [`rodio`] per riprodurre un file audio (`confirmation_sound.mp3`) situato nella directory principale del progetto.
- Viene inizializzato un flusso audio con `rodio::OutputStream::try_default()`.
- Il file audio viene aperto e decodificato tramite `rodio::Decoder`.
- Il volume viene impostato al 10% per evitare livelli sonori troppo elevati.
- Il suono viene riprodotto e il programma attende la fine della riproduzione prima di continuare.
- Due file audio vengono riprodotti per notificare l'utente in caso di errore nel loading del file di configurazione o nel caso di unità USB non trovata o con spazio insufficiente. Questo aiuta l'utente con schermo inagibile a procedere nell'operazione di backup.

### 2. Rilevamento Dinamico delle Dimensioni dello Schermo

La funzione `get_screen_dimensions()` sfrutta la libreria [`winit`] per determinare la risoluzione dello schermo del sistema:
- Viene creato un `EventLoop` per accedere alle informazioni del monitor primario.
- Se il monitor è rilevato correttamente, vengono restituite le dimensioni in pixel.
- In caso di errore, vengono usati valori di fallback di `1920x1080`.

### 3. Calcolo Spazio Necessario per l'unità USB

La funzione `get_folder_size(bytes_needed: u64)` calcola i bytes necessari in base al vettore di estensioni nel file 'config.toml':
- Il numero di bytes che dovranno essere trasferiti viene passato come parametro alla funzione.
- In `find_usb.rs` avviene la ricerca in un ciclo for, dove viene selezionata l'unità USB che ha maggiore spazio sufficiente per ricevere i dati.
- In caso di spazio insufficiente, un audio viene riprodotto e la ricerca viene ripetuta dopo 5 secondi.

### 4. Monitoraggio dell'Uso della CPU

Un thread separato monitora il consumo della CPU del processo corrente utilizzando la libreria [`sysinfo`]:
- Si ottiene il `Pid` del processo attuale con `std::process::id()`.
- Il numero di core logici viene determinato tramite `system.cpus().len()`.
- I dati sul consumo della CPU vengono aggiornati ogni 2 minuti e scritti in un file di log (`cpu_usage.txt`).
- Il valore `raw_cpu_usage` rappresenta l'uso effettivo del processo, mentre `normalized_cpu_usage` fornisce un valore normalizzato rispetto al numero di core disponibili.


### 5. Dipendenze Utilizzate

- **`std::fs`**: Per operazioni di file I/O come la lettura, la scrittura e la copia dei file.
- **`std::time::Instant`**: Per misurare il tempo impiegato nel processo di backup.
- **`walkdir`**: Per l'esplorazione ricorsiva delle directory.
- **`std::io::Write`**: Per scrivere nel file di log.
- **`crate::config_boot::Config`**: Per caricare e utilizzare la configurazione del backup.






