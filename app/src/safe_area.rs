use std::sync::atomic::{AtomicBool, Ordering};
use std::sync::Mutex;

/// Safe area insets in physical (device) pixels.
#[derive(Debug, Clone, Copy, PartialEq, Default)]
pub struct SafeArea {
    pub left: f32,
    pub top: f32,
    pub right: f32,
    pub bottom: f32,
}

static SAFE_AREA: Mutex<SafeArea> = Mutex::new(SafeArea {
    left: 0.0,
    top: 0.0,
    right: 0.0,
    bottom: 0.0,
});

static INSETS_CHANGED: AtomicBool = AtomicBool::new(false);

/// Returns the current safe area insets in physical pixels.
pub fn get_safe_area() -> SafeArea {
    SAFE_AREA.lock().map(|s| *s).unwrap_or_default()
}

/// Sets the current safe area insets in physical pixels.
pub fn set_safe_area(left: f32, top: f32, right: f32, bottom: f32) {
    if let Ok(mut lock) = SAFE_AREA.lock() {
        *lock = SafeArea {
            left,
            top,
            right,
            bottom,
        };
    }
    INSETS_CHANGED.store(true, Ordering::SeqCst);
}

/// Checks and clears the insets changed flag.
pub fn take_insets_changed() -> bool {
    INSETS_CHANGED.swap(false, Ordering::SeqCst)
}

// ── JNI Exports for Android WindowInsets ─────────────────────────────────────
//
// These native methods are called from Java in `GpuiActivity` when
// ViewCompat.setOnApplyWindowInsetsListener fires on the Android UI thread.

#[no_mangle]
pub extern "system" fn Java_dev_gpui_mobile_GpuiActivity_nativeSetInsets(
    _env: *mut std::ffi::c_void,
    _class: *mut std::ffi::c_void,
    left: i32,
    top: i32,
    right: i32,
    bottom: i32,
) {
    set_safe_area(left as f32, top as f32, right as f32, bottom as f32);
}

#[no_mangle]
pub extern "system" fn Java_dev_gpui_mobile_MainActivity_nativeSetInsets(
    env: *mut std::ffi::c_void,
    class: *mut std::ffi::c_void,
    left: i32,
    top: i32,
    right: i32,
    bottom: i32,
) {
    Java_dev_gpui_mobile_GpuiActivity_nativeSetInsets(env, class, left, top, right, bottom);
}

#[no_mangle]
pub extern "system" fn Java_dev_gpui_mobile_example_MainActivity_nativeSetInsets(
    env: *mut std::ffi::c_void,
    class: *mut std::ffi::c_void,
    left: i32,
    top: i32,
    right: i32,
    bottom: i32,
) {
    Java_dev_gpui_mobile_GpuiActivity_nativeSetInsets(env, class, left, top, right, bottom);
}

#[cfg(test)]
mod tests {
    use super::*;

    static TEST_LOCK: Mutex<()> = Mutex::new(());

    #[test]
    fn test_safe_area_set_and_get() {
        let _guard = TEST_LOCK.lock().unwrap();
        set_safe_area(12.0, 24.0, 12.0, 48.0);
        let insets = get_safe_area();
        assert_eq!(insets.left, 12.0);
        assert_eq!(insets.top, 24.0);
        assert_eq!(insets.right, 12.0);
        assert_eq!(insets.bottom, 48.0);
        assert!(take_insets_changed());
        assert!(!take_insets_changed());

        // Reset to default
        set_safe_area(0.0, 0.0, 0.0, 0.0);
        let insets = get_safe_area();
        assert_eq!(insets.bottom, 0.0);
    }

    #[test]
    fn test_jni_native_set_insets() {
        let _guard = TEST_LOCK.lock().unwrap();
        Java_dev_gpui_mobile_GpuiActivity_nativeSetInsets(
            std::ptr::null_mut(),
            std::ptr::null_mut(),
            0,
            35,
            0,
            70,
        );
        let insets = get_safe_area();
        assert_eq!(insets.top, 35.0);
        assert_eq!(insets.bottom, 70.0);
        assert!(take_insets_changed());

        // Reset
        set_safe_area(0.0, 0.0, 0.0, 0.0);
    }
}
