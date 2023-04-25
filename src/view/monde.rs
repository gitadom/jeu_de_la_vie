use sdl2::video::Window;
use sdl2::render::Canvas;
use sdl2::pixels::Color;
use sdl2::rect::Rect;

use crate::cellule::map::Map;
use crate::view::board_button::Renderer;
#[allow(dead_code, unused)]
pub fn monde_sdl(monde:&mut  Map, canvas: &mut Canvas<Window>) {
    let mut index = 0;
    let mut case_ha = 0;
    let mut case_co = 0;
    for hauteur in 0..=80 {
        for colonne in 0..=80 {
            index = (hauteur * 80) + colonne;
            case_ha = hauteur * 10;
            case_co = colonne * 10;
            let rec = Rect::new(case_co, case_ha, 10, 10);
            let couleur: sdl2::pixels::Color;
            if monde.get_tableau(index) {
                couleur = Color::RGB(0, 0, 0);
            }
            else {
                couleur = Color::RGB(128, 128, 0);
            }
            let rendu = Renderer::new(rec, couleur);
            rendu.render(canvas);
        }
    }

}