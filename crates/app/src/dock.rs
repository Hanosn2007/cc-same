//! macOS Dock visibility. Activation policy leaves windows and the status item intact.

#[cfg(target_os = "macos")]
pub fn set_visible(visible: bool) -> bool {
    use objc2::MainThreadMarker;
    use objc2_app_kit::{NSApplication, NSApplicationActivationPolicy};
    let Some(main) = MainThreadMarker::new() else { return false };
    let app = NSApplication::sharedApplication(main);
    let policy =
        if visible { NSApplicationActivationPolicy::Regular } else { NSApplicationActivationPolicy::Accessory };
    app.activationPolicy() == policy || app.setActivationPolicy(policy)
}

#[cfg(not(target_os = "macos"))]
pub fn set_visible(_visible: bool) -> bool {
    true
}
