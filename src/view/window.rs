// Bibliothèque  de std 
use std::time::Duration;
use std::thread;


// Bibliothèque SDL
use sdl2::mouse::MouseButton;
use sdl2::pixels::Color;
use sdl2::event::Event;
use sdl2::rect::Rect;
use sdl2::image::InitFlag;

// Fichier courant
use crate::cellule::map::Map;
use crate::os_info::path::path::*;
use crate::view::text::{ecriture, texture_in_canvas};

use crate::view::board_button;
use crate::view::monde::monde_sdl;
use crate::view::sdl_init::ttf_int;
use crate::view::sdl_init::sdl_init;

//#[derive(Copy)]

pub fn window (monde: &mut Map, titre: &str)-> Result<i32, String> {
    let mut automatique = false;
    let tempo = Duration::new(0, 99800000);
    let window_width: u32 = 980; // Largeur
    let window_height:u32 = 800; // Hauteur

    /*
        Non des images pour les bouttons dans la liste "name_button"
        Nom du dossier "case",
        Liste de tuple des positions des bouttons (colonne, ligne)
     */
    let name_button = ["bouton_new.png", "bouton_avance.png", 
                            "bouton_retour.png", "bouton_RAZ.png", "bouton_auto.png", "bouton_stop.png"];
    let name_case = "image";
    let locate_button = [(820, 10), (820, 77), (820, 144), (820, 211), (820, 278), (820, 345)];
    
    // initialisation de SDL2
    
    let (sdl_context, _video_sudsytheme , mut canvas ) = sdl_init(titre, &window_height, &window_width);
    
    let _image_context = match sdl2::image::init(InitFlag::PNG){
        Ok(context) => context,
        Err(err) => {return Err(err);}
    };

    // init texte
    let ttf_context = ttf_int();

    // adresse du font
    let path_font = path_new("open-sans", "OpenSans-Regular.ttf");
     // couleur des textes
    let text_color = Color::RGBA(128, 128, 0, 255);
    // Création de la surface texte génération
    let texte = ecriture(&ttf_context, &path_font, 18,
        sdl2::ttf::FontStyle::BOLD , "Génération".to_string(),
        text_color);
    // Création de la surface texte nb cellule
    let texte_nb_cell = ecriture(&ttf_context, &path_font, 18,
        sdl2::ttf::FontStyle::BOLD , "Cellules vivantes".to_string(),
        text_color);
    // Rectangle des textes
    let texte_rec = Rect::new(840, 450, 100, 40);
    let mut texte_nb_rec = Rect::new(878, 500, 20, 25);
    let texte_cellule_rec = Rect::new(825, 550, 130,45);
    //let mut texte_nb_cel_rec = Rect::new(878, 600, 20, 25);
    // Fond de la fenêtre
    let board_view = board_button::Renderer::new(Rect::new(0, 0, window_width, window_height),
                                                Color::RGB(50, 50, 51));
    
    // création du fond coté monde
    let map = board_button::Renderer::new(Rect::new(0, 0, 800, 800),
                                            Color::RGB(200, 200, 10));

    // Création de la boucle et affichage
    let mut liste_event = sdl_context.event_pump().unwrap(); // prise des evenements

    // le stop de la boucle
    let mut stop = false;

    // boucle de rendu
    while stop != true {

        board_view.render(&mut canvas);
        map.render(&mut canvas);

        for i in 0..name_button.len(){ // Affichage des boutons
            let button = board_button::Button::new(locate_button[i].0,
                                                    locate_button[i].1,
                                                    path_new(name_case, name_button[i]));
            button.button(&mut canvas);
        }
        // Affichage du texte 
        texture_in_canvas(&canvas.texture_creator(),  &texte, & mut canvas, texte_rec);
        // adaptation de affichage au nombre de génération
        match monde.get_generation() {
            g if g <=9 => {texte_nb_rec = Rect::new(878, 500, 20, 25);},
            g if g > 9 && g <= 99=> {texte_nb_rec  = Rect::new(866, 500, 45, 25);},
            g if g > 99 && g <= 999 => {texte_nb_rec  = Rect::new(853, 500, 70, 25);},
            g if g > 999 && g <= 9999 => {texte_nb_rec  = Rect::new(841, 500, 95, 25);},
            g if g > 9999 && g <= 99999 => {texte_nb_rec  = Rect::new(828, 500, 120, 25);},
            g if g > 99999 && g <= 999999 => {texte_nb_rec  = Rect::new(816, 500, 145, 25);},
            _ => {}
        };
        // Affichage du nombre de génération
        let texte_generation = ecriture(&ttf_context, &path_font, 50, // Création de la surface du texte chiffre génération
            sdl2::ttf::FontStyle::NORMAL , monde.get_generation().to_string(),
            text_color);

        texture_in_canvas(&canvas.texture_creator(),  &texte_generation, & mut canvas, texte_nb_rec );
        // Affichage pour les cellules vivante
        texture_in_canvas(&canvas.texture_creator(), &texte_nb_cell, &mut canvas, texte_cellule_rec);
        match monde.get_nb_cellule() {
            g if g <= 9 => {texte_nb_rec  = Rect::new(878, 600, 20, 25);},
            g if g > 9 && g <= 99=> {texte_nb_rec  = Rect::new(866, 600, 45, 25);},
            g if g > 99 && g <= 999 => {texte_nb_rec  = Rect::new(853, 600, 70, 25);},
            g if g > 999 && g <= 9999 => {texte_nb_rec  = Rect::new(841, 600, 95, 25);},
            _ => {}
        };
        let texte_nb_cell = ecriture(&ttf_context, &path_font, 50, // Création de la surface du texte chiffre génération
            sdl2::ttf::FontStyle::NORMAL , monde.get_nb_cellule().to_string(),
            text_color);
        texture_in_canvas(&canvas.texture_creator(),  &texte_nb_cell, & mut canvas, texte_nb_rec );
        if automatique {// passage des générations en automatique
            monde.new_generation();
            thread::sleep(tempo); // temporisation
        }
        monde_sdl(monde, &mut canvas);

        canvas.present();
        for event in liste_event.poll_iter() {
            match event {
                Event::Quit {..} => {stop = true;},
                Event::MouseButtonUp {mouse_btn:MouseButton::Left,
                                    x, y, .. } => {clic_button(x, y, locate_button, monde, &mut automatique); },
                _ => {}
            }
        }
        
    }


    Ok(1)
}

