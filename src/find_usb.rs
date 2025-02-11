use sysinfo::{System, SystemExt, DiskExt};



pub fn find_usb_disks(bytes_needed: u64) -> Option<String> {
    let mut system = System::new_all();
    system.refresh_disks_list();


    let mut best_disk: Option<String> = None;
    let mut max_available_space: u64 = 0;

    for disk in system.disks() {
        if disk.is_removable() {
            if let Some(path) = disk.mount_point().to_str() {
                let available_space = disk.available_space();
                println!(
                    "Dispositivo: {:?}, Spazio disponibile: {} bytes, Necessario: {} bytes",
                    path, available_space, bytes_needed
                );

                // Controlla se la chiavetta ha spazio sufficiente
                if available_space >= bytes_needed && available_space > max_available_space {
                    max_available_space = available_space;
                    best_disk = Some(path.to_string());
                }
            }
        }
    }

    best_disk
}