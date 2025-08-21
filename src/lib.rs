use tauri::{
    plugin::{Builder, TauriPlugin},
    Manager, Runtime,
};

pub use models::*;

#[cfg(desktop)]
mod desktop;
#[cfg(mobile)]
mod mobile;

mod commands;
mod error;
mod models;

pub use error::{Error, Result};

#[cfg(desktop)]
use desktop::Notifications;
#[cfg(mobile)]
use mobile::Notifications;

/// Extensions to [`tauri::App`], [`tauri::AppHandle`] and [`tauri::Window`] to access the push notifications APIs.
/// Basic notification functionality is available through the official tauri-plugin-notification.
pub trait NotificationsExt<R: Runtime> {
    fn notifications(&self) -> &Notifications<R>;
}

impl<R: Runtime, T: Manager<R>> crate::NotificationsExt<R> for T {
    fn notifications(&self) -> &Notifications<R> {
        self.state::<Notifications<R>>().inner()
    }
}

/// Initializes the push notifications plugin.
/// This plugin provides push notification capabilities.
///
/// Note: This plugin no longer re-exports tauri-plugin-notification. If you need basic
/// notification functionality, you must initialize tauri-plugin-notification separately:
/// ```rust
/// tauri::Builder::default()
///     .plugin(tauri_plugin_notification::init())
///     .plugin(tauri_plugin_push_notification::init())
///     // ...
/// ```
pub fn init<R: Runtime>() -> TauriPlugin<R> {
    Builder::new("push-notification")
        .invoke_handler(tauri::generate_handler![
            commands::check_registration_status,
            commands::register_for_remote_notifications,
            commands::watch_notifications,
        ])
        .setup(|app, api| {
            #[cfg(mobile)]
            let notifications = mobile::init(app, api)?;
            #[cfg(desktop)]
            let notifications = desktop::init(app, api)?;
            app.manage(notifications);
            Ok(())
        })
        .build()
}
