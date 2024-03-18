use bevy::math::Vec3;
/// Create a few trees, a light source and a camera, and position them such that the trees are lit
/// and visible from the camera. This code contains code copied from pyramid.rs.
use bevy::prelude::*;
use bevy::render::mesh::{Indices, Mesh};
use bevy::render::render_resource::PrimitiveTopology;

fn setup(
    mut commands: Commands,
    mut meshes: ResMut<Assets<Mesh>>,
    mut materials: ResMut<Assets<StandardMaterial>>,
) {
    // Create and add a default material
    let mut material_handle_crown = materials.add(StandardMaterial {
        base_color: Color::rgb(0.3, 0.8, 0.3),
        ..Default::default()
    });

    let mut material_handle_trunk = materials.add(StandardMaterial {
        base_color: Color::rgb(0.5, 0.3, 0.3),
        ..Default::default()
    });

    create_tree(
        &mut commands,
        &mut meshes,
        &mut material_handle_crown,
        &mut material_handle_trunk,
        1.,
        0.3,
        2.3,
        0.8,
        Vec3::new(0.5, 0., -8.),
    );

    create_tree(
        &mut commands,
        &mut meshes,
        &mut material_handle_crown,
        &mut material_handle_trunk,
        1.,
        0.3,
        2.3,
        0.8,
        Vec3::new(-0.5, 0., -7.),
    );

    create_tree(
        &mut commands,
        &mut meshes,
        &mut material_handle_crown,
        &mut material_handle_trunk,
        1.,
        0.25,
        3.5,
        0.6,
        Vec3::new(-1.5, 0., -10.),
    );

    // Light
    commands.spawn(PointLightBundle {
        transform: Transform::from_xyz(2., 5., 2.),
        ..Default::default()
    });

    // Camera
    commands.spawn(Camera3dBundle {
        transform: Transform::from_xyz(0., 1.5, 0.),
        ..Default::default()
    });
}

/// Construct a tree with a pyramid for the crown and a shape::Box for the trunk. The height and
/// width of each are passed as parameters. The bottom of the trunk is positioned at `location`.
/// Materials for the crown and trunk are passed in `material_handle_crown' and
/// `material_handle_trunk`. The tree and its meshes are added to `commands` and `meshes`.
fn create_tree(
    commands: &mut Commands,
    meshes: &mut ResMut<Assets<Mesh>>,
    material_handle_crown: &Handle<StandardMaterial>,
    material_handle_trunk: &Handle<StandardMaterial>,
    trunk_height: f32,
    trunk_width: f32,
    crown_height: f32,
    crown_width: f32,
    location: Vec3,
) {
    // Create a mesh for the tree trunk
    commands.spawn(PbrBundle {
        mesh: meshes.add(Mesh::from(shape::Box::new(
            trunk_width,
            trunk_height,
            trunk_width,
        ))),
        material: material_handle_trunk.clone(),
        transform: Transform::from_translation(location + Vec3::new(0., trunk_height / 2.0, 0.)),
        ..Default::default()
    });

    // Create a mesh for the tree top
    commands.spawn(PbrBundle {
        mesh: meshes.add(Mesh::from(Pyramid::new(8, crown_width, crown_height))),
        material: material_handle_crown.clone(),
        transform: Transform::from_translation(location + Vec3::new(0., trunk_height, 0.)),
        ..Default::default()
    });
}

fn main() {
    App::new()
        .add_plugins(DefaultPlugins)
        .add_systems(Startup, setup)
        .run();
}

/* Everything below this line is intended to be in a separate file analogous to those in:
 * bevy_render/src/mesh/shape/torus.rs
 */

/// A pyramid with a base in the XZ plane centered on the origin and its apex along +Y.

#[derive(Debug, Clone, Copy)]
pub struct Pyramid {
    pub sides: u32,
    pub side_length: f32,
    pub height: f32,
}

impl Pyramid {
    pub fn new(sides: u32, side_length: f32, height: f32) -> Self {
        assert!(sides > 2, "Pyramids must have 3 or more sides");
        Pyramid {
            sides,
            side_length,
            height,
        }
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
            [
                (pos.x + limit) / (limit * 2.),
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

        let num_vertexes = 6 * p.sides - 6;

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
