use bevy::diagnostic::{DiagnosticsStore, FrameTimeDiagnosticsPlugin};
use bevy::prelude::{Commands, NextState, Query, Res, ResMut, Time, Window};
use bevy::window::PresentMode;
use bevy_egui::{egui, EguiContexts};
use bevy_egui::egui::Align;
use chrono::NaiveDateTime;
use crate::setup::StartingTime;
use crate::simulation::components::lock_on::LockOn;
use crate::simulation::components::physics::{Pause, SubSteps};
use crate::simulation::components::speed::Speed;
use crate::simulation::{SimState, SimStateType};
use crate::simulation::components::editor::{EditorSystemType, EditorSystems};
use crate::simulation::ui::{SimTime, UiState};
use crate::simulation::ui::bottom_bar::get_date_from_millis;

pub fn editor_bottom_bar(
    mut sim_time: ResMut<SimTime>,
    mut egui_context: EguiContexts,
    mut windows: Query<&mut Window>,
    mut lock_on_parent: ResMut<LockOn>,
    mut state: ResMut<NextState<SimState>>,
    starting_time: Res<StartingTime>,
    diagnostics: Res<DiagnosticsStore>,
    systems: Res<EditorSystems>,
    mut commands: Commands
) {
    let mut window = windows.single_mut();
    let date = get_date_from_millis(starting_time.0, sim_time.0);
    egui::TopBottomPanel::bottom("time_panel")
        .resizable(false)
        .show(egui_context.ctx_mut(), |ui| {
            ui.horizontal(|ui| {
                ui.with_layout(egui::Layout::left_to_right(egui::Align::LEFT), |ui| {
                    ui.horizontal_centered(|ui| {
                        ui.label(format!(
                            "Date: {}",
                            date.format("%d.%m.%Y"),
                        ));
                    });
                });

                ui.with_layout(egui::Layout::left_to_right(Align::Center), |ui| {
                    if ui.button("Reset").on_hover_text("Reset scenario from file").clicked() {
                        let _ = state.set(SimState::Reset);
                    }
                    if ui.button("Save").on_hover_text("Save scenario to file").clicked() {
                        commands.run_system(systems.0[EditorSystemType::SAVE_SCENARIO])
                    }
                });

                ui.with_layout(egui::Layout::right_to_left(egui::Align::RIGHT), |ui| {
                    ui.checkbox(&mut lock_on_parent.enabled, "Lock on Parent");
                    let mut vsync = window.present_mode == PresentMode::AutoVsync;
                    let old_option = vsync;
                    ui.checkbox(&mut vsync, "VSync");
                    if old_option != vsync {
                        if vsync {
                            window.present_mode = PresentMode::AutoVsync;
                        } else {
                            window.present_mode = PresentMode::AutoNoVsync;
                        }
                    }
                    if let Some(fps) = diagnostics.get(&FrameTimeDiagnosticsPlugin::FPS) {
                        if let Some(value) = fps.smoothed() {
                            // Update the value of the second section
                            ui.label(format!("{:.0} FPS", value));
                        }
                    }
                })
            });
        });
}
