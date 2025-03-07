use macroquad::prelude::*;

const GRAVITATIONAL_CONSTANT: f32 = 6.67430e-11;

trait Entiy {
  fn apply_gravity(&mut self, star_position: Vec2);
  fn update(&mut self);
  fn draw(&self);
}
struct Spaceship {
  mass: f32,
  position: Vec2,
  nose: Vec2,
  right_wing: Vec2,
  left_wing: Vec2,
  velocity: Vec2,
  angular_velocity: f32,
}
impl Entity for Spaceship {
  fn gravitate(&mut self, star: Star, players: Vec<Spaceship>, bullets: Vec<Bullet>) {
    let distance = self.position.distance(star.position);
    let force = GRAVITATIONAL_CONSTANT*(self.mass * star.mass)/(distance*distance);
    let direction = (star.position - self.position).normalize();
    self.velocity += direction * force;
    for i in players.iter() {
      let distance = self.position.distance(i.position);
      let force = GRAVITATIONAL_CONSTANT*(self.mass * i.mass)/(distance*distance);
      let direction = (i.position - self.position).normalize();
      self.velocity += direction * force/self.mass;
    }
    for i in bullets.iter() {
      let distance = self.position.distance(i.position);
      let force = GRAVITATIONAL_CONSTANT*(self.mass * i.mass)/(distance*distance);
      let direction = (i.position - self.position).normalize();
      self.velocity += direction * force/self.mass;
    }
  }
  fn update(&mut self) {
    self.position += self.velocity;
    self.angular_velocity += self.velocity.y;
    //self.velocity *= 0.99;
    //self.angular_velocity *= 0.99;
    self.nose = self.position + vec2(
      -10*self.angle.cos(), 10*self.angle.sin()
    );
    self.right_wing = self.position + vec2(
      5*self.angle.cos() - 5*self.angle.sin(), 5*self.angle.sin() + 5*self.angle.cos()
    );
    self.left_wing = self.position + vec2(
      -5*self.angle.cos() - 5*self.angle.sin(), -5*self.angle.sin()+5*self.angle.cos()
    );
  }
  fn draw(&self) {
    draw_triangle(self.nose, self.right_wing, self.left_wing, WHITE);
  }
}
struct Bullet {
  mass: f32,
  position: Vec2,
  velocity: f32
}
impl Entity for Bullet {
  impl Entity for Spaceship {
    fn gravitate(&mut self, star: Star, players: Vec<Spaceship>, bullets: Vec<Bullet>) {
      let distance = self.position.distance(star.position);
      let force = GRAVITATIONAL_CONSTANT*(self.mass * star.mass)/(distance*distance);
      let direction = (star.position - self.position).normalize();
      self.velocity += direction * force;
      for i in players.iter() {
        let distance = self.position.distance(i.position);
        let force = GRAVITATIONAL_CONSTANT*(self.mass * i.mass)/(distance*distance);
        let direction = (i.position - self.position).normalize();
        self.velocity += direction * force/self.mass;
      }
      for i in bullets.iter() {
        let distance = self.position.distance(i.position);
        let force = GRAVITATIONAL_CONSTANT*(self.mass * i.mass)/(distance*distance);
        let direction = (i.position - self.position).normalize();
        self.velocity += direction * force/self.mass;
      }
    }
    fn update(&mut self) {
      self.position += self.velocity;
      if self.position.x <= 0 {
        self.position.x = scren_width();
      }if else self.position.x >= scren_width() {
        self.position.x = 0;
      }if else self.position.y <= 0 {
        self.position.y = scren_height();
      }if else self.position.y >= scren_height() {
        self.position.y = 0;
      }
    }
    fn draw(&self) {
      draw_circle(self.position, 5, RED);
    }
}
#[macroquad::main("Spacewar")]
async fn main() {
  
}