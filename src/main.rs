use macroquad::prelude::*;

const GRAVITATIONAL_CONSTANT: f32 = 6.67430e-11;

trait Entity {
  fn gravitate(&mut self, star_position: Vec2);
  fn update(&mut self);
  fn draw(&self);
}
#[derive(Clone)]
struct Star {
  position: Vec2,
  velocity: Vec2,
  mass: f32,
}
impl Star {
  fn draw(&self) {
    draw_circle(self.position.x,self.position.y, 10.0, YELLOW);
  }
}
#[derive(Clone)]
struct Spaceship {
  mass: f32,
  position: Vec2,
  angle: f32,
  nose: Vec2,
  right_wing: Vec2,
  left_wing: Vec2,
  velocity: Vec2,
  angular_velocity: f32,
}
impl Spaceship {
  fn gravitate(&mut self, star: Star, players: Vec<Spaceship>, bullets: Vec<Bullet>) {
    let distance = self.position.distance(star.position);
    let force = GRAVITATIONAL_CONSTANT*(self.mass * star.mass)/(distance*distance);
    let direction = (star.position - self.position).normalize();
    self.velocity += direction * force;

    for player in players.iter() {
      let distance = self.position.distance(player.position);
      let force = GRAVITATIONAL_CONSTANT*(self.mass * player.mass)/(distance*distance);
      let direction = (player.position - self.position).normalize();
      self.velocity += direction * force/self.mass;
    }
    for bullet in bullets.iter() {
      let distance = self.position.distance(bullet.position);
      let force = GRAVITATIONAL_CONSTANT*(self.mass * bullet.mass)/(distance*distance);
      let direction = (bullet.position - self.position).normalize();
      self.velocity += direction * force/self.mass;
    }
  }
  fn update(&mut self) {
    self.position += self.velocity;
    self.angular_velocity += self.velocity.y;
    //self.velocity *= 0.99;
    //self.angular_velocity *= 0.99;
    self.nose = self.position + vec2(
      -10.0*self.angle.cos(), 10.0*self.angle.sin()
    );
    self.right_wing = self.position + vec2(
      5.0*self.angle.cos() - 5.0*self.angle.sin(), 5.0*self.angle.sin() + 5.0*self.angle.cos()
    );
    self.left_wing = self.position + vec2(
      -5.0*self.angle.cos() - 5.0*self.angle.sin(), -5.0*self.angle.sin()+5.0*self.angle.cos()
    );
  }
  fn draw(&self) {
    draw_triangle(self.nose, self.right_wing, self.left_wing, WHITE);
    draw_circle(self.position.x,self.position.y, 10.0, WHITE);
  }
  fn shoot(&mut self, mut bullets: Vec<Bullet>) {
    bullets.push(Bullet {
      mass: 0.001,
      position: self.nose,
      velocity: self.velocity + vec2(
        10.0 * self.angle.cos(),
        10.0 * self.angle.sin()
      )
    });
  }
}
#[derive(Clone)]
struct Bullet {
  mass: f32,
  position: Vec2,
  velocity: Vec2,
}
impl Bullet {
    fn gravitate(&mut self, star: Star, players: Vec<Spaceship>, bullets: Vec<Bullet>) {
      let distance = self.position.distance(star.position);
      let force = GRAVITATIONAL_CONSTANT*(self.mass * star.mass)/(distance*distance);
      let direction = (star.position - self.position).normalize();
      self.velocity += direction * force;

      for player in players.iter() {
        let distance = self.position.distance(player.position);
        let force = GRAVITATIONAL_CONSTANT*(self.mass * player.mass)/(distance*distance);
        let direction = (player.position - self.position).normalize();
        self.velocity += direction * force/self.mass;
      }
      for bullet in bullets.iter() {
        let distance = self.position.distance(bullet.position);
        let force = GRAVITATIONAL_CONSTANT*(self.mass * bullet.mass)/(distance*distance);
        let direction = (bullet.position - self.position).normalize();
        self.velocity += direction * force/self.mass;
      }
    }
    fn update(&mut self) {
      self.position += self.velocity;
      if self.position.x <= 0.0 {
        self.position.x = screen_width();
      }else if self.position.x >= screen_width() {
        self.position.x = 0.0;
      }else if self.position.y <= 0.0 {
        self.position.y = screen_height();
      }else if self.position.y >= screen_height() {
        self.position.y = 0.0;
      }
    }
    fn draw(&self) {
      draw_circle(self.position.x, self.position.y, 5.0, RED);
    }
}

struct Spacewar {
  players: Vec<Spaceship>,
  star: Star,
  bullets: Vec<Bullet>,
}
impl Spacewar {
  fn update(&mut self) {
    let players_compare = self.players.clone();
    let bullets_compare = self.bullets.clone();
    let star_compare = self.star.clone();
    
    for player in self.players.iter_mut() {
      player.gravitate(star_compare.clone(), players_compare.clone(), bullets_compare.clone());
      player.update();
    }
    for bullet in self.bullets.iter_mut() {
      bullet.gravitate(star_compare.clone(), players_compare.clone(), bullets_compare.clone());
      bullet.update();
    }
  }
  fn draw(&self) {
    for player in self.players.iter() {
      player.draw();
    }
    for bullet in self.bullets.iter() {
      bullet.draw();
    }
    self.star.draw();
  }
  fn new() -> Spacewar {
    Spacewar {
      players: Vec::<Spaceship>::new(),
      bullets: Vec::<Bullet>::new(),
      star: Star {
        position: vec2(screen_width()*0.5, screen_height()*0.5),
        velocity: vec2(0.0, 0.0),
        mass: 1.0,
      },
    }
  }
}

#[macroquad::main("Spacewar")]
async fn main() {
  let mut started = true;
  let mut spacewar = Spacewar::new();
  spacewar.players.push(
    Spaceship{
      mass: 0.2,
      angle: 0.0,
      position: vec2(20.0, 20.0),
      nose: vec2(0.0, 0.0),
      right_wing: vec2(0.0, 0.0),
      left_wing: vec2(0.0, 0.0),
      velocity: vec2(0.0, 0.0),
      angular_velocity: 0.0,
    }
  );
  spacewar.players[1].update();
  spacewar.star = Star {
    position: vec2(screen_width()*0.5, screen_height()*0.5),
    velocity: vec2(0.0, 0.0),
    mass: 10.0,
  };
  while started {
    loop {
      clear_background(BLACK);
      spacewar.update();
      spacewar.draw();
      next_frame().await;
    }
    next_frame().await;
  }
}