// Copyright (C) 2026  Antonio-M. Corbi Bellot
//
// This program is free software: you can redistribute it and/or modify
// it under the terms of the GNU General Public License as published by
// the Free Software Foundation, either version 3 of the License, or
// (at your option) any later version.
//
// This program is distributed in the hope that it will be useful,
// but WITHOUT ANY WARRANTY; without even the implied warranty of
// MERCHANTABILITY or FITNESS FOR A PARTICULAR PURPOSE.  See the
// GNU General Public License for more details.
//
// You should have received a copy of the GNU General Public License
// along with this program.  If not, see <http://www.gnu.org/licenses/>.

// -- Uses: ---------------------------------------------------------------
use crate::types::Point3D;
use egui::{pos2, remap, Color32, Rect, Stroke};

// -- Constants: ----------------------------------------------------------
const MIN_ZOOM: f32 = 0.25;
const MAX_ZOOM: f32 = 10.00;

const MIN_ANGLE: f32 = 0.00;
const MAX_ANGLE: f32 = 360.00;

// -- Structs: ------------------------------------------------------------
pub struct App3D {
    rotx: bool,
    roty: bool,
    rotz: bool,
    angle: f32,
    zoom: f32,
}

// -- Implementation App3D: -----------------------------------------------
impl App3D {
    pub fn new() -> Self {
        Self {
            rotx: false,
            roty: false,
            rotz: true,
            angle: 0.0,
            zoom: 1.0,
        }
    }

    fn draw_circle(&self, painter: &egui::Painter) {
        // Obtener las dimensiones
        // let width = painter.clip_rect().width();
        // let height = painter.clip_rect().height();

        let world: Rect = Rect::from_min_max(pos2(0.0, 0.0), pos2(100.0, 100.0));
        let screen = painter.clip_rect();
        let scrx = remap(50.0, world.min.x..=world.max.x, screen.min.x..=screen.max.x);
        let scry = remap(50.0, world.min.y..=world.max.y, screen.min.y..=screen.max.y);

        // También puedes obtener los límites
        // let min = painter.clip_rect().min; // Esquina superior izquierda (Pos2)
        // let max = painter.clip_rect().max; // Esquina inferior derecha (Pos2)

        let centro = pos2(scrx, scry);
        let radio = 1.0 * self.zoom;
        let color = Color32::from_rgb(255, 255, 255);

        painter.circle_filled(centro, radio, color);
    }

    pub fn draw_object3D(&self, painter: &egui::Painter) {
        let world: Rect = Rect::from_min_max(pos2(0.0, 0.0), pos2(1.0, 1.0));
        let screen: Rect = painter.clip_rect();
        let dz = self.zoom;

        for f in crate::penger::FS {
            for i in 0..f.len() {
                let a = crate::penger::VS[f[i] as usize];
                let b = crate::penger::VS[f[(i + 1) % f.len()] as usize];
                // dbg!(a);
                // dbg!(b);
            }
        }
    }

    pub fn draw_contents(&self, painter: &egui::Painter) {
        self.draw_circle(painter);
        self.draw_object3D(painter);
    }
}

// -- Implementation eframe@App3D: ----------------------------------------
impl eframe::App for App3D {
    /// Called by the framework to save state before shutdown.
    // fn save(&mut self, storage: &mut dyn eframe::Storage) {
    //     eframe::set_value(storage, eframe::APP_KEY, self);
    // }

    /// Called each time the UI needs repainting, which may be many times per second.
    fn update(&mut self, ctx: &egui::Context, frame: &mut eframe::Frame) {
        egui::CentralPanel::default().show(ctx, |ui| {
            // Panel de controles en la parte superior
            ui.vertical(|ui| {
                ui.horizontal(|ui| {
                    ui.colored_label(egui::Color32::RED, "·:Penger 3D:·");

                    ui.colored_label(egui::Color32::LIGHT_BLUE, "Theme: ");
                    egui::widgets::global_theme_preference_buttons(ui);

                    let is_web = cfg!(target_arch = "wasm32");
                    if !is_web {
                        //ui.menu_button("File", |ui| {
                        if ui.button("Quit").clicked() {
                            ctx.send_viewport_cmd(egui::ViewportCommand::Close);
                        }
                        //});
                        ui.add_space(16.0);
                    }
                });

                ui.separator();

                ui.horizontal(|ui| {
                    ui.checkbox(&mut self.rotx, "Rotate X");
                    ui.checkbox(&mut self.roty, "Rotate Y");
                    ui.checkbox(&mut self.rotz, "Rotate Z");
                    ui.separator();
                    ui.colored_label(egui::Color32::LIGHT_YELLOW, "Angle: ");
                    ui.add(
                        egui::DragValue::new(&mut self.angle)
                            .speed(0.1)
                            .range(MIN_ANGLE..=MAX_ANGLE),
                    );
                    ui.separator();
                    ui.colored_label(egui::Color32::LIGHT_YELLOW, "Zoom: ");
                    ui.add(
                        egui::DragValue::new(&mut self.zoom)
                            .speed(0.1)
                            .range(MIN_ZOOM..=MAX_ZOOM),
                    );
                    ui.separator();

                    if ui.button("Restart View").clicked() {
                        //self.calculate_bounds_and_fit(ui.available_rect_before_wrap());
                        *self = Self::new();
                    }
                });

                ui.separator();
            });

            // El área de dibujo para el mapa
            let available_rect_before_wrap = ui.available_rect_before_wrap();
            let painter = ui.painter_at(available_rect_before_wrap);

            // Dibujar un fondo para el área del mapa
            painter.rect_filled(
                available_rect_before_wrap,
                0.0,
                egui::Color32::from_rgb(30, 30, 30),
            );

            self.draw_contents(&painter);

            // Si hay datos cargados pero la escala aún es la predeterminada (1.0),
            // y aún no se ha ajustado, hacerlo ahora.
            // Esto asegura que el mapa se ajuste automáticamente en la primera renderización
            // o después de una carga.
            if self.zoom == 1.0 {
                //self.calculate_bounds_and_fit(available_rect_before_wrap);
            }

            // Draw the points
            // for feature in &self.geo_features {
            //     if let Some(geometry) = &feature.geometry {
            //         self.draw_geometry(&painter, &geometry.value, available_rect_before_wrap);
            //     }
            // }

            ui.with_layout(egui::Layout::bottom_up(egui::Align::LEFT), |ui| {
                powered_by_egui_and_eframe(ui);
                egui::warn_if_debug_build(ui);
                ui.separator();
            });

            // Continuous update
            ctx.request_repaint();
        });
    }
}

// -- Free functions: -----------------------------------------------------
fn powered_by_egui_and_eframe(ui: &mut egui::Ui) {
    ui.horizontal(|ui| {
        ui.spacing_mut().item_spacing.x = 0.0;
        ui.label("Powered by ");
        ui.hyperlink_to("egui", "https://github.com/emilk/egui");
        ui.label(" and ");
        ui.hyperlink_to(
            "eframe",
            "https://github.com/emilk/egui/tree/master/crates/eframe",
        );
        ui.label(".");
    });
}
