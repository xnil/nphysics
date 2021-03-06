extern crate "nalgebra" as na;
extern crate ncollide;
extern crate nphysics;
extern crate nphysics_testbed3d;

use std::num::Float;
use na::{Pnt3, Vec3, Translation};
use ncollide::shape::{Ball, Plane};
use nphysics::world::World;
use nphysics::object::RigidBody;
use nphysics_testbed3d::Testbed;

fn main() {
    /*
     * World
     */
    let mut world = World::new();
    world.set_gravity(Vec3::new(0.0, -9.81, 0.0));

    /*
     * Planes
     */
    let normals = [
        Vec3::new(-1.0, 1.0, -1.0 ),
        Vec3::new(1.0, 1.0, -1.0 ),
        Vec3::new(-1.0, 1.0, 1.0 ),
        Vec3::new(1.0, 1.0, 1.0 )
    ];
    for n in normals.iter() {
        let rb   = RigidBody::new_static(Plane::new(*n), 0.3, 0.6);

        world.add_body(rb);
    }

    /*
     * Create the balls
     */
    let num     = 1500.0f32.powf(1.0f32 / 3.0) as uint;
    let rad     = 0.5;
    let shift   = 2.5 * rad;
    let centerx = shift * (num as f32) / 2.0;
    let centery = shift * (num as f32) / 2.0;

    for i in range(0u, num) {
        for j in range(0u, num) {
            for k in range(0u, num) {
                let x = i as f32 * 2.5 * rad - centerx;
                let y = 10.0 + j as f32 * 2.5 * rad + centery * 2.0;
                let z = k as f32 * 2.5 * rad - centerx;

                let mut rb = RigidBody::new_dynamic(Ball::new(rad), 1.0, 0.3, 0.6);

                rb.append_translation(&Vec3::new(x, y, z));

                world.add_body(rb);
            }
        }
    }

    /*
     * Set up the testbed.
     */
    let mut testbed = Testbed::new(world);

    testbed.look_at(Pnt3::new(-10.0, 50.0, -10.0), Pnt3::new(0.0, 0.0, 0.0));
    testbed.run();
}
