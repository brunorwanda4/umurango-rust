//! Self-update manager checking GitHub Releases, downloading new platform binaries
//! (Windows `.exe` and Android `.apk`), replacing/installing the package, and restarting.

use serde::{Deserialize, Serialize};
use std::env;
use std::fs;
use std::io::Read;
use std::path::{Path, PathBuf};
use std::process::Command;
use std::sync::{Arc, OnceLock};

/// The compiled version of this application binary from `Cargo.toml`.
pub const CURRENT_VERSION: &str = env!("CARGO_PKG_VERSION");

/// Default GitHub repository (`owner/repo`) checked for releases.
pub const DEFAULT_GITHUB_REPO: &str = "brunorwanda4/umurango-rust";

/// Metadata for an available application update from GitHub Releases.
#[derive(Clone, Debug, PartialEq, Eq, Serialize, Deserialize)]
pub struct ReleaseUpdate {
    pub version: String,
    pub tag_name: String,
    pub title: String,
    pub notes: String,
    pub html_url: String,
    pub asset_name: Option<String>,
    pub download_url: Option<String>,
    pub asset_size: u64,
}

/// Current state of the self-updater in the UI.
#[derive(Clone, Debug, Default, PartialEq, Eq)]
pub enum UpdateState {
    #[default]
    Idle,
    Checking,
    UpToDate,
    Available(ReleaseUpdate),
    Downloading(ReleaseUpdate),
    Error(String),
}

/// Outcome of applying an update.
#[derive(Clone, Debug, PartialEq, Eq)]
pub enum UpdateApplyOutcome {
    /// New executable was installed and spawned; the current process should exit immediately.
    RestartLaunched,
    /// Android package installer was launched for the downloaded `.apk`.
    AndroidInstallerLaunched,
    /// Android opened system settings to grant "Install unknown apps" permission.
    AndroidPermissionRequired,
    /// Opened external browser / installer (e.g. when no direct binary asset was found).
    OpenedExternal(String),
}

/// Platform-specific update installer hook (used on Android to download `.apk`
/// into the app's `FileProvider` cache and launch the system package installer).
pub trait PlatformUpdateInstaller: Send + Sync {
    /// Returns the directory where downloaded update packages (e.g. `.apk`) should be stored.
    fn update_cache_dir(&self) -> Result<PathBuf, String>;
    /// Triggers the platform package installer for the downloaded package file.
    fn install_package(&self, package_path: &Path) -> Result<UpdateApplyOutcome, String>;
    /// Opens an external URL in the platform browser.
    fn open_external_url(&self, url: &str) -> Result<(), String>;
}

static PLATFORM_INSTALLER: OnceLock<Arc<dyn PlatformUpdateInstaller>> = OnceLock::new();

/// Registers a platform-specific update installer (called at startup by the Android host).
pub fn register_platform_installer(installer: Arc<dyn PlatformUpdateInstaller>) -> bool {
    PLATFORM_INSTALLER.set(installer).is_ok()
}

#[derive(Debug, Deserialize)]
struct GithubReleaseDto {
    tag_name: String,
    #[serde(default)]
    name: Option<String>,
    #[serde(default)]
    body: Option<String>,
    html_url: String,
    #[serde(default)]
    draft: bool,
    #[serde(default)]
    prerelease: bool,
    #[serde(default)]
    assets: Vec<GithubAssetDto>,
}

#[derive(Clone, Debug, Deserialize, PartialEq, Eq)]
pub struct GithubAssetDto {
    pub name: String,
    pub browser_download_url: String,
    #[serde(default)]
    pub size: u64,
}

/// Resolves the GitHub repository (`owner/repo`) for update checks.
pub fn update_repo() -> String {
    env::var("UMURYANGO_UPDATE_REPO")
        .ok()
        .map(|s| s.trim().to_string())
        .filter(|s| !s.is_empty())
        .unwrap_or_else(|| DEFAULT_GITHUB_REPO.to_string())
}

