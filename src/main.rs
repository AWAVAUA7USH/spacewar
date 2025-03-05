use macroquad::prelude::*;
struct Spaceship {
    position: Vec2,
    velocity: Vec2,
    angle: f32,
    angular_velocity: f32,d
    active: bool,
}
impl Spaceship {
    fn draw(&self){
        nose = Vec2((0*cos(self.angle)+10*sin(self.angle)),(o*sin(self.angle)+10*cos(self.angle)));
        right_wing = Vec2((5*cos(self.angle)+5*sin(self.angle)),(5*sin(self.angle)+5*cos(self.angle)));
        left_wing = Vec2((-5*cos(self.angle)+-5*sin(self.angle)),(-5*sin(self.angle)+-5*cos(self.angle)));
        draw_triangle( self.position + nose, self.position + right_wing, self.position + left_wing, WHITE);
    }
    fn accel(&mut self) {
        self.velocity += Vec2(0.0, 0.1);
    }
    fn rotate_left(&mut self) {
        self.angle -= 0.1;
    }
    fn rotate_right(&mut self) {
        self.angle += 0.1;
    }
    fn fire(%mut self, bullets: &mut Vec<Bullet>) {
        bullets.push(Bullet {
            position: self.position + Vec2((0*cos(self.angle)+-11*sin(self.angle)),(0*sin(self.angle)+-11*cos(self.angle))),
            velocity: self.velocity *= 2,
        });
    }
}
struct Bullet {
    position: Vec2,
    velocity: Vec2,
}
impl Bullet {
    fn draw(&self){
        draw_circle(self.position, 5, WHITE);
    }
}

struct Spacewar{
    player: Vec<Spaceship>,
    bullets: Vec<Bullet>,
}
impl Spacewar {
    fn update() {
        for bullet in self.bullets.iter_mut() {
            bullet.position += bullet.velocity;
        }
        for spaceship in self.player.iter_mut() {
            if spaceship.thrust {
                spaceship.velocity += Vec2(0, -0.1);
            }
            if spaceship.reverse {
                spaceship.velocity += Vec2(0, 0.1);
            }
            spaceship.position += spaceship.velocity;
            spaceship.angle += spaceship.angular_velocity;
            if spaceship.position.x < 0 {
                spaceship.position.x = screen_width();
            }
            if spaceship.position.x > screen_width() {
                spaceship.position.x = 0;
            }
            if spaceship.position.y < 0 {
                spaceship.position.y = screen_height();
            }
            if spaceship.position.y > screen_height() {
                spaceship.position.y = 0;
            }
            for bullet in self.bullets.iter_mut() {
                if point_in_triangle(bullet.position, spaceship.position + nose, spaceship.position + right_wing, spaceship.position + left_wing) {
                    self.bullets.remove(bullet);
                    spaceship.active = false
                }
            }
        }
    }
    fn draw() {
        for spaceship in self.player.iter() {
            spaceship.draw();
        }
        for bullet in self.bullets.iter() {
            bullet.draw();
        }
    }
}

#[macroquad::main("Macroquad Blank Page")]
async fn main() {
    game = Spacewar {}
    selection = 0;
    started = false;
    loop {
        clear_background(WHITE);
        if selection == 0 {
            draw_text("START", 200, 200, 20, RED);

            if key_is_pressed(KeyCode::Enter) {
                started = true;
            }
        }else if selection == 1 {
            draw_text("START", 200, 200, 20, WHITE);
        }
        if started {
            game.players.add(Spaceship);
        }
        while started {
            clear_background(BLACK);
            if key_is_pressed(KeyCode::W) {
                game.player[0].accel();
            }
            if key_is_pressed(KeyCode::S) {
                game.player[0].reverse();
            }
            if key_is_pressed(KeyCode::A) {
                game.player[0].rotate_left();
            }
            if key_is_pressed(KeyCode::D) {
                game.player[0].rotate_right();
            }
            if key_is_pressed(KeyCode::Space) {
                game.player[0].fire();
            }
            game.update()
        }
        next_frame().await;
    }
}