#![cfg_attr(not(debug_assertions), windows_subsystem = "windows")]
mod body;
mod constants;
mod setup;
mod bodies;
mod physics;
mod egui_input_block;
mod speed;
mod selection;
mod menu;
mod skybox;
mod diameter;
mod ui;
mod orbit_lines;
mod reset;
mod rotation;
mod serialization;
mod lock_on;
mod input;
mod camera;
mod loading;

use bevy::app::{App, PluginGroup, AppLabel};
use bevy::DefaultPlugins;
use bevy::diagnostic::{LogDiagnosticsPlugin, FrameTimeDiagnosticsPlugin};
use bevy::prelude::{default, States, Msaa};
use bevy::render::RenderPlugin;
use bevy::render::settings::{Backends, WgpuSettings};
use bevy::window::{WindowPlugin, Window, PresentMode};
use bevy_inspector_egui::quick::WorldInspectorPlugin;
use camera::PanOrbitCameraPlugin;
use diameter::DiameterPlugin;
use input::InputPlugin;
use loading::LoadingPlugin;
use lock_on::LockOnPlugin;
use orbit_lines::OrbitLinePlugin;
use reset::ResetPlugin;
use rotation::RotationPlugin;
use serialization::{SerializationPlugin, SimulationData};
use skybox::SkyboxPlugin;
use speed::SpeedPlugin;
use ui::UIPlugin;
use crate::menu::MenuPlugin;
use crate::physics::PhysicsPlugin;
use crate::selection::SelectionPlugin;
use crate::setup::SetupPlugin;

#[derive(Debug, Clone, Copy, Default, Eq, PartialEq, Hash, States)]
pub enum SimState {
    #[default]
    Menu,
    Loading,
    Simulation,
    Reset,
    ExitToMainMenu
}

fn main() {
    App::new()
     //   .add_plugins(DefaultPlugins)
        .add_plugins(DefaultPlugins
            .set(RenderPlugin {
                wgpu_settings: WgpuSettings {
                    backends: Some(Backends::VULKAN),
                    ..default()
                },
            })
            .set(WindowPlugin {
                primary_window: Some(Window {
                    title: "Solar System Simulation (Jan Tennert)".to_string(),
                    present_mode: PresentMode::AutoVsync,
                    ..default()
                }),
                ..default()
            })
        ) 
        .add_plugins(WorldInspectorPlugin::new())
  //      .add_plugins(DefaultPickingPlugins)
        .add_plugins(LockOnPlugin)
        .add_plugins(SerializationPlugin)
        .add_plugins(LoadingPlugin)
   //     .add_plugins(PanOrbitCameraPlugin)
        .add_plugins(PanOrbitCameraPlugin)
        .add_plugins(SetupPlugin)
        .add_plugins(PhysicsPlugin)
        .add_plugins(MenuPlugin)
        .add_plugins(InputPlugin)
        .add_plugins(SelectionPlugin)
        .add_plugins(SkyboxPlugin)
        .add_plugins(UIPlugin)
        .add_plugins(SpeedPlugin)
        .add_plugins(ResetPlugin)
        .add_plugins(OrbitLinePlugin)
        .add_plugins(RotationPlugin)
        .add_plugins(FrameTimeDiagnosticsPlugin::default())
        .add_plugins(DiameterPlugin)
    //    .add_plugins(ScreenDiagnosticsPlugin::default())
  //      .add_plugins(ScreenFrameDiagnosticsPlugin)
        .add_state::<SimState>()
        .run();
}