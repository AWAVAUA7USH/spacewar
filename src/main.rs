use macroquad::prelude::*;

const GRAVITY_STRENGTH: f32 = 300.0;

struct Spaceship {
    position: Vec2,
    velocity: Vec2,
    angular_velocity: f32,
    angle: f32,
    reverse: bool,
    thrust: bool,
    alive: bool,
}
impl Spaceship {
    fn draw(&self) {
        let color = if self.thrust { GREEN } else { WHITE };
        let color = if self.reverse { RED } else { color };
        
        let cangle = self.angle.to_degrees() + 90.0;
        let cangle = cangle.to_radians();
        
        let nose = vec2(0.0, -20.0);
        let left = vec2(-6.0, 6.0);
        let right = vec2(6.0, 6.0);
    
        let rotated_nose = vec2(
            nose.x * cangle.cos() - nose.y * cangle.sin(),
            nose.x * cangle.sin() + nose.y * cangle.cos(),
        );
        let rotated_left = vec2(
            left.x * cangle.cos() - left.y * cangle.sin(),
            left.x * cangle.sin() + left.y * cangle.cos(),
        );
        let rotated_right = vec2(
            right.x * cangle.cos() - right.y * cangle.sin(),
            right.x * cangle.sin() + right.y * cangle.cos(),
        );
    
        let nose_position = self.position + rotated_nose;
        let left_position = self.position + rotated_left;
        let right_position = self.position + rotated_right;
    
        draw_triangle(nose_position, left_position, right_position, color);
    }
    fn update(&mut self, star_position: Vec2) {
        if self.thrust {
            self.velocity += vec2(self.angle.cos(), self.angle.sin()) * 0.01;
        }
        if self.reverse {
            self.velocity -= vec2(self.angle.cos(), self.angle.sin()) * 0.01;
        }
        if(self.position.x > screen_width()) {
            self.position.x = 0.0;
        }
        if(self.position.x < 0.0) {
            self.position.x = screen_width();
        }
        if(self.position.y > screen_height()) {
            self.position.y = 0.0;
        }
        if(self.position.y < 0.0) {
            self.position.y = screen_height();
        }

        let direction = star_position - self.position;

        let distance = direction.length().max(10.0);

        let direction_unit = direction / distance;

        let force = direction_unit * (GRAVITY_STRENGTH / distance.powi(2));

        self.velocity += force;
        self.position += self.velocity;
        self.angle += self.angular_velocity;
        //self.angular_velocity *= 0.95;
    }
}

struct Projectile {
    position: Vec2,
    velocity: Vec2,
    active: bool,
}
impl Projectile {
    fn draw(&self) {
        if self.active {
            draw_circle(self.position.x, self.position.y, 2.0, RED);
        }
    }
    fn update(&mut self, star_position: Vec2) {
        if !self.active {
            return;
        }
        let direction = star_position - self.position;

        let distance = direction.length().max(10.0);

        let direction_unit = direction / distance;

        let force = direction_unit * (GRAVITY_STRENGTH / distance.powi(2));
        self.velocity += force;
        self.position += self.velocity;
        if self.position.x > screen_width() || self.position.x < 0.0 || self.position.y > screen_height() || self.position.y < 0.0 {
            self.active = false;
        }
    }
}

