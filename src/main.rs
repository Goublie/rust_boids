use ::macroquad::prelude::*;
use macroquad::ui::{hash, root_ui};
use Zone::*;
use ::rand::*;

const SIM_WIDTH  : f32 = 500.0;
const SIM_HEIGHT : f32 = 500.0;
const WINDOW_WIDTH : f32 = 500.0;
const WINDOW_HEIGHT : f32 = 750.0;

enum Zone {
    Repulsion,
    Orientation,
    Attraction,
    Liberte
}

struct TriangleIsocele {
    s         : Vec2, //Sommet principal
    hauteur   : f32, //hauteur du triangle
    vec_dir_h : Vec2,  //vecteur directeur hauteur du triangle
    largeur   : f32, //logueur de la base
}

impl TriangleIsocele{
    pub fn new(s:Vec2, hauteur:f32, largeur:f32) -> TriangleIsocele{
        TriangleIsocele{
            s:s,
            hauteur:hauteur,
            vec_dir_h:vec2(0.0, -1.0),
            largeur:largeur
        }
    }

    fn get_base_unitaire(&self) -> Vec2 {
        vec2(-self.vec_dir_h.y, self.vec_dir_h.x)
    }

    pub fn get_points(&self) -> (Vec2,Vec2,Vec2){
        let mut res = (vec2(0.0,0.0),vec2(0.0,0.0),vec2(0.0,0.0));
        res.0 = self.s;

        let vec_base_norm = self.get_base_unitaire();

        let m = self.s - self.vec_dir_h*self.hauteur;

        res.1 = m + vec_base_norm*self.largeur/2.0;
        res.2 = m - vec_base_norm*self.largeur/2.0;

        res
    }
}

struct Bird {
    forme               : TriangleIsocele,
    
    vec_vitesse         : Vec2,
    v_max               : f32,
    
    zone_repulsion      : f32,
    zone_orientation    : f32,
    zone_attraction     : f32,

    force_repulsion     : f32,
    force_attraction    : f32,
    force_orientation   : f32,
    facteur_chaos       : f32,
}

impl Bird{
    pub fn new(tete:Vec2) -> Bird{
        Bird{
            forme               : TriangleIsocele::new(tete, 8.0, 4.0),
            vec_vitesse         : vec2(0.0,0.0),
            v_max               : 5.0,
            zone_repulsion      : 40.0,
            zone_orientation    : 60.0,
            zone_attraction     : 80.0,
            force_attraction    : 0.1,
            force_repulsion     : 2.0,
            force_orientation   : 0.1,
            facteur_chaos       : 0.5,
        }
    }

    pub fn new_random() -> Bird{
        let position = vec2(random::<f32>()*SIM_WIDTH ,random::<f32>()*SIM_HEIGHT);
        Bird::new(position)
    }
    
    pub fn set_v_max(&mut self, v_max :f32){
        self.v_max=v_max;
    }

    pub fn set_zone_repulsion(&mut self, d :f32){
        self.zone_repulsion=d;
    }

    pub fn set_zone_orientation(&mut self, d :f32){
        self.zone_orientation=d;
    }

    pub fn set_zone_attraction(&mut self, d :f32){
        self.zone_attraction=d;
    }

    pub fn set_force_repulsion(&mut self, f : f32){
        self.force_repulsion = f;
    }

    pub fn set_force_attraction(&mut self, f : f32){
        self.force_attraction = f;
    }

    pub fn set_force_orientation(&mut self, f : f32){
        self.force_orientation = f;
    }

    pub fn set_facteur_chaos(&mut self, f : f32){
        self.facteur_chaos = f;
    }

    pub fn afficher(&self){
        let points = self.forme.get_points();
        draw_triangle(points.0, points.1, points.2, RED);
    }

    fn evaluation_distance(&self, copain : &Bird) -> (Zone,Vec2) {
        let mut vec_distance = self.forme.s - copain.forme.s;
        
        // Si les deux oiseaux sont parfaitement superposés, on ajoute un mini décalage
        // pour éviter qu'ils ne se séparent jamais (distance = 0)
        if vec_distance.length() == 0.0 {
            vec_distance = vec2(random_range(-0.1..0.1), random_range(-0.1..0.1));
        }
        
        let distance = vec_distance.length();

        let mut eval = (Liberte,vec_distance);

        if distance < self.zone_repulsion{
            eval.0 = Repulsion;
        }
        else if distance >= self.zone_repulsion && distance < self.zone_orientation {
            eval.0 = Orientation;
        }
        else if distance >= self.zone_orientation && distance < self.zone_attraction {
            eval.0 = Attraction;
        }
        eval
    }

    //Renvoie le vecteur accélération pour corriger la position de l'oiseau
    fn observer(&self, oiseaux : &[Bird]) -> Vec2{
        let mut vec_accel = vec2(0.0, 0.0);
        for copain in oiseaux {
                let eval = self.evaluation_distance(copain);
                
                match eval.0 {
                    //S'il est trop proche
                    Repulsion => vec_accel += eval.1.normalize_or(eval.1) / self.force_repulsion,
                    //S'il est à bonne distance
                    Orientation => vec_accel += self.force_orientation*copain.vec_vitesse,
                    //S'il est éloigné
                    Attraction => vec_accel -= self.force_attraction*eval.1,
                    //S'il est trop loin
                    Liberte => ()
                }
        }
        vec_accel
    }

