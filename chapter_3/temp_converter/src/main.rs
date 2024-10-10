// ### Author
// 
// * UlrichNyx
//
// ### Date
//
// * 2024-10-10
//
// ### Description
//
// A Rust program using the `fltk` crate to create a graphical temperature converter. 
// The application allows users to input a temperature value and convert it between 
// Celsius and Fahrenheit based on a toggle switch. The GUI includes a button, 
// input field, and frames for displaying information.

use fltk::{app, button::Button, frame::Frame, input::Input, prelude::*, window::Window, button::CheckButton};

/// ### Main function
///
/// This is the entry point of the application. It sets up the FLTK app and window,
/// initializes input fields, frames, buttons, and a toggle switch to switch between
/// Fahrenheit and Celsius conversion modes. When the user inputs a value and presses
/// the convert button, it displays the converted temperature.
fn main() {
    let app = app::App::default();
    let mut wind = Window::new(100, 100, 400, 300, "Temperature Converter");
    let mut title = Frame::new(0, 0, 400, 50, "Convert from: ");
    let mut frame = Frame::new(0, 170, 400, 50, "");
    let input = Input::new(100, 80, 200, 30, "");
    let mut but = Button::new(160, 210, 80, 40, "Convert");

    // Create a CheckButton (toggle switch behavior)
    let mut switch = CheckButton::new(160, 100, 80, 40, "use Farenheit");

    // Set an initial state
    switch.set_value(false);

    // Set a callback to handle toggle behavior
    switch.set_callback(move |sw| {
        let mut mode = String::from("Farenheit");
        if sw.is_checked() {
            println!("Converting from Farenheit");
            mode = "Farenheit".to_string();
        } else {
            println!("Converting from Celsius");
            mode = "Celsius".to_string();
        }
        let title_text = format!("Convert from {}", mode);
        title.set_label(&title_text);
    });

    wind.end();
    wind.show();

    but.set_callback(move |_| {
        let text = input.value(); // Get the input field's value
        let result_num: Result<i32, std::num::ParseIntError> = text.parse();

        match result_num {
            Ok(num) => {
                if switch.is_checked() {
                    let celsius = (num as f64 - 32.0) * (5.0/9.0);
                    let text: String = celsius.to_string() + "°C";
                    frame.set_label(&text);  
                } else {
                    let farenheit: f64 = num as f64 * (9.0/5.0) + 32.0;
                    let text: String = farenheit.to_string() + "°F";
                    frame.set_label(&text);  
                }
            },
            Err(e) => println!("Failed to parse number: {}", e),
        }
    });

    app.run().unwrap();
}
