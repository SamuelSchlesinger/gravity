use std::{collections::BTreeMap, f32::consts::PI};

use bevy::{prelude::*, sprite::MaterialMesh2dBundle, time::FixedTimestep};

fn main() {
    App::new()
        .add_plugins(DefaultPlugins)
        .add_startup_system(setup)
        .add_system_set(
            SystemSet::new()
                .with_run_criteria(FixedTimestep::steps_per_second(60.))
                .with_system(transform_is_position)
                .with_system(scale_for_mass)
                .with_system(gravity)
                .with_system(velocity),
        )
        .run();
}

const G: f32 = 1.0;

#[derive(Component)]
struct Velocity(Vec2);

#[derive(Component)]
struct Position(Vec2);

#[derive(Component)]
struct Mass(f32);

#[derive(Component)]
struct GameCamera;

fn setup(
    mut commands: Commands,
    mut meshes: ResMut<Assets<Mesh>>,
    mut materials: ResMut<Assets<ColorMaterial>>,
) {
    commands
        .spawn_bundle(Camera2dBundle::default())
        .insert(GameCamera);

    for i in -10..10 {
        for j in -10..10 {
            fn go() -> f32 {
                (rand::random::<f32>() + 1.) / 2.
            }
            let mass = Mass(rand::random::<f32>() * 100.);
            let color = Color::rgb(go(), go(), go());
            commands
                .spawn_bundle(MaterialMesh2dBundle {
                    mesh: meshes
                        .add(Mesh::from(shape::Circle {
                            radius: 10.,
                            vertices: 20,
                        }))
                        .into(),
                    transform: Transform::default().with_translation(Vec3::new(
                        (i * 30) as f32,
                        (j * 30) as f32,
                        0.,
                    )),
                    material: materials.add(ColorMaterial::from(color)),
                    ..default()
                })
                .insert(Position(Vec2::new((i * 30) as f32, (j * 30) as f32)))
                .insert(Velocity(Vec2::new(
                    (i as f32 * 2. * PI / 100.).sin(),
                    (j as f32 * 2. * PI / 100.).cos(),
                )))
                .insert(mass);
        }
    }
}

fn scale_for_mass(mut query: Query<(&mut Transform, &Mass)>) {
    for (mut transform, mass) in query.iter_mut() {
        transform.scale.x = (mass.0 / PI).sqrt();
        transform.scale.y = (mass.0 / PI).sqrt();
    }
}

fn transform_is_position(mut query: Query<(&mut Transform, &Position)>) {
    for (mut transform, position) in query.iter_mut() {
        transform.translation.x = position.0.x;
        transform.translation.y = position.0.y;
    }
}

fn velocity(mut query: Query<(&mut Position, &Velocity)>, time: Res<Time>) {
    for (mut position, velocity) in query.iter_mut() {
        position.0.x += velocity.0.x * time.delta_seconds();
        position.0.y += velocity.0.y * time.delta_seconds();
    }
}

fn gravity(mut query: Query<(Entity, &mut Velocity, &Position, &Mass)>) {
    let mut particle_map = vec![];
    for (entity, _velocity, position, mass) in query.iter() {
        particle_map.push((entity, (position.0.clone(), mass.0)));
    }

    for (entity, mut velocity, position, mass) in query.iter_mut() {
        for (other_entity, (other_position, other_mass)) in particle_map.iter() {
            if *other_entity != entity {
                let vector_to_other = *other_position - position.0;
                let r = vector_to_other.length();
                let direction = vector_to_other.normalize();
                let magnitude = G * (mass.0 * other_mass) / r.powi(2);
                velocity.0 += direction * magnitude;
            }
        }
    }
}
