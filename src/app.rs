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
use crate::types::{Axe, Point2D, Point3D};
use egui::{pos2, remap, Color32, Pos2, Rect, Stroke};

// -- Constants: ----------------------------------------------------------
const MIN_ZOOM: f32 = 0.25;
const MAX_ZOOM: f32 = 15.00;

const MIN_ANGLE_STEP: f32 = 0.00;
const MAX_ANGLE_STEP: f32 = 10.00;

// -- Structs: ------------------------------------------------------------
pub struct App3D {
    rotx: bool,
    roty: bool,
    rotz: bool,
    draw_vs: bool,
    draw_fs: bool,
    angle_step: f32,
    zoom: f32,
    file_path: String,
}

// -- Implementation App3D: -----------------------------------------------
impl App3D {
    pub fn new() -> Self {
        Self {
            rotx: false,
            roty: true,
            rotz: false,
            draw_vs: false,
            draw_fs: true,
            angle_step: 0.0,
            zoom: 1.0,
            file_path: String::new(),
        }
    }

    // fn draw_circle(&self, painter: &egui::Painter) {
    //     // Obtener las dimensiones
    //     // let width = painter.clip_rect().width();
    //     // let height = painter.clip_rect().height();
    //
    //     let world: Rect = Rect::from_min_max(pos2(0.0, 0.0), pos2(100.0, 100.0));
    //     let screen = painter.clip_rect();
    //     let scrx = remap(50.0, world.min.x..=world.max.x, screen.min.x..=screen.max.x);
    //     let scry = remap(50.0, world.min.y..=world.max.y, screen.min.y..=screen.max.y);
    //
    //     // También puedes obtener los límites
    //     // let min = painter.clip_rect().min; // Esquina superior izquierda (Pos2)
    //     // let max = painter.clip_rect().max; // Esquina inferior derecha (Pos2)
    //
    //     let centro = pos2(scrx, scry);
    //     let radio = 1.0 * self.zoom;
    //     let color = Color32::from_rgb(255, 255, 255);
    //
    //     painter.circle_filled(centro, radio, color);
    // }

    fn draw_point(p: Point2D, zoom: f32, painter: &egui::Painter) {
        // También puedes obtener los límites
        // let min = painter.clip_rect().min; // Esquina superior izquierda (Pos2)
        // let max = painter.clip_rect().max; // Esquina inferior derecha (Pos2)

        let centro = pos2(p.x, p.y);
        // let radio = zoom.min(3.5);
        let radio = (zoom + 0.25) / 3.0;
        // let color = Color32::from_rgb(255, 255, 255);
        let color = Color32::LIGHT_RED;

        painter.circle_filled(centro, radio, color);
    }

    fn draw_lines(lines: &Vec<Pos2>, painter: &egui::Painter) {
        let stroke = Stroke::new(0.5, egui::Color32::LIGHT_YELLOW);
        painter.line(lines.to_vec(), stroke);
    }

    pub fn draw_object3D(&self, painter: &egui::Painter) {
        let dz = MAX_ZOOM - self.zoom;
        let worldr: Rect = Rect::from_min_max(pos2(-1.0, -1.0), pos2(1.0, 1.0));
        let screenr: Rect = painter.clip_rect();
        static mut ANGLE: f32 = 0.0;
        unsafe {
            ANGLE = (ANGLE + self.angle_step) % 360.0;
            // if ANGLE > 360.0 {
            //     ANGLE = 0.0;
            // }
        }

        // Draw points@vertices
        if self.draw_vs {
            for v in crate::penger::VS {
                let mut a = *v;
                a.y = -1.0 * a.y;

                unsafe {
                    if self.rotx {
                        a = a.rotate(ANGLE, Axe::X);
                    }
                    if self.roty {
                        a = a.rotate(ANGLE, Axe::Y);
                    }
                    if self.rotz {
                        a = a.rotate(ANGLE, Axe::Z);
                    }
                }
                let p2d = a.convert_to_2D(dz, &worldr, &screenr);
                App3D::draw_point(p2d, self.zoom, painter);
            }
        }

        // Draw Lines between vertices
        if self.draw_fs {
            let mut lines: Vec<Pos2> = vec![];
            for f in crate::penger::FS {
                for i in 0..f.len() {
                    let mut a = crate::penger::VS[f[i] as usize];
                    let mut b = crate::penger::VS[f[(i + 1) % f.len()] as usize];
                    a.y = -1.0 * a.y; // Invert Y-coordinate top-down
                    b.y = -1.0 * b.y; // Invert Y-coordinate top-down

                    unsafe {
                        if self.rotx {
                            a = a.rotate(ANGLE, Axe::X);
                            b = b.rotate(ANGLE, Axe::X);
                        }
                        if self.roty {
                            a = a.rotate(ANGLE, Axe::Y);
                            b = b.rotate(ANGLE, Axe::Y);
                        }
                        if self.rotz {
                            a = a.rotate(ANGLE, Axe::Z);
                            b = b.rotate(ANGLE, Axe::Z);
                        }
                    }
                    // let p1 = App3D::world2screen(a.translate_z(dz).project(), painter);
                    // let p2 = App3D::world2screen(b.translate_z(dz).project(), painter);

                    // let p1 = a.translate_z(dz).project().world2screen(worldr, screenr);
                    // let p2 = b.translate_z(dz).project().world2screen(worldr, screenr);

                    let p1 = a.convert_to_2D(dz, &worldr, &screenr);
                    let p2 = b.convert_to_2D(dz, &worldr, &screenr);

                    let p1: Pos2 = pos2(p1.x, p1.y);
                    let p2: Pos2 = pos2(p2.x, p2.y);
                    lines.push(p1);
                    lines.push(p2);
                }
            }
            App3D::draw_lines(&lines, painter);
        }
    }

    pub fn draw_contents(&self, painter: &egui::Painter) {
        //self.draw_circle(painter);
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
                ui.label("Obj file:");
                ui.text_edit_singleline(&mut self.file_path);
                if ui.button("Loaf file").clicked() {
                    // if let Err(e) = self.load_topojson_from_file(&self.file_path) {
                    //     self.error_message = Some(format!("Error al cargar el archivo: {}", e));
                    // } else {
                    //     // Una vez cargado el archivo, recalcular los límites y ajustes
                    //     self.calculate_bounds_and_fit(ui.available_rect_before_wrap());
                    // }
                }
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

                    ui.horizontal(|ui| {
                        ui.checkbox(&mut self.draw_vs, "Vertices");
                        ui.checkbox(&mut self.draw_fs, "Faces");
                    });

                    ui.separator();
                    ui.colored_label(egui::Color32::LIGHT_YELLOW, "Angle Step: ");
                    ui.add(
                        egui::DragValue::new(&mut self.angle_step)
                            .speed(0.1)
                            .range(MIN_ANGLE_STEP..=MAX_ANGLE_STEP),
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

            // El área de dibujo para el objeto 3D
            let mut available_rect_before_wrap = ui.available_rect_before_wrap();
            available_rect_before_wrap.max.y -= 40.0; // Important for clipping
            let mut painter = ui.painter_at(available_rect_before_wrap);

            // Dibujar un fondo para el área del mapa
            painter.rect_filled(
                available_rect_before_wrap,
                0.0,
                egui::Color32::from_rgb(50, 50, 50),
            );
            let screenr: Rect = painter.clip_rect();
            painter.set_clip_rect(screenr);

            self.draw_contents(&painter);

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
