

pub mod path {
    use crate::os_info::name_os::is_type_os::is_windows;

    pub fn path_new(name_case:&str, name_file:&str) -> String {
        // contrôle l'os soit sythéme windows
        // soit systhéme unix
        // retourne une string de adresse courante plus le dossier et fichier
        // si le nom du dossie = "" ne met que le nom du fichier
        
        if name_case != "" {
            //let mut case = String::new();
            if is_windows() {
                //eprintln!("Dans path_new windows case = {}, fichier == {}", case, file);
                path_windows(name_case, name_file)
            } else {
                //eprintln!("Dans path_new unix case = {}, fichier == {}", case, file);
                path_unix(name_case, name_file)
            }
        } else {
            if is_windows() {
                path_windows_file(name_file)
            } else {
                path_unix_file(name_file)
            }
        }
        
    }

    fn path_windows(case:&str, file:&str) -> String {
        // Retour une string de l'adresse plus le nom du dossier et du fichier
        // l'adresse et adresse courante au format Windows
            match crate::os_info::name_os::env::dir() {
                Ok(path) => format!("{}\\{}\\{}", path.to_string_lossy(),
                                            case, file),
                Err(_) => "\\".to_string(),
            }
            
        }

    fn path_unix(case:&str, file:&str) -> String{
        // Retour une string de l'adresse plus le nom du dossier et du fichier
        // l'adresse et adresse courante au format unix
        match crate::os_info::name_os::env::dir() {
            Ok(path) => format!("{}/{}/{}", path.to_string_lossy(),
                                        case, file),
            Err(_) => "/".to_string(),
         }
    }

    fn path_windows_file(file:&str) -> String {
        // Retour une string de l'adresse plus le nom du dossier et du fichier
        // l'adresse et adresse courante au format Windows
            match crate::os_info::name_os::env::dir() {
                Ok(path) => format!("{}\\{}", path.to_string_lossy(),file),
                Err(_) => "\\".to_string(),
            }
            
        }

    fn path_unix_file(file:&str) -> String{
        // Retour une string de l'adresse plus le nom du dossier et du fichier
        // l'adresse et adresse courante au format unix
        eprintln!("Dans path_unix_file fichier == {}", file);
        match crate::os_info::name_os::env::dir() {
            Ok(path) => format!("{}/{}", path.to_string_lossy(), file),
            Err(_) => "/".to_string(),
         }
    }
}