/// Resolves the GitHub Releases latest API endpoint URL.
pub fn latest_release_api_url() -> String {
    if let Ok(custom_url) = env::var("UMURYANGO_UPDATE_URL") {
        let trimmed = custom_url.trim();
        if !trimmed.is_empty() {
            return trimmed.to_string();
        }
    }
    format!(
        "https://api.github.com/repos/{}/releases/latest",
        update_repo()
    )
}

/// Parses a semantic version string (e.g. `"v0.2.1"`, `"0.2.1-beta.1"`) into `(major, minor, patch)`.
pub fn parse_semver(version: &str) -> Option<(u64, u64, u64)> {
    let trimmed = version
        .trim()
        .trim_start_matches('v')
        .trim_start_matches('V');
    let core = trimmed.split(['-', '+']).next()?.trim();
    let mut parts = core.split('.');
    let major = parts.next()?.parse::<u64>().ok()?;
    let minor = parts.next().unwrap_or("0").parse::<u64>().ok()?;
    let patch = parts.next().unwrap_or("0").parse::<u64>().ok()?;
    if parts.next().is_some() {
        return None;
    }
    Some((major, minor, patch))
}

/// Normalizes a version tag by removing a leading `v` or `V`.
pub fn normalize_version(tag: &str) -> String {
    tag.trim()
        .trim_start_matches('v')
        .trim_start_matches('V')
        .to_string()
}

/// Returns `true` if `candidate` is strictly newer than `current`.
pub fn is_newer_version(current: &str, candidate: &str) -> bool {
    match (parse_semver(current), parse_semver(candidate)) {
        (Some(curr), Some(cand)) => cand > curr,
        _ => false,
    }
}

/// Selects the best matching release asset for the given target OS and CPU architecture.
pub fn select_platform_asset<'a>(
    assets: &'a [GithubAssetDto],
    target_os: &str,
    target_arch: &str,
) -> Option<&'a GithubAssetDto> {
    if assets.is_empty() {
        return None;
    }

    let os_lower = target_os.to_lowercase();
    let arch_lower = target_arch.to_lowercase();

    // Filter out checksum / signature sidecar files
    let candidates: Vec<&GithubAssetDto> = assets
        .iter()
        .filter(|a| {
            let n = a.name.to_lowercase();
            !n.ends_with(".sha256")
                && !n.ends_with(".sig")
                && !n.ends_with(".txt")
                && !n.ends_with(".pdb")
        })
        .collect();

    if os_lower == "windows" {
        // Prefer exact arch .exe match first, then any .exe
        if let Some(exact) = candidates.iter().copied().find(|a| {
            let n = a.name.to_lowercase();
            n.ends_with(".exe") && (n.contains(&arch_lower) || n.contains("windows"))
        }) {
            return Some(exact);
        }
        if let Some(any_exe) = candidates
            .iter()
            .copied()
            .find(|a| a.name.to_lowercase().ends_with(".exe"))
        {
            return Some(any_exe);
        }
    } else if os_lower == "android" {
        let is_arm64 = arch_lower == "aarch64" || arch_lower == "arm64";
        if let Some(arch_apk) = candidates.iter().copied().find(|a| {
            let n = a.name.to_lowercase();
            n.ends_with(".apk")
                && (n.contains(&arch_lower)
                    || (is_arm64 && (n.contains("arm64") || n.contains("aarch64")))
                    || n.contains("android"))
        }) {
            return Some(arch_apk);
        }
        if let Some(apk) = candidates
            .iter()
            .copied()
            .find(|a| a.name.to_lowercase().ends_with(".apk"))
        {
            return Some(apk);
        }
    } else if os_lower == "macos" {
        if let Some(mac) = candidates.iter().copied().find(|a| {
            let n = a.name.to_lowercase();
            (n.contains("macos") || n.contains("darwin") || n.contains("apple"))
                && (n.contains(&arch_lower) || !n.ends_with(".exe"))
        }) {
            return Some(mac);
        }
    } else if os_lower == "linux" {
        if let Some(lin) = candidates.iter().copied().find(|a| {
            let n = a.name.to_lowercase();
            n.contains("linux") && !n.ends_with(".exe") && !n.ends_with(".apk")
        }) {
            return Some(lin);
        }
    }

    None
}

