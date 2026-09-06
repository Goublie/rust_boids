use macroquad::prelude::*;
use ::rand::*;

const WIDTH : i32 = 500;
const HEIGHT : i32 = 500;

struct TriangleIsocele {
    s:Vec2, //Sommet principal
    m:Vec2, //milieu de la base
    largeur:f32, //logueur de la base
}

impl TriangleIsocele{
    pub fn new(s:Vec2, m:Vec2, largeur:f32) -> TriangleIsocele{
        TriangleIsocele{
            s:s,
            m:m,
            largeur:largeur
        }
    }

    pub fn get_hauteur(&self) -> Vec2 {
        vec2(self.s.x - self.m.x, self.s.y - self.m.y)
    }

    fn get_base_unitaire(&self) -> Vec2 {
        let vec_dir : Vec2 = self.get_hauteur();
        let norme : f32 = vec_dir.length();
        let vec_base = vec2(-vec_dir.y, vec_dir.x)/norme;
        vec_base
    }

    pub fn get_points(&self) -> (Vec2,Vec2,Vec2){
        let mut res = (vec2(0.0,0.0),vec2(0.0,0.0),vec2(0.0,0.0));
        res.0 = self.s;

        let vec_base_norm = self.get_base_unitaire();

        res.1 = self.m + vec_base_norm*self.largeur/2.0;
        res.2 = self.m - vec_base_norm*self.largeur/2.0;

        res
    }
}

struct Bird {
    forme:TriangleIsocele,
    vec_mouv:Vec2,
    vitesse:f32
}

impl Bird{
    pub fn new(tete:Vec2) -> Bird{
        Bird{
            forme:TriangleIsocele::new(tete, vec2(tete.x, tete.y+32.0), 16.0),
            vec_mouv:vec2(0.0,0.0),
            vitesse:1.0
        }
    }

    pub fn new_random() -> Bird{
        let position = vec2(random::<f32>()*(WIDTH as f32) ,random::<f32>()*(HEIGHT as f32));
        Bird::new(position)
    }

    pub fn afficher(&self){
        let points = self.forme.get_points();
        draw_triangle(points.0, points.1, points.2, RED);
    }
}


fn fenetre_config() -> Conf {
    Conf {
        window_title: "Vol en essain".to_owned(),
        window_width: WIDTH,
        window_height: HEIGHT,
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

        for oiseau in &oiseaux {
            oiseau.afficher();
        }

        next_frame().await
    }
}