extern crate "nalgebra" as na;
extern crate ncollide;
extern crate nphysics;
extern crate nphysics_testbed3d;

use std::sync::Arc;
use na::{Pnt3, Vec3, Iso3, Translation};
use ncollide::shape::{Plane, Cuboid, Compound, CompoundData, Shape3};
use ncollide::volumetric::Volumetric;
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
    let rb = RigidBody::new_static(Plane::new(Vec3::new(0.0, 1.0, 0.0)), 0.3, 0.6);

    world.add_body(rb);

    /*
     * Cross shaped geometry
     */
    let delta1 = Iso3::new(Vec3::new(0.0, -5.0, 0.0), na::zero());
    let delta2 = Iso3::new(Vec3::new(-5.0, 0.0, 0.0), na::zero());
    let delta3 = Iso3::new(Vec3::new(5.0, 0.0, 0.0), na::zero());

    let mut cross_geoms = CompoundData::new();
    cross_geoms.push_shape(delta1, Cuboid::new(Vec3::new(4.96, 0.21, 0.21)), 1.0);
    cross_geoms.push_shape(delta2, Cuboid::new(Vec3::new(0.21, 4.96, 0.21)), 1.0);
    cross_geoms.push_shape(delta3, Cuboid::new(Vec3::new(0.21, 4.96, 0.21)), 1.0);

    let compound = Compound::new(cross_geoms);
    let mass     = compound.mass_properties(1.0);
    let cross    = box compound as Box<Shape3<f32>>;
    let cross    = Arc::new(cross);

    /*
     * Create the crosses 
     */
    let num     = 6;
    let rad     = 5.0;
    let shift   = rad * 2.0;
    let centerx = shift * (num as f32) / 2.0;
    let centery = 30.0 + shift / 2.0;
    let centerz = shift * (num as f32) / 2.0;

    for i in range(0u, num) {
        for j in range(0u, num) {
            for k in range(0u, num) {
                let x = i as f32 * shift - centerx;
                let y = j as f32 * shift + centery;
                let z = k as f32 * shift - centerz;

                let mut rb = RigidBody::new(cross.clone(), Some(mass), 0.3, 0.5);

                rb.append_translation(&Vec3::new(x, y, z));

                world.add_body(rb);
            }
        }
    }

    /*
     * Set up the testbed.
     */
    let mut testbed = Testbed::new(world);

    testbed.look_at(Pnt3::new(-30.0, 30.0, -30.0), Pnt3::new(0.0, 0.0, 0.0));
    testbed.run();
}