/// Removes any leftover `.old` executable created during a previous self-update on Windows.
pub fn cleanup_old_binary() {
    if let Ok(current_exe) = env::current_exe() {
        let old_exe = backup_exe_path(&current_exe);
        if old_exe.exists() {
            let _ = fs::remove_file(&old_exe);
        }
        let tmp_exe = staging_exe_path(&current_exe);
        if tmp_exe.exists() {
            let _ = fs::remove_file(&tmp_exe);
        }
    }
}

fn backup_exe_path(current_exe: &Path) -> PathBuf {
    let mut os_str = current_exe.as_os_str().to_os_string();
    os_str.push(".old");
    PathBuf::from(os_str)
}

fn staging_exe_path(current_exe: &Path) -> PathBuf {
    let mut os_str = current_exe.as_os_str().to_os_string();
    os_str.push(".new.tmp");
    PathBuf::from(os_str)
}

/// Queries GitHub Releases for the latest release and checks if it is newer than `CURRENT_VERSION`.
pub fn check_for_update() -> Result<Option<ReleaseUpdate>, String> {
    let url = latest_release_api_url();
    let user_agent = format!("umuryango-app/{CURRENT_VERSION}");

    let response = match ureq::get(&url)
        .set("User-Agent", &user_agent)
        .set("Accept", "application/vnd.github+json")
        .call()
    {
        Ok(resp) => resp,
        Err(ureq::Error::Status(404, _)) => {
            // No published releases in the repository yet
            return Ok(None);
        }
        Err(ureq::Error::Status(status, _)) => {
            return Err(format!("Update server returned HTTP {status}"));
        }
        Err(ureq::Error::Transport(transport)) => {
            return Err(format!(
                "Network error while checking updates: {}",
                transport.message().unwrap_or("connection failed")
            ));
        }
    };

    let release = response
        .into_json::<GithubReleaseDto>()
        .map_err(|e| format!("Failed to parse release metadata: {e}"))?;

    if release.draft || release.prerelease {
        return Ok(None);
    }

    let latest_version = normalize_version(&release.tag_name);
    if !is_newer_version(CURRENT_VERSION, &latest_version) {
        return Ok(None);
    }

    let matched_asset = select_platform_asset(&release.assets, env::consts::OS, env::consts::ARCH);

    let title = release
        .name
        .filter(|n| !n.trim().is_empty())
        .unwrap_or_else(|| format!("Umuryango v{latest_version}"));
    let notes = release
        .body
        .filter(|b| !b.trim().is_empty())
        .unwrap_or_else(|| "Performance improvements and new features.".to_string());

    Ok(Some(ReleaseUpdate {
        version: latest_version,
        tag_name: release.tag_name,
        title,
        notes,
        html_url: release.html_url,
        asset_name: matched_asset.map(|a| a.name.clone()),
        download_url: matched_asset.map(|a| a.browser_download_url.clone()),
        asset_size: matched_asset.map(|a| a.size).unwrap_or(0),
    }))
}

fn download_release_bytes(download_url: &str) -> Result<Vec<u8>, String> {
    let user_agent = format!("umuryango-app/{CURRENT_VERSION}");
    let response = ureq::get(download_url)
        .set("User-Agent", &user_agent)
        .call()
        .map_err(|e| format!("Failed to download update package: {e}"))?;

    let mut bytes = Vec::new();
    response
        .into_reader()
        .take(250 * 1024 * 1024) // 250 MB safety cap
        .read_to_end(&mut bytes)
        .map_err(|e| format!("Failed to read update stream: {e}"))?;

    if bytes.len() < 1024 {
        return Err("Downloaded update package appears incomplete or corrupted.".to_string());
    }
    Ok(bytes)
}