fn clic_button (x: i32, y: i32 , locate: [(i32, i32); 6], map : & mut Map ,  auto: &mut bool) {
    let a = 820;//dimention du boutton
    let b = 940;
    match (x, y) {
        (l, h) if ((l >= a && l <= b)
                         && (h>= locate[0].1 && h <= (locate[0].1 + 57))) => {// Bouton nouveau monde
                            *auto = false;
                            map.new_map();},
        (l, h) if ((l >= a && l <= b)
                         && (h>= locate[1].1 && h <= (locate[1].1 + 57))) => {// Bouton génération +1
                            map.new_generation();},
        (l, h) if ((l >= a && l <= b)
                        && (h>= locate[2].1 && h <= (locate[2].1 + 57))) => {// Bouton génération -1
                            *auto = false;
                            map.undo();},
        (l, h) if ((l >= a && l <= b)
                            && (h>= locate[3].1 && h <= (locate[3].1 + 57))) => {// Bouton remise a zéro
                            *auto = false;
                            map.raz();},
        (l, h) if ((l >= a && l <= b)
                        && (h>= locate[4].1 && h <= (locate[4].1 + 57))) => {// Bouton automatique
                            *auto = true;},
        (l, h) if ((l >= a && l <= b)
                        && (h>= locate[5].1 && h <= (locate[5].1 + 57))) => {// Bouton stop
                            *auto = false;},
    
        (l, h) if ((l >= 0 && l <= 800) 
                         && (h >= 0 && h <= 800)) => {// Sur la carte et invése l'etat de la cellule
                        let hauteur = y as u32/10; let colonne = x as u32/10;
                        let index = (hauteur * 80)+ colonne;
                        map.switch_cellule(index as i32);},
        
         
        _ => { /*println!("clic dehors -> {:?}  y-> {:?}", x, y); */ },
    };
}