    fn step(&mut self, accel_correction : Vec2){
        //On génère une acccélération
        let mut accel = vec2(0.0, 0.0);
        if self.facteur_chaos > 0.0 {
            accel = vec2(random_range(-self.facteur_chaos..self.facteur_chaos), random_range(-self.facteur_chaos..self.facteur_chaos));
        }

        //On ajoute l'accélération au vecteur vitesse
        self.vec_vitesse += accel+accel_correction;
        
        if (self.vec_vitesse).length() > self.v_max {
            self.vec_vitesse = self.vec_vitesse.normalize()*self.v_max;
        }

        self.forme.s += self.vec_vitesse;

        self.effet_pac_man();
        
    }

    fn effet_pac_man(&mut self){
        if self.forme.s.x < 0.0 {
            self.forme.s.x += SIM_WIDTH;
        } else if self.forme.s.x > SIM_WIDTH {
            self.forme.s.x -= SIM_WIDTH;
        }

        if self.forme.s.y < 0.0 {
            self.forme.s.y += SIM_HEIGHT;
        } else if self.forme.s.y > SIM_HEIGHT {
            self.forme.s.y -= SIM_HEIGHT;
        }

        if self.vec_vitesse.length() > 0.0 {
            self.forme.vec_dir_h = self.vec_vitesse.normalize();
        }
    }

}

fn fenetre_config() -> Conf {
    Conf {
        window_title: "Vol en essaim".to_owned(),
        window_width: (WINDOW_WIDTH as i32),
        window_height: (WINDOW_HEIGHT as i32),
        ..Default::default()
    }
}

#[macroquad::main(fenetre_config)]
async fn main() {

    let mut oiseaux :Vec<Bird> = Vec::new();
    let mut nb_oiseaux_global : f32 = 50.0;

    for _ in 0..(nb_oiseaux_global as i32) {
        oiseaux.push(Bird::new_random());
    }

    let mut v_max_global            : f32 = 5.0;

    let mut zone_repulsion_global   : f32 = 40.0;
    let mut zone_orientation_global : f32 = 60.0;
    let mut zone_attraction_global  : f32 = 80.0;

    let mut force_repulsion_global  : f32 = 2.0;
    let mut force_attraction_global : f32 = 0.1;
    let mut force_orientation_global: f32 = 0.1;
    let mut facteur_chaos_global    : f32 = 0.5;

    loop {
        clear_background(BLACK);

        draw_rectangle_lines(0.0, 0.0, SIM_WIDTH, SIM_HEIGHT, 2.0, GREEN);

        root_ui().window(hash!(), vec2(10.0, SIM_HEIGHT + 10.0), vec2(WINDOW_WIDTH - 20.0, WINDOW_HEIGHT - SIM_HEIGHT - 20.0), |ui| {
            ui.label(None, "Parametres de l'essaim");
            
            // Un curseur qui modifie directement notre variable
            ui.slider(hash!(), "Nb oiseaux", 1.0..300.0, &mut nb_oiseaux_global);
            ui.slider(hash!(), "Vitesse Max", 1.0..20.0, &mut v_max_global);
            
            ui.slider(hash!(), "Zone repuls.", 10.0..100.0, &mut zone_repulsion_global);
            ui.slider(hash!(), "Zone orient.", 10.0..200.0, &mut zone_orientation_global);
            ui.slider(hash!(), "Zone attract.", 10.0..300.0, &mut zone_attraction_global);

            ui.slider(hash!(), "Force repuls.", 0.1..5.0, &mut force_repulsion_global);
            ui.slider(hash!(), "Force attract.", 0.01..1.0, &mut force_attraction_global);
            ui.slider(hash!(), "Force orient.", 0.01..1.0, &mut force_orientation_global);
            ui.slider(hash!(), "Chaos", 0.0..2.0, &mut facteur_chaos_global);
        });

        // Gestion du nombre d'oiseaux
        let n_oiseaux = nb_oiseaux_global as usize;

        while oiseaux.len() < n_oiseaux {
            oiseaux.push(Bird::new_random());
        }
        while oiseaux.len() > n_oiseaux {
            oiseaux.pop();
        }

        //MAJ des variables
        for oiseau in &mut oiseaux{
            oiseau.set_v_max(v_max_global);

            oiseau.set_zone_repulsion(zone_repulsion_global);
            oiseau.set_zone_orientation(zone_orientation_global);
            oiseau.set_zone_attraction(zone_attraction_global);

            oiseau.set_force_repulsion(force_repulsion_global);
            oiseau.set_force_attraction(force_attraction_global);
            oiseau.set_force_orientation(force_orientation_global);
            oiseau.set_facteur_chaos(facteur_chaos_global);
        }

    let mut accelerations:Vec<Vec2> = Vec::new();

        //Boucle d'observation
        for oiseau in &oiseaux{
            accelerations.push(oiseau.observer(&oiseaux));
        }

        //Boucle de marche
        for (oiseau, accel) in &mut oiseaux.iter_mut().zip(accelerations.iter()) {
            oiseau.afficher();
            oiseau.step(*accel);
        }

        next_frame().await
    }
}