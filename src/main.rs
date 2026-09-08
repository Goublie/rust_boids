use ::macroquad::prelude::*;
use Zone::*;
use ::rand::*;

const WIDTH  : f32 = 500.0;
const HEIGHT : f32 = 500.0;

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
}

impl Bird{
    pub fn new(tete:Vec2) -> Bird{
        Bird{
            forme               : TriangleIsocele::new(tete, 32.0, 16.0),
            vec_vitesse         : vec2(0.0,0.0),
            v_max               : 5.0,
            zone_repulsion      : 40.0,
            zone_orientation    : 60.0,
            zone_attraction     : 80.0
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

    fn evaluation_distance(&self, copain : &Bird) -> (Zone,Vec2) {
        let vec_distance = self.forme.s - copain.forme.s;
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
                    Repulsion => vec_accel += 0.1*(eval.1),
                    //S'il est à bonne distance
                    Orientation => vec_accel += 0.1*copain.vec_vitesse,
                    //S'il est éloigné
                    Attraction => vec_accel -= 0.1*eval.1,
                    //S'il est trop loin
                    Liberte => ()
                }
        }
        vec_accel
    }

    fn step(&mut self, accel_correction : Vec2){
        //On génère une acccélération
        let accel = vec2(random_range(-0.5..0.5),random_range(-0.5..0.5));

        //On ajoute l'accélération au vecteur vitesse
        self.vec_vitesse += accel+accel_correction;
        
        if (self.vec_vitesse).length() > self.v_max {
            self.vec_vitesse = self.vec_vitesse.normalize()*self.v_max;
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