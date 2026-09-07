use macroquad::prelude::*;
use ::rand::*;

const WIDTH  : f32 = 500.0;
const HEIGHT : f32 = 500.0;

struct TriangleIsocele {
    s         :Vec2, //Sommet principal
    hauteur   :f32, //hauteur du triangle
    vec_dir_h : Vec2,  //vecteur directeur hauteur du triangle
    largeur   :f32, //logueur de la base
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
    forme       : TriangleIsocele,
    vec_vitesse : Vec2,
    v_max       : f32
}

impl Bird{
    pub fn new(tete:Vec2) -> Bird{
        let forme = TriangleIsocele::new(tete, 32.0, 16.0);
        Bird{
            forme       : forme,
            vec_vitesse : vec2(0.0,0.0),
            v_max       : 5.0
        }
    }

    pub fn new_random() -> Bird{
        let position = vec2(random::<f32>()*WIDTH ,random::<f32>()*HEIGHT);
        Bird::new(position)
    }

    pub fn afficher(&self){
        let points = self.forme.get_points();
        draw_triangle(points.0, points.1, points.2, RED);
    }


    fn dans_l_ecran(&self) -> bool{
        0.0 < self.forme.s.x && 
        self.forme.s.x < WIDTH && 
        0.0 < self.forme.s.y && 
        self.forme.s.y < HEIGHT
    }

    pub fn avancer(&mut self){
        //On génère une acccélération
        let accel = vec2(random_range(-1.0..1.0),random_range(-1.0..1.0));

        //On ajoute l'accélération au vecteur vitesse
        if (self.vec_vitesse + accel).length() < self.v_max {
            self.vec_vitesse += accel;
        }

        if !self.dans_l_ecran(){
            self.forme.vec_dir_h *= -1.0;
            self.vec_vitesse *= -1.0;
        }

        self.forme.s += self.vec_vitesse;
        self.forme.vec_dir_h = self.vec_vitesse.normalize();
            
    }
}


fn fenetre_config() -> Conf {
    Conf {
        window_title: "Vol en essain".to_owned(),
        window_width: (WIDTH as i32),
        window_height: (HEIGHT as i32),
        ..Default::default()
    }
}

#[macroquad::main(fenetre_config)]
async fn main() {

    let mut oiseaux :Vec<Bird> = Vec::new();

    for _ in 0..random::<u32>()%10 {
        oiseaux.push(Bird::new_random());
    }

    loop {
        clear_background(BLACK);

        for oiseau in &mut oiseaux {
            oiseau.afficher();
            oiseau.avancer();
        }

        next_frame().await
    }
}