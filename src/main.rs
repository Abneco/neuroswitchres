// Copyright (c) 2023 Sylvain Bernard
// Ce fichier fait partie de neuro-switchres, qui est distribué sous la licence GPLv3.
// Voir le fichier LICENSE.txt pour plus de détails.

#![windows_subsystem = "windows"]
extern crate winapi;

use std::ptr::null_mut;
use winapi::um::wingdi::{DEVMODEW, DISPLAY_DEVICEW};
use winapi::um::winuser::{CDS_UPDATEREGISTRY, ChangeDisplaySettingsExW, ENUM_CURRENT_SETTINGS, EnumDisplayDevicesW, EnumDisplaySettingsW};
use systray::Application;

fn set_resolution(width: u32, height: u32) {
    let mut device_number = 0;

    unsafe {
        loop {
            let mut display_device: DISPLAY_DEVICEW = std::mem::zeroed();
            display_device.cb = std::mem::size_of::<DISPLAY_DEVICEW>() as u32;

            if { EnumDisplayDevicesW(null_mut(), device_number, &mut display_device, 0) } == 0 {
                // Plus d'écrans à énumérer
                break;
            }

            let mut devmode: DEVMODEW = std::mem::zeroed();
            devmode.dmSize = std::mem::size_of::<DEVMODEW>() as u16;

            // Obtenir les paramètres actuels pour cet écran
            if { EnumDisplaySettingsW(display_device.DeviceName.as_ptr(), ENUM_CURRENT_SETTINGS, &mut devmode) } != 0 {
                devmode.dmPelsWidth = width;
                devmode.dmPelsHeight = height;
                devmode.dmFields = 0x00180000; // DM_PELSWIDTH | DM_PELSHEIGHT

                // Appliquer les nouveaux paramètres
                let result = { ChangeDisplaySettingsExW(display_device.DeviceName.as_ptr(), &mut devmode, null_mut(), CDS_UPDATEREGISTRY, null_mut()) };

                if result != 0 {
                    println!("Erreur lors du changement de résolution pour l'écran {}: {}", device_number, result);
                }
            } else {
                println!("Impossible d'obtenir les paramètres d'affichage actuels pour l'écran {}.", device_number);
            }

            device_number += 1;
        }
    }
}

fn main() -> Result<(), Box<dyn std::error::Error>> {
    let mut app = Application::new()?;
    app.set_tooltip("NEURO-SWITCHRES")?;
    app.set_icon_from_file("resources/neuroswitchres-tray.ico")?; // copier le fichier dans le répertoire target

    app.add_menu_item("Résolution 2160p - 4K", |_| {
        set_resolution(3840, 2160);
        Ok::<_, systray::Error>(())
    })?;

    app.add_menu_item("Résolution 1440p - 2K", |_| {
        set_resolution(2560, 1440);
        Ok::<_, systray::Error>(())
    })?;

    app.add_menu_item("Résolution 1080p - FULL HD", |_| {
        set_resolution(1920, 1080);
        Ok::<_, systray::Error>(())
    })?;

    app.add_menu_separator()?;

    app.add_menu_item("Quitter", |window| {
        window.quit();
        Ok::<_, systray::Error>(())
    })?;

    app.wait_for_message()?;
    Ok(())
}
