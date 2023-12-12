use libflatpak::{gio::Cancellable, prelude::*};
use serde::{Deserialize, Serialize};

use crate::imp::get_icon_path;

#[derive(Debug, zvariant::Type, Serialize, Deserialize)]
pub struct AppInfo {
    pub app_id: String,
    pub name: String,
    pub from_flathub: bool,
    pub icon: String,
    pub install_location: InstallLocation,
    pub flatpak_ref: String,
    pub summary: String,
}

#[derive(Debug, zvariant::Type, Serialize, Deserialize, Clone, Copy)]
pub enum InstallLocation {
    System,
    User,
}

impl Into<libflatpak::Installation> for InstallLocation {
    fn into(self) -> libflatpak::Installation {
        match self {
            InstallLocation::System => libflatpak::Installation::new_system(Cancellable::NONE)
                .expect("Failed to create system installation"),
            InstallLocation::User => libflatpak::Installation::new_user(Cancellable::NONE)
                .expect("Failed to create user installation"),
        }
    }
}

impl From<libflatpak::InstalledRef> for AppInfo {
    fn from(installed_ref: libflatpak::InstalledRef) -> Self {
        let pfx = std::env::var("XDG_DATA_HOME")
            .unwrap_or(format!("{}/.local/share", std::env::var("HOME").unwrap()));

        let install = if installed_ref.deploy_dir().unwrap().starts_with(&pfx) {
            InstallLocation::User
        } else {
            InstallLocation::System
        };

        let app_id = installed_ref.name().unwrap().to_string();
        let remote = installed_ref.origin().unwrap_or("".into()).to_string();

        Self {
            icon: get_icon_path(&install.into(), &app_id, &remote).unwrap_or("".to_string()),
            app_id: app_id,
            name: installed_ref
                .appdata_name()
                .unwrap_or(installed_ref.name().unwrap())
                .to_string(),
            from_flathub: remote.starts_with("flathub"),
            flatpak_ref: installed_ref.format_ref().unwrap().to_string(),
            install_location: install,
            summary: installed_ref
                .appdata_summary()
                .unwrap_or("".into())
                .to_string(),
        }
    }
}
