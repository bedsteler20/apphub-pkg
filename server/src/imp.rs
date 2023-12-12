use libflatpak::gio::Cancellable;
use libflatpak::{prelude::*, glib};
use types::AppInfo;


pub fn get_app_info(app_id: &str) -> Result<Option<AppInfo>, glib::Error> {
    let u_install = libflatpak::Installation::new_user(Cancellable::NONE)?;
    let s_install = libflatpak::Installation::new_system(Cancellable::NONE)?;

    for i in vec![u_install, s_install] {
        let refs = i.list_installed_refs(Cancellable::NONE)?;
        for r in refs {
            if r.name().unwrap() == app_id {
                return Ok(Some(r.into()));
            }
        }
    }
    Ok(None)
}

pub fn is_app_installed(app_id: &str) -> Result<bool, glib::Error> {
    match get_app_info(app_id) {
        Ok(Some(_)) => Ok(true),
        Ok(None) => Ok(false),
        Err(e) => Err(e), 
    }
} 


