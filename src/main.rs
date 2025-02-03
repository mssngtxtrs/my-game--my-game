use macroquad::prelude::*;
use ::rand::{Rng, rng};
use crate::GameStates::GameOver;

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

//--IMPLEMENTATIONS--
//Player
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
        draw_circle(self.circle.x, self.circle.y, 10., WHITE);
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
    fn collision(&mut self, other: &Square){
        if self.circle.overlaps_rect(&other.rect) {
            self.death = true;
        }
    }
}
//Squares
impl Square {
    fn new(x: f32, y: f32, speed: f32, size: f32) -> Self {
        Self{
            rect: Rect::new(x, y, size, size),
            speed,
        }
    }
    fn draw(&self) {
        draw_rectangle(self.rect.x, self.rect.y, self.rect.w, self.rect.h, YELLOW);
    }
    fn movement(&mut self) {
        self.rect.y += self.speed * get_frame_time();
    }
}


//--GAME WINDOW CONFIGURATION--
fn conf() -> Conf {
    Conf {
        window_title: "моя игра моя игра".to_string(),
        window_width: 1000,
        window_height: 1000,
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
    //Creating game state variable
    let mut game_state = GameStates::Playing;

    //Creating player
    let mut player = Player::new(3);

    //Creating vector with squares
    let mut squares = vec![];

    //RNG
    let mut rng = rng();

    loop {
        //Clearing screen for next frame
        clear_background(BLACK);

        //Checking game state
        match game_state {
            GameStates::Menu => {}
            GameStates::Playing => {
                //Moving player
                player.movement(vec2(
                    if is_key_down(KeyCode::Left) { -1. } else if is_key_down(KeyCode::Right) { 1. } else { 0. },
                    if is_key_down(KeyCode::Up) { -1. } else if is_key_down(KeyCode::Down) { 1. } else { 0. }));

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

                //Moving and drawing squares
                for square in &mut squares {
                    square.movement();
                    square.draw();
                }

                //Collision and game over
                for square in &squares {
                    player.collision(square)
                }

                //Check for death
                if player.death {
                    if player.lives > 1 {
                        player = new_attempt(&player, &mut squares);
                    } else if player.lives <= 1 {
                        game_state = GameOver;
                    }
                }

                //Removing squares out of bounds
                squares.retain(|square| square.rect.y < screen_height() + square.rect.h);

                //Drawing player
                player.draw();
            }
            GameStates::GameOver => {}
            GameStates::None => {}
        }

        //Waiting for next frame
        next_frame().await;
    }
}

//New attempt function
fn new_attempt(player: &Player, squares: &mut Vec<Square>) -> Player{
    squares.clear();
    Player::new(player.lives - 1)
}