struct Spacewar {
    player1: Spaceship,
    player2: Spaceship,
    projectiles: Vec<Projectile>,
    star_position: Vec2,
}
impl Spacewar {
    fn new() -> Self {
        Spacewar {
            player1: Spaceship {
                position: vec2(100.0, 130.0),
                velocity: vec2(0.0, 0.0),
                angular_velocity: 0.0,
                angle: 0.0,
                thrust: false,
                reverse: false,
                alive: true,
            },
            player2: Spaceship {
                position: vec2(screen_width() - 100.0, 130.0),
                velocity: vec2(0.0, 0.0),
                angular_velocity: 0.0,
                angle: 0.0,
                thrust: false,
                reverse: false,
                alive: true,
            },
            projectiles: Vec::new(),
            star_position: vec2(screen_width()/2.0, screen_height()/2.0),
        }
    }
    fn draw(&self) {
        draw_circle(self.star_position.x, self.star_position.y, 20.0, YELLOW);
        self.player1.draw();
        self.player2.draw();
        for projectile in self.projectiles.iter() {
            projectile.draw();
        }
    }
    fn update(&mut self) {
        self.player1.update(self.star_position);
        self.player2.update(self.star_position);
        for projectile in self.projectiles.iter_mut() {
            if !projectile.active {
                continue;
            }
            projectile.update(self.star_position);
            if projectile.position.distance(self.star_position) < 25.0 {
                projectile.active = false;
            }
            if projectile.position.distance(self.player1.position) < 10.0 {
                self.player1.alive = false;
            }
            if projectile.position.distance(self.player2.position) < 10.0 {
                self.player2.alive = false;
            }
            if !projectile.active {
                continue;
            }
        }
        if self.player1.position.distance(self.star_position) < 25.0 {
            self.player1.alive = false;
        }
        if self.player2.position.distance(self.star_position) < 25.0 {
            self.player1.alive = false;
        }
    }
}

#[macroquad::main("Spacewar")]
async fn main() {
    let mut started: bool = false;
    let star_position = vec2(screen_width()/2.0, screen_height()/2.0);
    let gravity_strength = 0.1;
    let mut game = Spacewar::new();
    
    loop {
        clear_background(WHITE);
        draw_text("Press SPACE to host a server", 20.0, 20.0, 30.0, BLACK);
        game.player1.position = vec2(100.0, 130.0);
        game.player1.velocity = vec2(0.0, 0.0);
        game.player1.angular_velocity = 0.0;
        game.player1.angle = 0.0;
        game.player1.thrust = false;
        game.player1.reverse = false;
        game.player1.alive = true;
        game.projectiles = Vec::new();
        game.player2.position = vec2(screen_width() - 100.0, 130.0);
        game.player2.velocity = vec2(0.0, 0.0);
        game.player2.angular_velocity = 0.0;
        game.player2.angle = 0.0;
        game.player2.thrust = false;
        game.player2.reverse = false;
        game.player2.alive = true;
        while started{
            if !game.player1.alive {
                started = false;
            }
            if !game.player2.alive {
                started = false;
            }
            clear_background(BLACK);
            game.draw();
            game.update();
            if is_key_down(KeyCode::W) {
                game.player1.thrust = true;
            } else {
                game.player1.thrust = false;
            }
            if is_key_down(KeyCode::S) {
                game.player1.reverse = true;
            } else {
                game.player1.reverse = false;
            }
            if is_key_down(KeyCode::A) {
                game.player1.angular_velocity -= 0.004;
            }
            if is_key_down(KeyCode::D) {
                game.player1.angular_velocity += 0.004;
            }
            if is_key_down(KeyCode::Space) {
                game.projectiles.push(
                    Projectile {
                    position: game.player1.position + vec2(game.player1.angle.cos(), game.player1.angle.sin()) * 10.0,
                    velocity: game.player1.velocity + vec2(game.player1.angle.cos(), game.player1.angle.sin()) * 4.0,
                    active: true,
                }
                )
            }
            if is_key_down(KeyCode::I) {
                game.player2.thrust = true;
            }
            else {
                game.player2.thrust = false;
            }
            if is_key_down(KeyCode::K) {
                game.player2.reverse = true;
            }
            else {
                game.player2.reverse = false;
            }
            if is_key_down(KeyCode::J) {
                game.player2.angular_velocity -= 0.004;
            }
            if is_key_down(KeyCode::L) {
                game.player2.angular_velocity += 0.004;
            }
            if is_key_pressed(KeyCode::Enter) {
                game.projectiles.push(
                    Projectile {
                    position: game.player2.position + vec2(game.player2.angle.cos(), game.player2.angle.sin()) * 10.0,
                    velocity: game.player2.velocity + vec2(game.player2.angle.cos(), game.player2.angle.sin()) * 4.0,
                    active: true,
                }
                )
            }
            next_frame().await;
        }

        next_frame().await;
        if is_key_pressed(KeyCode::H) {
            started = true;
        }
    }
}