/// Downloads the release asset, replaces the current executable (on desktop) or triggers the
/// Android package installer (on Android), and restarts/upgrades the application.
pub fn download_and_apply_update(update: &ReleaseUpdate) -> Result<UpdateApplyOutcome, String> {
    if let Some(installer) = PLATFORM_INSTALLER.get() {
        let Some(ref download_url) = update.download_url else {
            installer.open_external_url(&update.html_url)?;
            return Ok(UpdateApplyOutcome::OpenedExternal(update.html_url.clone()));
        };

        let cache_dir = installer.update_cache_dir()?;
        fs::create_dir_all(&cache_dir)
            .map_err(|e| format!("Failed to create Android update cache folder: {e}"))?;

        let apk_filename = update
            .asset_name
            .as_deref()
            .filter(|n| n.to_lowercase().ends_with(".apk"))
            .map(ToOwned::to_owned)
            .unwrap_or_else(|| format!("umuryango-app-android-{}.apk", update.version));
        let apk_path = cache_dir.join(apk_filename);

        let bytes = download_release_bytes(download_url)?;
        fs::write(&apk_path, &bytes)
            .map_err(|e| format!("Failed to save downloaded APK update: {e}"))?;

        return installer.install_package(&apk_path);
    }

    let Some(ref download_url) = update.download_url else {
        open_url_in_browser(&update.html_url)?;
        return Ok(UpdateApplyOutcome::OpenedExternal(update.html_url.clone()));
    };

    if cfg!(target_os = "android") {
        open_url_in_browser(download_url)?;
        return Ok(UpdateApplyOutcome::OpenedExternal(download_url.clone()));
    }

    let current_exe =
        env::current_exe().map_err(|e| format!("Cannot resolve current executable path: {e}"))?;
    let staging_path = staging_exe_path(&current_exe);
    let backup_path = backup_exe_path(&current_exe);

    let bytes = download_release_bytes(download_url)?;

    fs::write(&staging_path, &bytes)
        .map_err(|e| format!("Failed to write staged update file: {e}"))?;

    #[cfg(unix)]
    {
        use std::os::unix::fs::PermissionsExt;
        if let Ok(meta) = fs::metadata(&staging_path) {
            let mut perms = meta.permissions();
            perms.set_mode(0o755);
            let _ = fs::set_permissions(&staging_path, perms);
        }
    }

    replace_executable_and_restart(&current_exe, &staging_path, &backup_path)?;
    Ok(UpdateApplyOutcome::RestartLaunched)
}

fn replace_executable_and_restart(
    current_exe: &Path,
    staging_path: &Path,
    backup_path: &Path,
) -> Result<(), String> {
    if backup_path.exists() {
        let _ = fs::remove_file(backup_path);
    }

    // Rename running executable to `<exe>.old` (permitted on Windows, Linux, and macOS),
    // then move `<exe>.new.tmp` into `<exe>`.
    if fs::rename(current_exe, backup_path).is_ok() {
        if let Err(err) = fs::rename(staging_path, current_exe) {
            // Rollback if moving staged file failed
            let _ = fs::rename(backup_path, current_exe);
            return Err(format!("Failed to activate updated executable: {err}"));
        }

        Command::new(current_exe)
            .spawn()
            .map_err(|e| format!("Updated binary installed, but failed to relaunch: {e}"))?;
        return Ok(());
    }

    // Fallback on Windows if direct rename is locked: launch a background PowerShell helper
    // that waits briefly for this process to exit, swaps the file, and restarts the app.
    #[cfg(target_os = "windows")]
    {
        let curr = current_exe.to_string_lossy().replace('\'', "''");
        let stage = staging_path.to_string_lossy().replace('\'', "''");
        let ps_script = format!(
            "Start-Sleep -Milliseconds 900; Move-Item -Force -LiteralPath '{stage}' -Destination '{curr}'; Start-Process -FilePath '{curr}'"
        );
        Command::new("powershell")
            .args([
                "-NoProfile",
                "-WindowStyle",
                "Hidden",
                "-Command",
                &ps_script,
            ])
            .spawn()
            .map_err(|e| format!("Failed to launch Windows update installer helper: {e}"))?;
        Ok(())
    }

    #[cfg(not(target_os = "windows"))]
    {
        Err("Failed to replace current executable with the downloaded update.".to_string())
    }
}

