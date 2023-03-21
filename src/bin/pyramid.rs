/// Create a pyramid, light source and camera, and position them such that the pyramid is lit and
/// visible from the camera.

use bevy::prelude::*;

/* For the pyramid code intended to be split into a separate file. */
use bevy::render::mesh::{Indices, Mesh};
use bevy::math::Vec3;
// use wgpu::PrimitiveTopology;  //// This variant is used by code *within* Bevy
use bevy::render::render_resource::PrimitiveTopology;


fn setup(
    mut commands: Commands,
    mut meshes: ResMut<Assets<Mesh>>,
    mut materials: ResMut<Assets<StandardMaterial>>,
) {

    // Create and add a default material
    let material_handle = materials.add(StandardMaterial {
        base_color: Color::rgb(0.7, 0.6, 0.7),
        ..Default::default()
    });

    // Location to place pyramid in worldspace
    let pyramid_location = Vec3::new(0.5, -0.5, -4.);

    // Create a mesh for the pyramid
    commands.spawn(PbrBundle {
        mesh: meshes.add(Mesh::from(Pyramid::new(5, 1.2, 1.))),
        material: material_handle.clone(),
        transform: Transform::from_translation(pyramid_location)
            .with_rotation(Quat::from_rotation_x(0.2)),
        ..Default::default()
    });

    // Light
    commands.spawn(PointLightBundle {
        transform: Transform::from_xyz(2., 5., 2.),
        ..Default::default()
    });

    // Camera
    commands.spawn(Camera3dBundle {
        ..Default::default()
    });
}

fn main() {
    App::new()
        .add_plugins(DefaultPlugins)
        .add_startup_system(setup)
        .run();
}


/* Everything below this line is intended to be in a separate file analogous to those in:
 * bevy_render/src/mesh/shape/torus.rs
 */

/// A pyramid with a squared base. The base lies in the XZ plane and its center is the origin. The
/// apex lies on the positive Y axis.

#[derive(Debug, Clone, Copy)]
pub struct Pyramid {
    pub sides: u32,
    pub side_length: f32,
    pub height: f32,
}

impl Pyramid {
    pub fn new(sides: u32, side_length: f32, height: f32) -> Self {
        assert!(sides > 2, "Pyramids must have 3 or more sides");  // TODO Better way to handle this error?
        Pyramid { sides, side_length, height }
    }
}

impl Default for Pyramid {
    fn default() -> Self {
        Pyramid {
            sides: 4,
            side_length: 1.0,
            height: 1.0,
        }
    }
}

impl From<Pyramid> for Mesh {
    fn from(p: Pyramid) -> Self {
        let angle = std::f32::consts::PI * 2. / p.sides as f32;
        let half_width = p.side_length / 2.;
        let radius = half_width / f32::sin(angle / 2.);
        let apex = Vec3::new(0., p.height, 0.);

        println!("sides = {}, side_length = {}, radius = {}", p.sides, p.side_length, radius);

        // Calculate vertexes forming each face. The first vertex is located on the positive Z axis
        // and faces are created counter-clockwise (looking down the Y axis towards negative Y.
        let mut base_vertexes = Vec::with_capacity(p.sides as usize);
        for s in 0..p.sides {
            let a = angle * s as f32;
            base_vertexes.push(Vec3::new(radius * f32::sin(a), 0., radius * f32::cos(a)));
        }

        let mut vertexes = Vec::new();
        let mut bottom_vertexes = Vec::new();

        for s in 0..p.sides as usize {
            // Determine normal by creating two vectors from the apex to the two other corners of
            // this face, calculating their cross product and normalizing the result.
            let b = &base_vertexes[s];
            let c = &base_vertexes[(s + 1) % p.sides as usize];

            let ver_ab = *b - apex;
            let ver_ac = *c - apex;
            let normal = ver_ab.cross(ver_ac).normalize().to_array();

            vertexes.push((apex.to_array(), normal, [0.5, 1.]));
            vertexes.push((b.to_array(), normal, [0., 0.]));
            vertexes.push((c.to_array(), normal, [1., 0.]));

            bottom_vertexes.push(b);
        }


        // Translate a `Vec3` position on the bottom face to u, v coordinates returned as an
        // array. `limit` is the largest absolute distance that the position can be from the
        // origin. This function therefore translates -limit..=limit to 0..=1 for both axes.
        fn xz_to_uv(pos: &Vec3, limit: f32) -> [f32; 2] {
            [ (pos.x + limit) / (limit * 2.),
                (pos.z + limit) / (limit * 2.),
            ]
        }

        // Vertexes for the bottom face were saved in a counter-clockwise direction when looking
        // from +Y to the origin. Their order is reversed so they are CCW when looking at the
        // bottom face of the pyramid from -Y.
        bottom_vertexes.reverse();

        // The last vertex in the list is the one nearest +Z. It is used as the first vertex in all
        // triangles forming the bottom face.
        let vertex_nearest_pos_z = bottom_vertexes.pop().unwrap();
        let texture_bound = vertex_nearest_pos_z.z;

        for pair in bottom_vertexes.windows(2) {
            let normal = [0., -1., 0.];


            vertexes.push((vertex_nearest_pos_z.to_array(), normal, [0.5, 1.]));
            vertexes.push((pair[0].to_array(), normal, xz_to_uv(pair[0], texture_bound)));
            vertexes.push((pair[1].to_array(), normal, xz_to_uv(pair[1], texture_bound)));
        }

        let num_vertexes =  6 * p.sides - 6;

        let mut positions = Vec::with_capacity(num_vertexes as usize);
        let mut normals = Vec::with_capacity(num_vertexes as usize);
        let mut uvs = Vec::with_capacity(num_vertexes as usize);

        for (position, normal, uv) in vertexes.iter() {
            positions.push(*position);
            normals.push(*normal);
            uvs.push(*uv);
        }

        let mut mesh = Mesh::new(PrimitiveTopology::TriangleList);
        mesh.insert_attribute(Mesh::ATTRIBUTE_POSITION, positions);
        mesh.insert_attribute(Mesh::ATTRIBUTE_NORMAL, normals);
        mesh.insert_attribute(Mesh::ATTRIBUTE_UV_0, uvs);
        mesh.set_indices(Some(Indices::U32((0..num_vertexes).collect())));
        mesh
    }
}
