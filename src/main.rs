use std::process::exit;
use macroquad::prelude::*;
use ::rand::{Rng, rng};
use macroquad::audio::{load_sound, play_sound_once};


//----TRAITS----
//--Collision--
trait Collide {
    fn collider_rect(&self) -> Rect;
}


//----CONSTANTS----
//--Colors--
const MAIN_COLOR: Color = WHITE;
const BACKGROUND_COLOR: Color = BLACK;
const ACCENT_COLOR: Color = RED;


//----ENUMERATIONS----
enum GameStates {
    Menu,
    Playing,
    GameOver,
    None
}


//----STRUCTURES----
//Player
#[derive(Copy, Clone)]
struct Player {
    //Movement
    circle: Circle,
    speed: f32,

    //Game design
    lives: i8,

    //Death state
    death: bool
}
//Squares
struct Square {
    rect: Rect,
    speed: f32
}
//Bullet
struct Bullet {
    rect: Rect,
    speed: f32,
}

//--IMPLEMENTATIONS--
//Player
impl Collide for &Player{
    fn collider_rect(&self) -> Rect {
        Rect::new(
            self.circle.x - self.circle.r,
            self.circle.y - self.circle.r,
            self.circle.x + self.circle.r,
            self.circle.y + self.circle.r,)
    }
}
impl Player{
    //Spawn player
    fn new(lives: i8) -> Self {
        Self {
            circle: Circle::new(screen_width() / 2., screen_height() - 100., 10.),
            speed: 320.,
            lives,
            death: false
        }
    }

    //Draw player
    fn draw(&self) {
        draw_circle(self.circle.x, self.circle.y, 10., MAIN_COLOR);
    }

    //Move player
    fn movement(&mut self, vector: Vec2) {
        if vector != vec2(0., 0.) {
            //Moving on X
            self.circle.x = (self.circle.x + self.speed * vector.normalize().x * get_frame_time())  //Add delta speed
                .clamp(self.circle.r, screen_width() - self.circle.r);  //Clamp to keep player inbounds

            //Moving on Y
            self.circle.y = (self.circle.y + self.speed * vector.normalize().y * get_frame_time())  //Add delta speed
                .clamp(self.circle.r, screen_width() - self.circle.r);  //Clamp to keep player inbounds
        }
    }

    //Collision
    fn collision<T: Collide>(&mut self, other: T)
    {
        match other {
            Square => {
                if self.circle.overlaps_rect(&Square.collider_rect()) {
                    self.death = true;
                }
            }
            _ => {}
        }
    }
}
//Squares
impl Collide for &Square{
    fn collider_rect(&self) -> Rect {
        self.rect
    }
}
impl Square {
    //New square
    fn new(x: f32, y: f32, speed: f32, size: f32) -> Self {
        Self{
            rect: Rect::new(x, y, size, size),
            speed,
        }
    }

    //Draw square
    fn draw(&self) {
        draw_rectangle(self.rect.x, self.rect.y, self.rect.w, self.rect.h, ACCENT_COLOR);
    }

    //Move square
    fn movement(&mut self) {
        self.rect.y += self.speed * get_frame_time();
    }
}
//Bullet
impl Collide for Bullet {
    fn collider_rect(&self) -> Rect {
        self.rect
    }
}
impl Bullet {
    //New bullet
    fn new(x: f32, y: f32) -> Self {
        Self{
            rect: Rect::new(x - 4., y - 6., 8., 12.),
            speed: 1280.
        }
    }
    //Draw bullet
    fn draw(&self){
        draw_rectangle(self.rect.x, self.rect.y, self.rect.w, self.rect.h, ACCENT_COLOR);
    }
    //Move bullet
    fn movement(&mut self) {
        self.rect.y -= self.speed * get_frame_time();
    }
}


//--GAME WINDOW CONFIGURATION--
fn conf() -> Conf {
    Conf {
        window_title: "моя игра моя игра".to_string(),
        window_width: 1000,
        window_height: 700,
        window_resizable: false,
        high_dpi: false,
        fullscreen: false,
        sample_count: 1,
        ..Default::default()
    }
}

