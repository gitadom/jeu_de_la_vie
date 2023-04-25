use std::path::Path;

use sdl2;
use sdl2::rect::Rect;
use sdl2::ttf::{FontStyle, Sdl2TtfContext};
use sdl2::render::{Canvas, TextureCreator};
use sdl2::pixels::Color;
use sdl2::surface::Surface;
use sdl2::video::{Window, WindowContext};



pub fn ecriture (ttf_context: &Sdl2TtfContext, path: &String, h_lettre:u16,
                style:FontStyle, texte:String, color: Color) -> Surface<'static> {
    // Création d'une surface
    // pour une texture
    let le_path = Path::new(path);
    let mut font = ttf_context.load_font(le_path, h_lettre).unwrap();
    font.set_style(style);
    let surface = font
                    .render(&texte)
                    .blended(color)
                    .map_err(|e| e.to_string()).unwrap();
    surface
}

pub fn texture_in_canvas (texture_creator:&TextureCreator<WindowContext>, surface: &Surface<'static>,
                          canvas:& mut Canvas<Window>, rec : Rect) {
    // Retourne une texture
    // pour la copie dans le canvas
    let texture = texture_creator.create_texture_from_surface(surface)
                    .map_err(|e| e.to_string()).unwrap();
    canvas.copy(&texture, None, Some(rec)).unwrap();
}
