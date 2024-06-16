#!/bin/bash

# Build the Rust application
cargo build --bin envycontrol-gtk

# Check if cargo build was successful
if [ $? -ne 0 ]; then
    echo "Build failed. Exiting."
    exit 1
fi

# Navigate to the directory where the executable is located
cd target/debug || exit

# Move the executable to /bin (requires sudo)
sudo mv envycontrol-gtk /bin/

# Go back to the main directory
cd ../../

# Copy the icons to /usr/share/envycontrol-gtk/icons/ (requires sudo)
sudo mkdir -p /usr/share/envycontrol-gtk/icons/
sudo cp icons/logo.png /usr/share/envycontrol-gtk/icons/logo.png
sudo cp icons/envy-banner.png /usr/share/envycontrol-gtk/icons/envy-banner.png

# Create the .desktop file in ~/.local/share/applications/
cat << EOF > ~/.local/share/applications/Envycontrol-Gtk.desktop
[Desktop Entry]
Name=Envycontrol-Gtk
Comment=Graphics Mode Controller
Exec=/bin/envycontrol-gtk
Icon=/usr/share/envycontrol-gtk/icons/logo.png
Terminal=false
Type=Application
Categories=GTK;Utility;
StartupWMClass=Envycontrol-Gtk
EOF

# Make the .desktop file executable (optional)
chmod +x ~/.local/share/applications/Envycontrol-Gtk.desktop

echo "Envycontrol-Gtk has been installed successfully!"
