mod utils;
extern crate rand;
extern crate web_sys;
use rand::thread_rng;
use std::collections::VecDeque;
use rand::Rng;
use wasm_bindgen::prelude::*;


// When the `wee_alloc` feature is enabled, use `wee_alloc` as the global
// allocator.
#[cfg(feature = "wee_alloc")]
#[global_allocator]
static ALLOC: wee_alloc::WeeAlloc = wee_alloc::WeeAlloc::INIT;


#[wasm_bindgen]
#[derive(Clone, Copy, Debug, PartialEq, Eq)]
pub struct Point{
  x:i32,
  y:i32
}


#[wasm_bindgen]
impl Point{
  pub fn get_x(&self) -> i32{
    self.x
  }
  pub fn get_y(&self) -> i32{
    self.y
  }
}

#[wasm_bindgen]
#[derive(Clone)]
pub struct Player{
  points: VecDeque<Point>,
  direction: DIRECTION
}

#[wasm_bindgen]
impl Player{
  pub fn get_point(&self,index:usize) -> Point{
    Point{x:self.points[index].get_x(),y:self.points[index].get_y()}
  }
  pub fn get_len(&self) -> usize{
    self.points.len()
  }

}

#[wasm_bindgen]
#[derive(Clone, Copy, Debug, PartialEq, Eq)]
pub enum DIRECTION{
  UP = 38,
  DOWN = 40,
  LEFT = 37,
  RIGHT = 39,

}

#[wasm_bindgen]
pub struct Game{
  resolution: Point,
  score : i32,
  player: Player,
  food: Point,
  gameover: bool
}

#[wasm_bindgen]
impl Game{
  pub fn new(width:i32, height:i32, density:i32) -> Game {
    let mut rng = thread_rng();
    let mut p = VecDeque::new();
    p.push_back(Point{x:0,y:0});
    let player:Player = Player{points: p, direction: DIRECTION::UP};
    let food:Point = Point{x: rng.gen_range(0, (width/density)-1), y: rng.gen_range(0, (height/density))-1};
    Game{
      resolution: Point{x: width/density, y: height/density},
      score: 0, 
      player,
      food,
      gameover: false
    }
  }
    pub fn get_player(&self) -> Player {
      self.player.clone()
    }
    pub fn get_score(&self) -> i32{
      self.score
    }
    pub fn get_gameover(&self) -> bool {
      self.gameover
    }
    pub fn get_food(&self) -> Point{
      self.food
    }
    pub fn move_it(&mut self, d: DIRECTION) {
      if ((self.player.direction == DIRECTION::UP || self.player.direction == DIRECTION::DOWN) && 
        (d == DIRECTION::LEFT || d == DIRECTION::RIGHT)) ||
       ((self.player.direction == DIRECTION::LEFT || self.player.direction == DIRECTION::RIGHT) && 
        (d == DIRECTION::UP || d == DIRECTION::DOWN))
      {
        self.player.direction = d
      }
    }

    pub fn update(&mut self){
      self.food_catch();
      self.player_move();
      self.gameover = self.player_check();
    }
}

impl Game{

  fn food_catch(&mut self){
    if self.food.x == self.player.points[0].x &&
     self.food.y == self.player.points[0].y {
       self.score+=1;
       let mut rng = thread_rng();
       self.food.y = rng.gen_range(0, self.resolution.y-1);
       self.food.x = rng.gen_range(0, self.resolution.x-1);
       let (x,y) = self.apply_direction();
       let nx = (self.player.points[0].x+x)%self.resolution.x;
       let ny = (self.player.points[0].y+y)%self.resolution.y;
       let n = Point{x:nx,y:ny};
       self.player.points.push_back(n);
    }
  }

  fn player_move(&mut self){
    let (x,y) = self.apply_direction();
    let nx = (self.player.points[0].x+x)%self.resolution.x;
    let ny = (self.player.points[0].y+y)%self.resolution.y;
    self.player.points.push_front(
      Point{x:if nx <0 {self.resolution.x-1} else {nx},
            y:if ny <0 {self.resolution.y-1} else {ny}});
    self.player.points.pop_back();

  }

  fn player_check(&self) -> bool{
    let len = self.player.points.len();
    for i in 0..len{
      for j in 0..len{
        if i != j && self.player.points[i] == self.player.points[j]{
          return true
        }
      }
    }
    return false
  }

  fn apply_direction(&self) -> (i32,i32){
    match self.player.direction{
      DIRECTION::UP => (0,-1),
      DIRECTION::DOWN => (0,1),
      DIRECTION::LEFT => (-1,0),
      DIRECTION::RIGHT => (1,0)
    }
  }
}
