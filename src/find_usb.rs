use sysinfo::{System, SystemExt, DiskExt};



pub fn find_usb_disks() -> Option<String> {
    let mut system = System::new_all();
    system.refresh_disks_list();
    for disk in system.disks() {
        println!("Disk: {:?}", disk.name());
        println!("Mount Point: {:?}", disk.mount_point());
        println!("Is Removable: {}", disk.is_removable());
        println!("Total Space: {} bytes", disk.total_space());
        println!();
    }

    for disk in system.disks() {
        if disk.is_removable() {
            if let Some(path) = disk.mount_point().to_str() {
                return Some(path.to_string());
            }
        }
    }


    None

    
}