//----MAIN FUNCTION----
#[macroquad::main(conf)]
async fn main() {
    //--RESOURCES--
    //Setting resources folder
    set_pc_assets_folder("resources");

    //Loading font
    let font = load_ttf_font("font.ttf").await.expect("Failed loading font");

    //Loading sounds
    let death = load_sound("player_death.ogg").await.expect("Failed loading player death sound");
    let kill = load_sound("enemy_death.ogg").await.expect("Failed loading kill sound");
    let shot = load_sound("shot.ogg").await.expect("Failed loading shot sound");


    //--GAMEPLAY VARIABLES--
    //Creating game state variable
    let mut game_state = GameStates::Menu;

    //Creating player
    let mut player = Player::new(3);

    //Creating vectors with squares and bullets
    let mut squares = vec![];
    let mut bullets = vec![];

    //RNG
    let mut rng = rng();


    //--MAIN LOOP--
    loop {
        //Clearing screen for next frame
        clear_background(BACKGROUND_COLOR);

        //Checking game state
        match game_state {
            GameStates::Menu => {
                //Text draw
                draw_text_ex("МОЯ ИГРА МОЯ ИГРА", screen_width() / 2. - 470., screen_height() / 2. - 20.,
                             TextParams{
                                 font: Some(&font),
                                 font_size: 100,
                                 color: ACCENT_COLOR,
                                 ..Default::default()
                             });
                draw_text_ex("Press SPACE to retry", screen_width() / 2. - 150., screen_height() / 2. + 60.,
                             TextParams{
                                 font: Some(&font),
                                 font_size: 32,
                                 color: MAIN_COLOR,
                                 ..Default::default()
                             });
                draw_text_ex("Press ESC to exit", screen_width() / 2. - 125., screen_height() / 2. + 120.,
                             TextParams{
                                 font: Some(&font),
                                 font_size: 32,
                                 color: MAIN_COLOR,
                                 ..Default::default()
                             });

                //Waiting for input
                if is_key_down(KeyCode::Space) {
                    player = new_attempt(3, &mut squares, &mut bullets);
                    game_state = GameStates::Playing;
                }
                if is_key_down(KeyCode::Escape) {
                    exit(1);
                }
            }
            GameStates::Playing => {
                //Moving player
                player.movement(vec2(
                    if is_key_down(KeyCode::Left) { -1. } else if is_key_down(KeyCode::Right) { 1. } else { 0. },
                    if is_key_down(KeyCode::Up) { -1. } else if is_key_down(KeyCode::Down) { 1. } else { 0. }));

                //Shooting
                if is_key_down(KeyCode::Enter){
                    bullets.push(Bullet::new(player.circle.x, player.circle.y))
                }

                //Creating squares with random chance
                if rng.random_bool(0.1) {
                    let size = rng.random_range(16. .. 64.);
                    squares.push(Square::new(
                        rng.random_range(size .. screen_width() - size),
                        -10.,
                        rng.random_range(50. .. 150.),
                        size
                        )
                    )
                };

                //Moving and drawing bullets, colliding with enemies
                for bullet in &mut bullets {
                    bullet.movement();
                    bullet.draw();
                    /*for square_collided in &mut squares {
                        bullet.collide
                    }*/
                }

                //Moving and drawing squares
                for square in &mut squares {
                    square.movement();
                    square.draw();
                }

                //Colliding with player and game over
                for square in &squares {
                    player.collision(square);
                }

                //Check for death
                if player.death {
                    play_sound_once(&death);
                    if player.lives > 0 {
                        player = new_attempt(player.lives - 1, &mut squares, &mut bullets);
                    } else if player.lives <= 0 {
                        game_state = GameStates::GameOver;
                    }
                }

                //Removing squares and bullets out of bounds
                squares.retain(|square| square.rect.y < screen_height() + square.rect.h);
                bullets.retain(|bullet| bullet.rect.y > -bullet.rect.h);

                //Drawing player
                player.draw();
            }
            GameStates::GameOver => {
                //Text draw
                draw_text_ex("GAME OVER!", screen_width() / 2. - 320., screen_height() / 2. - 20.,
                             TextParams{
                                 font: Some(&font),
                                 font_size: 120,
                                 color: ACCENT_COLOR,
                                 ..Default::default()
                             });
                draw_text_ex("Press SPACE to retry", screen_width() / 2. - 150., screen_height() / 2. + 60.,
                             TextParams{
                                 font: Some(&font),
                                 font_size: 32,
                                 color: MAIN_COLOR,
                                 ..Default::default()
                             });
                draw_text_ex("Press ESC to exit", screen_width() / 2. - 125., screen_height() / 2. + 120.,
                             TextParams{
                                 font: Some(&font),
                                 font_size: 32,
                                 color: MAIN_COLOR,
                                 ..Default::default()
                             });

                //Waiting for input
                if is_key_down(KeyCode::Space) {
                    player = new_attempt(3, &mut squares, &mut bullets);
                    game_state = GameStates::Playing;
                }
                if is_key_down(KeyCode::Escape) {
                    exit(1);
                }
            }
            GameStates::None => {}
        }

        //Waiting for next frame
        next_frame().await;
    }
}

//New attempt function
fn new_attempt(lives: i8, squares: &mut Vec<Square>, bullets: &mut Vec<Bullet>) -> Player{
    squares.clear();
    bullets.clear();
    Player::new(lives)
}