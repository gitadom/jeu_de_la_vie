use sdl2::render::Canvas;
use sdl2::video::Window;
use sdl2::image::LoadTexture;

use sdl2::pixels::Color;
use sdl2::rect::Rect;

// Structure du moteur de rendu
pub struct  Renderer {
    pub screen_area: Rect,
    pub clear_color: Color,
}

// Structure des image pour bouttons
pub struct Button {
    pub path: String, // Adresse de image
    pub width: i32,   // Largeur
    pub height: i32,  // Hauteur
}

impl Renderer {
    
    pub fn new(screen_area: Rect, clear_color: Color) -> Renderer {
        Renderer {
            screen_area,
            clear_color,
        }
    }

    pub fn render (&self, canvas: &mut Canvas<Window>) {
        // Mise du rendu dans le canvas
        canvas.set_draw_color(self.clear_color);
        canvas.fill_rect(self.screen_area).ok().unwrap_or_default();
    }
}

impl Button {
    
    pub fn new( width: i32, height: i32, path: String)-> Button {
        Button {
            path,
            width,
            height,
        }
    }

    pub fn button(&self, canvas: &mut Canvas<Window>) {
        // mise du boutton dans le canvas
        let largeur = 140; // dimention de l'image
        let hauteur = 57;
        //let path = Path::new(&self.path); // Adresse de image
        let icon_texture = canvas.texture_creator();
        let icon = match LoadTexture::load_texture(&icon_texture, &self.path) {
            Ok(retour)=> retour,
            Err(err) => {println!("Erreur path = {}", self.path);
                                panic!("{}", err);}
        };
        let src = Rect::new(0, 0, largeur, hauteur); // 
        let dst = Rect::new(self.width, self.height, largeur, hauteur);
        canvas.copy(&icon, src, dst).unwrap();
    }
}