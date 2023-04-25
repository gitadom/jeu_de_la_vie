use rand::{thread_rng, Rng};

#[derive(Copy, Clone, Debug, PartialEq)]
enum Cellule {
    Dead,
    Alive
}

pub struct Map {
    tableau: [Cellule; 6400],
    generation : u32,
    undo: Vec<[Cellule; 6400]>,
    tableau_plein: bool,
    nombre_alive: u16
}


impl Map {

    pub fn new ()-> Map {
        // Création de la structure Map
        Map {
            tableau: [Cellule::Dead; 6400 as usize], // tableau en une dimention
            generation: 0,
            undo : Vec::new(),
            tableau_plein: false,
            nombre_alive: 0,
        }
    }
    
    pub fn undo (& mut self) {
        // Retour en arriere
        // on revient a la carte d'avant
        if self.generation > 0 {
            self.generation -= 1;
            self.tableau = match self.undo.pop() {
                Some(pop) => pop,
                _ => {
                    eprintln!("Erreur au pop dans fn moin");
                    [Cellule::Dead; 6400]
                }
            };
        }
        else {
            eprintln!("la génération et a 0");
        }
        self.compte_cellule();
    }

    pub fn raz(& mut self) {
        // Remise a zéro du monde
        self.undo.clear();
        self.generation = 0;
        self.nombre_alive = 0;
        for i in 0..self.tableau.len() {
            self.tableau[i] = Cellule::Dead;
        }
    }

    pub fn new_map(& mut self) { // Création d'un monde avec des cellules aléatoire
        let cellule_max = ((80.0*80.0)*0.25) as i32;
        let mut cellules = 0;
        let mut rng = thread_rng();
        if self.tableau_plein {
            self.tableau = [Cellule::Dead; 6400 as usize];
        }
        while cellules < cellule_max {
            let column: usize = rng.gen_range(0, 80);
            let height : usize = (rng.gen_range(0, 80)) * 80;
            let index = column + height;
            if self.tableau[index] == Cellule::Dead {
                self.tableau[index] = Cellule::Alive;
                cellules += 1; 
            }

        }
        if self.undo.len() > 0 {
            self.undo.clear();
        }
        if self.generation > 0 {
            self.generation = 0;
        }
        self.compte_cellule();
        self.tableau_plein = true;
    }

    pub fn get_generation (&self) -> u32 { // retourne la valeur de génération
        self.generation
    }
    pub fn get_nb_cellule (&self) -> u16 {
        self.nombre_alive
    }

    pub fn new_generation (&mut self) { // Calcule de la génération suivante
        let mut new_tableau = [Cellule::Dead; 6400 as usize];
        let mut tableau_compteur = [0; 6400];
        for h in 0..80 { // boucle hauteur
            for c in 0..80 { // boucle colonne
                let rep = (h * 80) + c ; // rep pour repére
                let mut compteur = 0; // compteur de cellule vivante
                // Partie Gauche et dessus
                if(rep - 1) >= 0 && self.get_tableau(rep - 1) {
                    compteur += 1;
                }
                if(rep - 80) >= 0 && self.get_tableau(rep - 80) {
                    compteur += 1;
                }
                if(rep - 81) >= 0 && self.get_tableau(rep - 81) {
                    compteur += 1;
                }
                if(rep - 79) >= 0 && self.get_tableau(rep - 79) {
                    compteur += 1;
                }
                // Partie droite et dessous
                if(rep + 1) < 6400 && self.get_tableau(rep + 1) {
                    compteur += 1;
                }
                if(rep + 80) < 6400 && self.get_tableau(rep + 80) {
                    compteur += 1;
                }
                if(rep + 81) < 6400 && self.get_tableau(rep + 81) {
                    compteur += 1;
                }
                if(rep + 79) < 6400 && self.get_tableau(rep + 79) {
                    compteur += 1;
                }
                // Remplissage du tableau du nombre de cellule a coté de la casse
                let rep = rep as usize;
                tableau_compteur[rep] = compteur;
            }
        }
        for i in 0..6400 {
            new_tableau[i] = self.tableau[i];
        }
        
        for h in 0..80 { // Boucle hauteur
            for c in 0..80 { // Boucle colonne
                let index: usize = (h * 80) + c;
                let valeur = tableau_compteur[index];
                if valeur == 3 && self.get_tableau(index as i32) == false {
                    new_tableau[index] = Cellule::Alive;
                }
                else if (valeur < 2 || valeur >= 4) && self.get_tableau(index as i32) {
                    new_tableau[index] = Cellule::Dead;
                }
            }
        }
        self.undo.push(self.tableau);
        self.tableau = new_tableau;
        self.generation += 1;
        self.compte_cellule();
    }

    pub fn  in_tableau (&self, index:i32) -> bool {
        // retourne le resulta true si l'on est dans le tableau
        // evite les sorties de tableau
        if index >= 0 && index < 6400 {
           true
        }
        else {
            false
        }
    }

    pub fn get_tableau(&self, index:i32) -> bool {
        // retourne true si la cellule et Alive sinon false
        if self.in_tableau(index) {
            if self.tableau[index as usize] == Cellule::Alive {
                true
            }
            else {
                false
            }
        }
        else {
            false
        }
        
    }

    pub fn switch_cellule(& mut self, index: i32) {
        /* 
            Change état de la cellule
            Alive -> Dead
            Dead -> alive
        */
        if self.in_tableau(index) {
            if self.tableau[index as usize] == Cellule::Alive {
                self.tableau[index as usize] = Cellule::Dead;
                return;
            }
            if self.tableau[index as usize] == Cellule::Dead {
                self.tableau[index as usize] = Cellule::Alive;
            }
        }
        self.compte_cellule();
    }
    
    fn compte_cellule(&mut self){ 
        // met le nombre de cellule vivant dans la valeur nombre_alive
        let mut nb_cellule: u16 = 0;
        for index in 0..6400 {
            if self.get_tableau(index){
                nb_cellule += 1;
            }
        }
        self.nombre_alive = nb_cellule;
    }
}