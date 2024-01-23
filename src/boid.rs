use std::cmp::min;
use std::f32::consts::PI;
use std::iter::repeat_with;

use bytemuck::Zeroable;
use rand::Rng;

use crate::vector::Vector;
use crate::vertex::Vertex;

pub const MAX_BOIDS: u32 = 150;
const BOID_DENSITY: f64 = 0.00015;

const RADIUS: f32 = 80.0;
const SEPARATION_FACTOR: f32 = 30.0;
const ALIGNMENT_FACTOR: f32 = 0.02;
const COHESION_FACTOR: f32 = 0.0035;
const MAX_SPEED: f32 = 4.0;
const WALL_RADIUS: f32 = 100.0;
const WALL_FACTOR: f32 = 100.0;

#[derive(Clone, Copy, Debug)]
pub struct Boid {
    position: Vector,
    velocity: Vector,
}

impl Boid {
    fn zero() -> Self {
        Boid {
            position: Vector::zero(),
            velocity: Vector::zero(),
        }
    }

    fn forces(self, others: impl Iterator<Item = Boid>) -> Option<(Vector, Vector, Vector)> {
        let mut neighbors = 0;
        let mut separation_force = Vector::zero();
        let mut total_velocity = Vector::zero();
        let mut total_position = Vector::zero();

        for other in others {
            let delta = self.position - other.position;
            let distance = delta.magnitude();
            if distance < RADIUS {
                separation_force += delta / (distance * distance * distance);
                total_velocity += other.velocity;
                total_position += other.position;
                neighbors += 1;
            }
        }

        if neighbors > 0 {
            let average_velocity = total_velocity / neighbors as f32;
            let average_position = total_position / neighbors as f32;
            let cohesion_force = average_position - self.position;
            Some((separation_force, average_velocity, cohesion_force))
        } else {
            None
        }
    }

    fn step(mut self, lhs: &[Boid], rhs: &[Boid], width: f32, height: f32) -> Self {
        let forces = self.forces(lhs.iter().chain(rhs).copied());
        if let Some((separation, alignment, cohesion)) = forces {
            self.velocity += SEPARATION_FACTOR * separation;
            self.velocity += ALIGNMENT_FACTOR * alignment;
            self.velocity += COHESION_FACTOR * cohesion;
        }

        let speed = self.velocity.magnitude();
        if speed > MAX_SPEED {
            self.velocity *= MAX_SPEED / speed;
        }

        self.add_wall_force(width, height);

        self.position += self.velocity;
        self
    }

    fn add_wall_force(&mut self, width: f32, height: f32) {
        if self.position.x < WALL_RADIUS {
            let distance = clamp(self.position.x);
            self.velocity.x += WALL_FACTOR / (distance * distance);
        } else if self.position.x > width - WALL_RADIUS {
            let distance = clamp(width - self.position.x);
            self.velocity.x -= WALL_FACTOR / (distance * distance);
        }

        if self.position.y < WALL_RADIUS {
            let distance = clamp(self.position.y);
            self.velocity.y += WALL_FACTOR / (distance * distance);
        } else if self.position.y > height - WALL_RADIUS {
            let distance = clamp(height - self.position.y);
            self.velocity.y -= WALL_FACTOR / (distance * distance);
        }
    }
}

fn clamp(x: f32) -> f32 {
    if x < 1.0 {
        1.0
    } else {
        x
    }
}

pub struct Flock {
    boids: Vec<Boid>,
    scratch: Vec<Boid>,
    vertices: Vec<Vertex>,
}

const FRONT_OFFSET: f32 = 10.0;
const SIDE_OFFSET: f32 = 5.0;
const BACK_OFFSET: f32 = 2.0;
const MIN_START_VELOCITY: f32 = 1.0;
const MAX_START_VELOCITY: f32 = 2.0;

impl Flock {
    pub fn new(width: u64, height: u64) -> Self {
        let mut rng = rand::thread_rng();
        Flock {
            boids: repeat_with(|| {
                let mag = rng.gen_range(MIN_START_VELOCITY..MAX_START_VELOCITY);
                let theta = rng.gen_range(0.0..2.0 * PI);
                Boid {
                    position: Vector::new(
                        rng.gen_range(0..width) as f32,
                        rng.gen_range(0..height) as f32,
                    ),
                    velocity: mag * Vector::from_angle(theta),
                }
            })
            .take(MAX_BOIDS as usize)
            .collect(),
            scratch: vec![Boid::zero(); MAX_BOIDS as usize],
            vertices: vec![Vertex::zeroed(); 3 * MAX_BOIDS as usize],
        }
    }

    pub fn step(&mut self, width: u64, height: u64) -> u32 {
        let num_boids = boid_count(width, height);
        for i in 0..num_boids as usize {
            let (lhs, rest) = self.boids[..num_boids as usize].split_at(i);
            let (cur, rhs) = rest.split_first().unwrap();
            self.scratch[i] = cur.step(lhs, rhs, width as f32, height as f32);
        }
        std::mem::swap(&mut self.boids, &mut self.scratch);
        self.update_vertices(width, height, num_boids);
        num_boids
    }

    fn update_vertices(&mut self, width: u64, height: u64, num_boids: u32) {
        for (i, boid) in self.boids.iter().take(num_boids as usize).enumerate() {
            let heading = boid.velocity.normalize();
            let orthogonal = heading.orthogonal();

            let front = boid.position + FRONT_OFFSET * heading;
            let left = boid.position - BACK_OFFSET * heading + SIDE_OFFSET * orthogonal;
            let right = boid.position - BACK_OFFSET * heading - SIDE_OFFSET * orthogonal;

            self.vertices[3 * i] = front.map(width as f32, height as f32);
            self.vertices[3 * i + 1] = right.map(width as f32, height as f32);
            self.vertices[3 * i + 2] = left.map(width as f32, height as f32);
        }
    }

    pub fn vertices(&self, num_boids: u32) -> &[Vertex] {
        &self.vertices[..3 * num_boids as usize]
    }
}

fn boid_count(width: u64, height: u64) -> u32 {
    let area = width * height;
    let count = BOID_DENSITY * area as f64;
    min(MAX_BOIDS, count as u32)
}
