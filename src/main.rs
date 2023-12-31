mod api;

use api::{N2YOApi, RadioPasses};
use chrono::prelude::*;
use chrono_tz::Tz;

fn main() -> eframe::Result<()> {
    let native_options = eframe::NativeOptions::default();
    eframe::run_native(
        "Pass Finder",
        native_options,
        Box::new(|cc| Box::new(PassesApp::new(cc))),
    )
}

#[derive(serde::Serialize, serde::Deserialize, Default)]
pub struct PassesApp {
    lat: String,
    long: String,
    sats: Vec<usize>,
    api_key: String,
    cached_passes: Vec<RadioPasses>,

    #[serde(skip)]
    sat: String,
    #[serde(skip)]
    min_elevation: String,
    #[serde(skip)]
    days: String,
    #[serde(skip)]
    rough_elevation: String,
}

impl PassesApp {
    pub fn new(cc: &eframe::CreationContext<'_>) -> PassesApp {
        if let Some(c) = cc.storage {
            return eframe::get_value(c, eframe::APP_KEY).unwrap_or_default();
        }
        Default::default()
    }
}

fn convert_unix_to_local(date: i64) -> String {
    let naive = NaiveDateTime::from_timestamp_opt(date, 0).unwrap();
    let utc_time = naive.and_local_timezone(Utc).unwrap();
    let tz: Tz = "Australia/Sydney".parse().unwrap();
    utc_time.with_timezone(&tz).to_string()
}

impl eframe::App for PassesApp {
    fn save(&mut self, storage: &mut dyn eframe::Storage) {
        eframe::set_value(storage, eframe::APP_KEY, self);
    }
    fn update(&mut self, ctx: &egui::Context, _frame: &mut eframe::Frame) {
        let Self {
            lat,
            long,
            sats,
            sat,
            api_key,
            min_elevation,
            days,
            cached_passes,
            rough_elevation
        } = self;
        egui::SidePanel::left("sat_inputs")
            .max_width(150.)
            .show(ctx, |ui| {
                ui.heading("Satellite IDs");
                ui.horizontal(|ui| {
                    ui.label("Satellite ID:");
                    ui.text_edit_singleline(sat);
                    if ui.button("Add sat").clicked() {
                        sats.push(sat.parse().unwrap_or_else(|_| panic!()))
                    }
                });
                ui.separator();
                for (idx, sat) in sats.clone().into_iter().enumerate() {
                    ui.horizontal(|ui| {
                        ui.label(&sat.to_string());
                        if ui.button("Remove").clicked() {
                            sats.remove(idx);
                        }
                    });
                    ui.separator();
                }
            });
        egui::panel::CentralPanel::default().show(ctx, |ui| {
            ui.heading("Satellite passes");
            ui.separator();
            ui.horizontal(|ui| {
                ui.label("API Key:");
                ui.text_edit_singleline(api_key);
            });
            ui.horizontal(|ui| {
                ui.label("Latitude:");
                ui.text_edit_singleline(lat);
            });
            ui.horizontal(|ui| {
                ui.label("Longitude:");
                ui.text_edit_singleline(long);
            });
            ui.horizontal(|ui| {
                ui.label("Minimum elevation:");
                ui.text_edit_singleline(min_elevation);
            });
            ui.horizontal(|ui| {
                ui.label("Days to search:");
                ui.text_edit_singleline(days);
            });
            ui.horizontal(|ui| {
                ui.label("Rough elevation:");
                ui.text_edit_singleline(rough_elevation);
            });
            ui.separator();
            if ui.button("Get Passes").clicked() {
                let mut api = N2YOApi::new(api_key.to_string());
                *cached_passes = Vec::new();
                ui.separator();
                for id in sats {
                    api.get_radiopasses(
                        *id,
                        lat.parse().unwrap_or(0.),
                        long.parse().unwrap_or(0.),
                        min_elevation.parse().unwrap_or(0),
                        days.parse().unwrap_or(1),
                    );
                }
                *cached_passes = api.dispatch_reqs();
            }
            egui::ScrollArea::vertical().show(ui, |ui| {
                for (n, pass) in cached_passes.iter_mut().enumerate() {
                    egui::CollapsingHeader::new(&pass.info.satname)
                        .id_source(n)
                        .show(ui, |ui| {
                            ui.separator();
                            for (num, pass) in pass.passes.iter().enumerate() {
                                if let Ok(rough_elevation) = rough_elevation.parse::<usize>() {
                                    if (pass.max_el as usize) > rough_elevation.max(2)-2 && (pass.max_el as usize) < rough_elevation+2 {
                                        ui.heading(format!("Pass {}", num));
                                        ui.label(format!(
                                            "Starts: {}",
                                            convert_unix_to_local(pass.start_utc as i64)
                                        ));
                                        ui.label(format!(
                                            "Ends: {}",
                                            convert_unix_to_local(pass.end_utc as i64)
                                        ));
                                        ui.label(format!("Max elevation: {}", pass.max_el));
                                        ui.separator();
                                    }
                                } else {
                                    ui.heading(format!("Pass {}", num));
                                    ui.label(format!(
                                        "Starts: {}",
                                        convert_unix_to_local(pass.start_utc as i64)
                                    ));
                                    ui.label(format!(
                                        "Ends: {}",
                                        convert_unix_to_local(pass.end_utc as i64)
                                    ));
                                    ui.label(format!("Max elevation: {}", pass.max_el));
                                    ui.separator();
                                }
                            }
                        });
                }
            });
        });
    }
}
