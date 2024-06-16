#!/bin/bash

# Remove the executable from /bin (requires sudo)
sudo rm /bin/envycontrol-gtk

# Remove the icons directory and its contents (requires sudo)
sudo rm -rf /usr/share/envycontrol-gtk

# Remove the .desktop file
rm ~/.local/share/applications/Envycontrol-Gtk.desktop

echo "Envycontrol-Gtk has been uninstalled successfully!"

