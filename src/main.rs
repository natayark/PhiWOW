use slint::PlatformError;
slint::include_modules!{}

fn main() -> Result<(), PlatformError> {
    let mainw = MainWindow::new()?;
    /*
    mainw.on_resize_window(|w, h| {
        mainw.global::<slint::WindowProperties>()
            .set_size(slint::LogicalSize::new(800.0, 600.0));
    });
    */
    mainw.show();
    slint::run_event_loop()  
}