/// Opens a URL in the platform's default browser.
pub fn open_url_in_browser(url: &str) -> Result<(), String> {
    if let Some(installer) = PLATFORM_INSTALLER.get() {
        return installer.open_external_url(url);
    }

    #[cfg(target_os = "windows")]
    {
        Command::new("cmd")
            .args(["/C", "start", "", url])
            .spawn()
            .map_err(|e| format!("Failed to open browser: {e}"))?;
        Ok(())
    }
    #[cfg(target_os = "macos")]
    {
        Command::new("open")
            .arg(url)
            .spawn()
            .map_err(|e| format!("Failed to open browser: {e}"))?;
        Ok(())
    }
    #[cfg(all(unix, not(target_os = "macos")))]
    {
        Command::new("xdg-open")
            .arg(url)
            .spawn()
            .map_err(|e| format!("Failed to open browser: {e}"))?;
        Ok(())
    }
}

#[cfg(test)]
mod tests {
    use super::*;

    #[test]
    fn test_parse_semver() {
        assert_eq!(parse_semver("0.1.0"), Some((0, 1, 0)));
        assert_eq!(parse_semver("v0.2.3"), Some((0, 2, 3)));
        assert_eq!(parse_semver("V1.10.5-beta.1"), Some((1, 10, 5)));
        assert_eq!(parse_semver("2.0"), Some((2, 0, 0)));
        assert_eq!(parse_semver("invalid"), None);
        assert_eq!(parse_semver("1.2.3.4"), None);
    }

    #[test]
    fn test_is_newer_version() {
        assert!(is_newer_version("0.1.0", "0.1.1"));
        assert!(is_newer_version("0.1.0", "v0.2.0"));
        assert!(is_newer_version("v0.1.9", "v0.2.0"));
        assert!(is_newer_version("0.9.9", "1.0.0"));
        assert!(!is_newer_version("0.1.0", "0.1.0"));
        assert!(!is_newer_version("v0.2.0", "0.1.9"));
        assert!(!is_newer_version("0.1.0", "not-a-version"));
    }

    #[test]
    fn test_select_platform_asset() {
        let assets = vec![
            GithubAssetDto {
                name: "umuryango-app-windows-x86_64.exe.sha256".to_string(),
                browser_download_url: "https://example.com/win.sha256".to_string(),
                size: 64,
            },
            GithubAssetDto {
                name: "umuryango-app-windows-x86_64.exe".to_string(),
                browser_download_url: "https://example.com/win.exe".to_string(),
                size: 15_000_000,
            },
            GithubAssetDto {
                name: "umuryango-app-linux-x86_64".to_string(),
                browser_download_url: "https://example.com/linux".to_string(),
                size: 14_000_000,
            },
            GithubAssetDto {
                name: "umuryango-app-android-arm64.apk.sha256".to_string(),
                browser_download_url: "https://example.com/android.apk.sha256".to_string(),
                size: 64,
            },
            GithubAssetDto {
                name: "umuryango-app-android-arm64.apk".to_string(),
                browser_download_url: "https://example.com/android.apk".to_string(),
                size: 18_000_000,
            },
        ];

        let win = select_platform_asset(&assets, "windows", "x86_64").unwrap();
        assert_eq!(win.name, "umuryango-app-windows-x86_64.exe");

        let linux = select_platform_asset(&assets, "linux", "x86_64").unwrap();
        assert_eq!(linux.name, "umuryango-app-linux-x86_64");

        let android = select_platform_asset(&assets, "android", "aarch64").unwrap();
        assert_eq!(android.name, "umuryango-app-android-arm64.apk");
    }
}
