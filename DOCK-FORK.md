# Independent Dock and menu bar icons

This fork adds a persistent macOS **Always show Dock icon** setting beside the menu bar setting.
When it is off, the Dock icon follows the window: opening the window shows the Dock icon;
closing the last window hides it. The menu bar and independent sync agent keep running.

The default preserves the upstream behavior. If the menu bar is disabled or cannot be created,
the Dock icon is restored so the app retains an entry point. Other platforms keep their existing
activation policy. The sync engine, session data, and account switching behavior are unchanged.

Updates follow this fork's releases so an upstream update does not silently remove the setting.
The local build is ad-hoc signed; it does not carry the upstream author's Developer ID signature.

Manual acceptance: turn off Always show Dock icon while leaving the menu bar enabled. Check
that opening shows the Dock icon, closing hides it, and reopening restores it. Restart the app
and confirm that background sync stays active.
