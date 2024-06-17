extern crate gtk;
use gtk::prelude::*;
use std::process::Command;

fn main() {
    // Perform initial authentication with pkexec if setting mode requires it
    authenticate_with_pkexec_if_needed();

    // Initialize GTK application
    gtk::init().expect("Failed to initialize GTK.");

    // Create the main application window with specific dimensions
    let window = gtk::Window::new(gtk::WindowType::Toplevel);
    window.set_title("EnvyControl Gtk");
    window.set_default_size(520, 300); // Set specific dimensions
    window.set_position(gtk::WindowPosition::Center); // Center the window on the screen

    // Load the image
    let image_path = "/usr/share/envycontrol-gtk/icons/envy-banner.png";
    let image = gtk::Image::from_file(image_path);

    // Create labels and buttons
    let label = gtk::Label::new(None);
    update_graphics_mode(&label);

    let switch_button = gtk::Button::with_label("Switch Mode");
    let quit_button = gtk::Button::with_label("Quit");

    // Set padding for buttons
    switch_button.set_margin_start(20); // Padding on the left side of the button text
    switch_button.set_margin_end(20); // Padding on the right side of the button text
    quit_button.set_margin_start(20); // Padding on the left side of the button text
    quit_button.set_margin_end(20); // Padding on the right side of the button text

    // Set size request for buttons (5 pixels smaller)
    let button_width = 175;
    let button_height = 55;
    switch_button.set_size_request(button_width, button_height);
    quit_button.set_size_request(button_width, button_height);

    // Create a vertical box container for labels, image, and buttons
    let vbox = gtk::Box::new(gtk::Orientation::Vertical, 10); // Increased spacing between elements
    vbox.set_margin_top(20); // Margin at the top
    vbox.set_margin_bottom(20); // Margin at the bottom
    vbox.set_margin_start(40); // Margin on the left
    vbox.set_margin_end(40); // Margin on the right

    // Add the image with some space above it
    let image_box = gtk::Box::new(gtk::Orientation::Vertical, 10);
    image_box.pack_start(&image, false, false, 0); // Image at the top

    vbox.pack_start(&image_box, false, false, 0); // Add image_box to vbox

    // Add label with some space between the image and label
    let label_box = gtk::Box::new(gtk::Orientation::Vertical, 10);
    label_box.set_margin_top(20); // Space above the label

    let separator = gtk::Separator::new(gtk::Orientation::Horizontal);
    separator.set_margin_bottom(20); // Space below the separator

    label_box.pack_start(&label, false, false, 0); // Label below the separator
    label_box.pack_start(&separator, false, false, 0); // Separator below the label

    vbox.pack_start(&label_box, false, false, 0); // Add label_box to vbox

    // Create a horizontal box for buttons with spacing between them
    let button_box = gtk::Box::new(gtk::Orientation::Horizontal, 10); // 10px spacing between buttons

    // Pack the buttons with expand and fill properties to center horizontally
    switch_button.set_hexpand(true);
    quit_button.set_hexpand(true);

    button_box.pack_start(&switch_button, false, false, 0); // Switch Mode button
    button_box.pack_start(&quit_button, false, false, 0); // Quit button

    vbox.pack_start(&button_box, false, false, 0); // Add button_box to vbox

    // Add the vertical box to the window
    window.add(&vbox);

    // Handle button click events
    switch_button.connect_clicked({
        let window = window.clone();
        let label = label.clone();
        move |_| {
            let current_mode = get_current_graphics_mode();
            println!("Switch button clicked. Current graphics mode: {}", current_mode); // Debug info

            if current_mode == "integrated" {
                set_graphics_mode("hybrid");
            } else if current_mode == "hybrid" {
                set_graphics_mode("integrated");
            } else {
                println!("Unknown graphics mode detected.");
            }

            update_graphics_mode(&label);
            show_reboot_dialog(&window);
        }
    });

    quit_button.connect_clicked(|_| {
        println!("Quit button clicked."); // Debug info
        gtk::main_quit();
    });

    // Display all widgets
    window.show_all();

    // Start the GTK main event loop
    gtk::main();
}

/// Function to perform initial authentication with pkexec if setting mode requires it
fn authenticate_with_pkexec_if_needed() {
    // Implement logic here to authenticate with pkexec if setting mode requires it
    // For simplicity, assume authentication is handled by the system or a prior setup
}

/// Function to get the current graphics mode using `envycontrol -q`
fn get_current_graphics_mode() -> String {
    let output = Command::new("envycontrol")
        .arg("-q")
        .output()
        .expect("Failed to execute command");

    if output.status.success() {
        let stdout = String::from_utf8_lossy(&output.stdout);
        println!("Command output: {}", stdout); // Debug info
        stdout.trim().to_string()
    } else {
        let stderr = String::from_utf8_lossy(&output.stderr);
        println!("Command error: {}", stderr); // Debug info
        String::from("Unknown")
    }
}

/// Function to set the graphics mode using `pkexec envycontrol -s <mode>`
fn set_graphics_mode(mode: &str) {
    println!("Setting graphics mode to {}...", mode);
    let output = Command::new("pkexec")
        .arg("envycontrol")
        .arg("-s")
        .arg(mode)
        .output()
        .expect("Failed to execute command");

    if output.status.success() {
        println!("Successfully set graphics mode to {}.", mode); // Debug info
    } else {
        eprintln!("Error setting graphics mode: {}", String::from_utf8_lossy(&output.stderr));
    }
}

/// Function to update the label showing the current graphics mode
fn update_graphics_mode(label: &gtk::Label) {
    let current_mode = get_current_graphics_mode();
    println!("Updating label with current graphics mode: {}", current_mode); // Debug info

    // Use markup to make the mode text bold if it's "integrated" or "hybrid"
    let markup = if current_mode == "integrated" || current_mode == "hybrid" {
        format!("<b>Your graphics mode: {}</b>", current_mode)
    } else {
        format!("Your graphics mode: {}", current_mode)
    };

    label.set_markup(&markup);
}

/// Function to show the reboot dialog
fn show_reboot_dialog(window: &gtk::Window) {
    let dialog = gtk::MessageDialog::new(
        Some(window),
        gtk::DialogFlags::MODAL,
        gtk::MessageType::Info,
        gtk::ButtonsType::None,
        "Reboot to take effect",
    );

    dialog.add_button("Reboot", gtk::ResponseType::Accept);
    dialog.add_button("Maybe Later", gtk::ResponseType::Cancel);

    dialog.connect_response(|dialog, response| {
        if response == gtk::ResponseType::Accept {
            // Perform the reboot without pkexec
            Command::new("sh")
                .arg("-c")
                .arg("reboot")
                .spawn()
                .expect("Failed to execute reboot command");
        }
        dialog.close();
    });

    dialog.show_all();
}
