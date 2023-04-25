/*
    besoin de os_info

    [dependencies]
    os_info = "3.7.0"
*/
#[allow(dead_code, unused_variables, non_snake_case, unreachable_patterns)]
pub mod is_type_os {
    // Appel des modules
    use os_info;
    use os_info::Type::*;

    // Fonction des demandes d'OS

    pub fn is_windows() -> bool { // OS et windows retourne true
        let info = os_info::get();
        match info.os_type(){
            Windows => true,
            _ => false,
        }
    }
    
    pub fn is_linux() -> bool { // OS et du Linux retourne true
        match os_info::get().os_type() {
            Alpaquita => true,
            Alpine => true,
            Amazon => true,
            Android => true,
            Arch => true,
            Artix => true,
            CentOS => true,
            Debian => true,
            EndeavourOS => true,
            Fedora => true,
            Garuda => true,
            Gentoo => true,
            Illumos => true,
            Linux => true,
            Mabox => true,
            Manjaro => true,
            Mint => true,
            NixOS => true,
            OpenCloudOS => true,
            OpenEuler => true,
            OpenSUSE => true,
            OracleLinux => true,
            Pop => true,
            Raspbian => true,
            Redhat => true,
            RedHatEnterprise => true,
            Redox => true,
            Solus => true,
            Suse => true,
            Ubuntu => true,
            _ => false
        }
        
    }

    pub fn is_bds() -> bool {
        match os_info::get().os_type() { // OS et du bds retourne true
            DragonFly => true,
            FreeBSD => true,
            HardenedBSD => true,
            MidnightBSD => true,
            NetBSD => true,
            _ => false,
        }
    }

    pub fn is_macos() -> bool { // OS et du mac os retourne true
        match os_info::get().os_type() {
            Macos => true,
            _ => false,
        }
    }

    pub fn is_inconnue() -> bool { // OS et inconnue retourne true
        match os_info::get().os_type() {
            Unknown => true,
            _ => false,
        }
    }

    pub fn is_type_unix() -> bool {
        if is_linux() {
            return true;
        }
        if is_macos() {
            return true;
        }
        if is_bds() {
            return true;
        }
        false
    }
}

pub mod env {
    // Appel des modules
    use std::io;
    use std::path::PathBuf;
    use std::env::current_dir;
    
    pub fn dir () -> io::Result<PathBuf> {
        current_dir()
    }
    
}
