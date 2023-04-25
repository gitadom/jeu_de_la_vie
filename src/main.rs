mod cellule;

mod os_info;
use crate::cellule::map::Map;
mod view;
use crate::view::window::window;

fn main() {
    let mut map = Map::new();
    let _retour = match window(&mut map, "Jeu de la vie") {
        Ok(_) => {},
        Err(er) => {println!("{}", er);
    }
     };
}
