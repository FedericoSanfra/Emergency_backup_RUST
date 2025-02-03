# **Rust Project: Emergency Backup**

## **Project Description**
This project is a **Rust**-based application for PC that allows performing a backup to an external drive (USB stick) when the screen is unresponsive. Interaction occurs via conventional mouse commands.

---

## **Main Features**
1. **Emergency Backup via Mouse:**
    - Activating the backup by drawing a rectangle along the edges of the screen (screen size is dynamically detected).
    - Confirming the backup via a second mouse command by drawing a minus sign at the edges of the screen.

2. **Backup Configuration:**
    - Defining the backup source in the `config.toml` file.
    - Option to select different types of backups:
        - Backup of the entire folder.
        - Backup of files of a specific type: if no extension is specified, all file types will be backed up.

3. **Visual Confirmation Interface:**
    - A graphical window is shown to confirm the rectangle command has been correctly detected (window closes automatically after 5 seconds).

4. **Periodic Search for a Valid USB Device:**
    - The system periodically searches for a valid storage device to save files to. Once found, the backup operation proceeds.
      (Search starts every 5 seconds to allow time for the user to connect the device).

5. **Audio Confirmation Playback:**
    - An audio file is played to signal both commands have been detected in case the screen is not responsive.

6. **Resource Optimization:**
    - The application runs in the background with minimal CPU usage. A `condvar` is used to communicate between two threads, with the main thread managing the graphical interface.
    - CPU usage is logged every **2 minutes** in a log file, for a single core or normalized across all logical threads.

7. **Final Log:**
    - A log file is created on the USB stick at the end of the backup containing:
        - Total size of files saved.
        - CPU time spent completing the backup.

8. **Automatic Startup:**
    - The application is installed during the PC bootstrap process.

---

## **Project Requirements**
- **Technology Used:** Rust programming language.
- **Target Operating System:** Windows.

---

## **Code Structure**
### **1. Main Modules**
- `main.rs`: The entry point of the application.
- `mouse_listener.rs`: Manages mouse-drawn commands and the graphical confirmation interface.
- `backup.rs`: Backup logic (source, destination, filters).
- `find_usb.rs`: USB device search.
- `config_boot.rs`: Manages the configuration file.

### **2. Folders**
- **`src/`**:
    - `main.rs`
    - `mouse_listener.rs`
    - `backup.rs`
    - `find_usb.rs`
    - `config_boot.rs`
- **`logs/`**:
    - CPU usage log files.
    - Final log created after backup completion in the destination folder.
- **`config/`**:
    - Configuration file for the backup source.

---

## **USER MANUAL**
1. **Setup:**
    - Download and run the project.
    - Modify the `config.toml` file to insert the path to the backup source folder.
    - Modify the `config.toml` file to specify file types to be transferred (if the array is empty, all file types will be transferred).
    - Insert a valid USB stick with available space.

2. **Launching the Application:**
    - The application is launched automatically during system bootstrap.
    - It remains active in the background with minimal resource consumption.

3. **Recognition of Activation Command:**
    - Draw a rectangle along the edges of the screen with the mouse.
    - The application verifies the command and triggers the graphical confirmation interface, which closes automatically after 5 seconds.

4. **Backup Confirmation:**
    - Confirm the backup by drawing a minus sign from one edge of the screen to the other.
    - If the command is recognized, the application plays an audio file as a confirmation alert.

5. **Executing the Backup:**
    - The application searches for a valid USB stick and loads the configuration settings.
    - Files are copied from the specified source to the external drive, filtered according to the set parameters.

6. **Final Log Creation:**
    - At the end of the backup, a log file is created on the USB stick containing:
        - The total size of saved files.
        - CPU time used.

7. **CPU Usage Monitoring:**
    - Every 2 minutes, the application logs CPU usage in a log file in the project folder.

---

## **Configuration File**
File format:
```toml
{
 source_path = 'Path\\to\\source\\'
 file_types = ["*.txt", "*.pdf"]  # file types to include, if empty, all file types are accepted
}
