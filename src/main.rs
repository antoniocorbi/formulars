mod app;
mod penger;
mod types;

// fn main() {
//     let l: types::Line;
//     let ll: types::Lines;
//     let p: types::Point3D;
//
//     println!("point3d[300]  = {:?}", crate::penger::VS[300]);
//     println!("line[300]  = {:?}", crate::penger::FS[300]);
// }

// -- Native App: ---------------------------------------------------------
// When compiling natively:
#[cfg(not(target_arch = "wasm32"))]
fn main() -> eframe::Result {
    //env_logger::init(); // Log to stderr (if you run with `RUST_LOG=debug`).

    let native_options = eframe::NativeOptions {
        viewport: egui::ViewportBuilder::default()
            .with_app_id("Floater") // niri uses this app_id to make the window floating
            .with_inner_size([1024.0, 768.0])
            .with_min_inner_size([500.0, 300.0])
            .with_icon(
                // NOTE: Adding an icon is optional
                eframe::icon_data::from_png_bytes(&include_bytes!("../assets/icon-256.png")[..])
                    .expect("Failed to load icon"),
            ),
        ..Default::default()
    };

    eframe::run_native(
        "Penger3D GUI",
        native_options,
        Box::new(|cc| {
            let cc = cc;
            Ok(Box::new(app::App3D::new()))
        }),
    )
}
