//! Mobile entry points for Android and iOS packaging (`libumuryango_mobile.so`).

#[cfg(target_os = "android")]
extern crate gpui_mobile;

// Re-export JNI window insets symbols so the cdylib linker retains them.
pub use umuryango_app::safe_area::{
    Java_dev_gpui_mobile_GpuiActivity_nativeSetInsets,
    Java_dev_gpui_mobile_MainActivity_nativeSetInsets,
    Java_dev_gpui_mobile_example_MainActivity_nativeSetInsets,
};

#[cfg(target_os = "android")]
use std::path::{Path, PathBuf};
#[cfg(target_os = "android")]
use std::sync::Arc;

#[cfg(target_os = "android")]
use gpui_kit::*;
#[cfg(target_os = "android")]
use gpui_mobile::android::jni as gpui_jni;
#[cfg(target_os = "android")]
use umuryango_app::{
    app::UmuryangoApp,
    db, preferences, theme,
    updater::{self, PlatformUpdateInstaller, UpdateApplyOutcome},
};

#[cfg(target_os = "android")]
struct AndroidUpdateInstaller {
    app: android_activity::AndroidApp,
}

#[cfg(target_os = "android")]
impl AndroidUpdateInstaller {
    fn with_env<T>(
        &self,
        operation: impl FnOnce(
            &mut jni::JNIEnv<'_>,
            jni::objects::JObject<'_>,
        ) -> jni::errors::Result<T>,
    ) -> Result<T, String> {
        // SAFETY: AndroidActivity owns the VM for the process and keeps it alive
        // for the lifetime of this registered service.
        let vm = unsafe { jni::JavaVM::from_raw(self.app.vm_as_ptr().cast()) }
            .map_err(|error| error.to_string())?;
        let mut env = vm
            .attach_current_thread()
            .map_err(|error| error.to_string())?;
        // SAFETY: AndroidActivity supplies this live Activity reference, used only
        // for the duration of the JNI call.
        let activity =
            unsafe { jni::objects::JObject::from_raw(self.app.activity_as_ptr().cast()) };
        operation(&mut env, activity).map_err(|error| error.to_string())
    }
}

#[cfg(target_os = "android")]
impl PlatformUpdateInstaller for AndroidUpdateInstaller {
    fn update_cache_dir(&self) -> Result<PathBuf, String> {
        self.with_env(|env, activity| {
            let value =
                env.call_method(activity, "updateCachePath", "()Ljava/lang/String;", &[])?;
            let object = value.l()?;
            let string = jni::objects::JString::from(object);
            let path = env.get_string(&string)?.to_string_lossy().into_owned();
            Ok(PathBuf::from(path))
        })
    }

    fn install_package(&self, package_path: &Path) -> Result<UpdateApplyOutcome, String> {
        let path_str = package_path.to_string_lossy().into_owned();
        self.with_env(|env, activity| {
            let jpath = env.new_string(path_str)?;
            let code = env
                .call_method(
                    activity,
                    "requestInstallUpdate",
                    "(Ljava/lang/String;)I",
                    &[jni::objects::JValue::Object(&jpath)],
                )?
                .i()?;
            Ok(match code {
                0 => UpdateApplyOutcome::AndroidInstallerLaunched,
                1 => UpdateApplyOutcome::AndroidPermissionRequired,
                _ => {
                    UpdateApplyOutcome::OpenedExternal(package_path.to_string_lossy().into_owned())
                }
            })
        })
    }

    fn open_external_url(&self, url: &str) -> Result<(), String> {
        let url_owned = url.to_owned();
        self.with_env(|env, activity| {
            let jurl = env.new_string(url_owned)?;
            env.call_method(
                activity,
                "openExternalUrl",
                "(Ljava/lang/String;)V",
                &[jni::objects::JValue::Object(&jurl)],
            )?;
            Ok(())
        })
    }
}

#[cfg(target_os = "android")]
#[no_mangle]
fn android_main(app: android_activity::AndroidApp) {
    android_logger::init_once(
        android_logger::Config::default()
            .with_max_level(log::LevelFilter::Info)
            .with_tag("umuryango"),
    );

    gpui_jni::install_panic_hook();
    log::info!("android_main: entered Umuryango mobile runtime");

    let _ =
        updater::register_platform_installer(Arc::new(AndroidUpdateInstaller { app: app.clone() }));

    let data_dir = app
        .internal_data_path()
        .unwrap_or_else(|| PathBuf::from("/data/data/rw.umuryango.app/files"));
    let _ = std::fs::create_dir_all(&data_dir);
    let db_path = data_dir.join("umuryango.db");
    let db_path_str = db_path.to_string_lossy().into_owned();

    let connection = db::open_local(&db_path_str)
        .or_else(|_| db::open_local(":memory:"))
        .expect("local SQLite database must initialize");

    let _platform = gpui_jni::init_platform(&app);
    let shared = match gpui_jni::shared_platform() {
        Some(s) => s,
        None => {
            log::error!("android_main: shared_platform() returned None");
            return;
        }
    };

    Application::with_platform(shared.into_rc())
        .with_assets(gpui_kit::assets::Assets)
        .run(move |cx: &mut App| {
            init(cx);
            let prefs = preferences::load(&connection);
            theme::set_style_tokens(prefs.style, prefs.theme, prefs.mood, cx);
            open_window(
                WindowOptions {
                    window_bounds: None,
                    ..Default::default()
                },
                cx,
                move |window, cx| cx.new(|cx| UmuryangoApp::new(connection, window, cx)),
            )
            .expect("mobile window must open");
        });
}
