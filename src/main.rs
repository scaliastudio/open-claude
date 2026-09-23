//! Open Claude. It opens Claude.
//!
//! Security model: the only two destinations are the constants below. The program reads no
//! arguments, no environment, no files and no network, and exits as soon as it has asked the
//! operating system to open one of them.

#![windows_subsystem = "windows"]

/// Claude Desktop's registered link scheme. Opening it hands off to whatever the user installed.
const DESKTOP: &str = "claude://";
const WEB: &str = "https://claude.ai";

fn main() {
    if platform::desktop_installed() && platform::open(DESKTOP) {
        return;
    }
    platform::open(WEB);
}

#[cfg(windows)]
mod platform {
    use std::ptr::{null, null_mut};
    use windows_sys::Win32::Foundation::ERROR_SUCCESS;
    use windows_sys::Win32::System::Registry::{
        RegCloseKey, RegOpenKeyExW, RegQueryValueExW, HKEY, HKEY_CURRENT_USER, HKEY_LOCAL_MACHINE, KEY_READ,
    };
    use windows_sys::Win32::UI::Shell::ShellExecuteW;
    use windows_sys::Win32::UI::WindowsAndMessaging::SW_SHOWNORMAL;

    fn wide(s: &str) -> Vec<u16> {
        s.encode_utf16().chain(Some(0)).collect()
    }

    /// True when some app has registered the `claude:` URL protocol for this user or machine.
    pub fn desktop_installed() -> bool {
        let path = wide(r"Software\Classes\claude");
        let marker = wide("URL Protocol");
        [HKEY_CURRENT_USER, HKEY_LOCAL_MACHINE].into_iter().any(|root| {
            let mut key: HKEY = null_mut();
            // SAFETY: both strings are NUL-terminated and outlive the calls; `key` is closed below.
            unsafe {
                if RegOpenKeyExW(root, path.as_ptr(), 0, KEY_READ, &mut key) != ERROR_SUCCESS {
                    return false;
                }
                let found = RegQueryValueExW(key, marker.as_ptr(), null(), null_mut(), null_mut(), null_mut()) == ERROR_SUCCESS;
                RegCloseKey(key);
                found
            }
        })
    }

    pub fn open(target: &str) -> bool {
        let verb = wide("open");
        let file = wide(target);
        // SAFETY: NUL-terminated strings that outlive the call; ShellExecuteW returns a value > 32 on success.
        let result = unsafe { ShellExecuteW(null_mut(), verb.as_ptr(), file.as_ptr(), null(), null(), SW_SHOWNORMAL) };
        result as isize > 32
    }
}

#[cfg(not(windows))]
mod platform {
    // Released builds are Windows-only; Mac users get the web app. This keeps local builds working.
    pub fn desktop_installed() -> bool {
        cfg!(target_os = "macos") && std::path::Path::new("/Applications/Claude.app").exists()
    }

    pub fn open(target: &str) -> bool {
        let opener = if cfg!(target_os = "macos") { "/usr/bin/open" } else { "xdg-open" };
        std::process::Command::new(opener).arg(target).status().map(|s| s.success()).unwrap_or(false)
    }
}
