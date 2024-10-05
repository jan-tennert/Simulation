use crate::simulation::asset::serialization::SerializedBody;
use bevy::color::palettes::css;
use bevy::color::Color;
use bevy::core::Name;
use bevy::math::{DVec3, Vec3};
use bevy::prelude::{default, Bundle, Component, Entity, Handle, Reflect, Scene, Srgba, Transform};
use bevy::render::primitives::Aabb;
use std::collections::VecDeque;
use anise::structure::planetocentric::ellipsoid::Ellipsoid;
use crate::simulation::components::horizons::AniseMetadata;

#[derive(Component, Clone, Default, Reflect, Copy)]
pub struct Mass(pub f64);

#[derive(Default, Component, Reflect, Clone, Copy)]
pub struct Velocity(pub DVec3);

#[derive(Default, Component, Reflect, Clone, Copy)]
pub struct Acceleration(pub DVec3);

#[derive(Component, Reflect, Clone, Default, Copy)]
pub struct Scale(pub f32);

#[derive(Component, Reflect, Clone, Default, Copy)]
pub struct RotationSpeed(pub f64);

#[derive(Component, Reflect, Clone, Default, Copy)]
pub struct AxialTilt {
    pub num: f32,
    pub axis: Option<Vec3>,
    pub applied: bool
}

#[derive(Component, Reflect, Clone, Default)]
pub struct ModelPath(pub String);

impl ModelPath {

    pub fn cleaned(&self) -> String {
        self.0.replace("models/", "").replace("#Scene0", "")
    }

    pub fn from_cleaned(value: &str) -> Self {
        ModelPath(format!("models/{}#Scene0", value))
    }

}

#[derive(Component, Reflect, Clone, Default)]
pub struct BodyChildren(pub Vec<Entity>);

#[derive(Component, Reflect, Clone)]
pub struct BodyParent(pub Entity);

#[derive(Component, Reflect, Clone)]
pub struct OrbitSettings {
    
    pub color: Color,
    pub step: f32,
    pub lines: VecDeque<Vec3>,
    pub force_direction: DVec3,
    pub orbit_line_multiplier: f32,
    pub hide_lines: bool,
    pub draw_lines: bool,
    pub display_force: bool,
    pub display_velocity: bool,
    pub arrow_scale: u64,
    pub period: f64,
                         
}

#[derive(Default, Component, Reflect, Clone)]
pub struct BillboardVisible(pub bool);

impl Default for OrbitSettings {
    
    fn default() -> Self {
        OrbitSettings { color: css::GREEN.into(), lines: VecDeque::with_capacity(3000), force_direction: DVec3::ZERO, draw_lines: false, step: 0.0, period: 0.0, display_force: false, display_velocity: false, arrow_scale: 1, hide_lines: false, orbit_line_multiplier: 1.0 }
    }
    
}

#[derive(Component, Clone, Default)]
pub struct SimPosition(pub DVec3);

#[derive(Component, Clone)]
pub struct Diameter {

    pub num: f32,
    pub applied: bool,
    pub ellipsoid: Ellipsoid,
    pub path: String,

}

impl Default for Diameter {

    fn default() -> Self {
        Diameter { num: 0.0, applied: false, ellipsoid: Ellipsoid::from_sphere(1.0), path: "".to_string() }
    }

}

#[derive(Component, Reflect, Clone)]
pub struct SceneHandle(pub Handle<Scene>, pub Entity);

#[derive(Component, Reflect, Clone)]
pub struct SceneEntity;

#[derive(Component, Reflect, Clone)]
pub struct LightSource(pub Entity);

//Types:
#[derive(Component, Reflect, Clone, Default)]
pub struct Star {
    
    pub use_imposter: bool,

}

#[derive(Component, Reflect, Clone, Default)]
pub struct Planet;

#[derive(Component, Reflect, Clone, Default)]
pub struct Moon;

#[derive(Bundle, Clone, Default)]
pub struct BodyBundle {

    pub mass: Mass,
    pub transform: Transform,
    pub sim_position: SimPosition,
    pub vel: Velocity,
    pub acc: Acceleration,
    pub scale: Scale,
    pub name: Name,
    pub model_path: ModelPath,
    pub orbit: OrbitSettings,
    pub rotation_speed: RotationSpeed,
    pub axial_tilt: AxialTilt,   
    pub diameter: Diameter,
    pub billboard_visible: BillboardVisible,
    pub naif_id: AniseMetadata,

}

impl From<SerializedBody> for BodyBundle {
    
    fn from(value: SerializedBody) -> Self {
        BodyBundle {
            mass: Mass(value.data.mass),
            sim_position: SimPosition(DVec3::from(value.data.starting_position) * 1000.0),
            vel: Velocity(DVec3::from(value.data.starting_velocity) * 1000.0),
            name: Name::new(value.data.name),
            model_path: ModelPath(format!("models/{}#Scene0", value.data.model_path)),
            diameter: Diameter {
                num: (value.data.diameter * 1000.0) as f32,
                path: value.data.model_path,
                ellipsoid: value.data.ellipsoid,
                ..default()
            },
            axial_tilt: AxialTilt {
                num: value.data.axial_tilt,
                ..default()
            },
            rotation_speed: RotationSpeed(value.data.rotation_speed),
            naif_id: AniseMetadata {
                target_id: value.data.naif_id,
                orientation_id: value.data.orientation_id,
            },
           ..default()
        }
    }
    
}

impl BodyBundle {

    pub fn empty(index: i32) -> Self {
        BodyBundle {
            mass: Mass(0.0),
            sim_position: SimPosition(DVec3::ZERO),
            vel: Velocity(DVec3::ZERO),
            name: Name::new(format!("New body {}", index)),
            model_path: ModelPath("models/earth.glb#Scene0".to_string()),
            diameter: Diameter {
                num: 0.0,
                ..default()
            },
            axial_tilt: AxialTilt {
                num: 0.0,
                ..default()
            },
            rotation_speed: RotationSpeed(0.0),
            ..default()
        }
    }

}