use anyhow::Result;
use serde::Serialize;
use std::path::PathBuf;
use std::process::Command;

#[derive(Debug, clap::Subcommand)]
pub enum DebugCmd {
    /// Emit debugging info, useful for requesting support
    Info,
}

impl DebugCmd {
    pub fn offline(&self) -> bool {
        true
    }

    pub fn exec(&self, data_dir: PathBuf) -> Result<()> {
        match self {
            DebugCmd::Info => {
                let debug_info = DebugInfo::new(data_dir);
                // Using derived serialization as a cheap Display impl for formatting
                // the output. It's human-readable enough when pretty, plus we can parse it.
                let d = serde_json::to_string_pretty(&debug_info)?;
                println!("{d}");
                Ok(())
            }
        }
    }
}

/// Represents a fact sheet about system status, bottling up
/// common support-related questions like "What chain are you on?"
/// or "What version of Tendermint are you running?". Intended to display
/// output via `fcli debug info`, for ease of pasting into chat or issues.
// The DebugInfo struct is only used to print info to stdout,
// so its field names won't be accessed elsewhere; thus allow dead_code.
#[allow(dead_code)]
#[derive(Debug, Serialize)]
pub struct DebugInfo {
    /// CometBFT version, if cometbft is found on PATH.
    cometbft_version: Option<String>,
    /// Tendermint version, if tendermint is found on PATH.
    // Preserved for a while during Tendermint -> CometBFT,
    // to aid in debugging.
    tendermint_version: Option<String>,
    /// fnsd version, if fnsd is found on PATH.
    fnsd_version: Option<String>,
    /// fcli version; baked in at compile time, so will always be present.
    fcli_version: String,
    /// Platform and architecture info for current host.
    uname: Option<String>,
    /// Status of directory for storing view info locally.
    fcli_data_directory: Option<std::path::PathBuf>,
    /// Status of fcli config TOML, containing key material for fcli.
    fcli_config_file: Option<std::path::PathBuf>,
}

impl DebugInfo {
    pub fn new(data_dir: std::path::PathBuf) -> Self {
        let dd = Self::get_fcli_data_directory(data_dir);
        Self {
            cometbft_version: Self::get_cometbft_version(),
            tendermint_version: Self::get_tendermint_version(),
            fnsd_version: Self::get_fnsd_version(),
            fcli_version: Self::get_fcli_version(),
            uname: Self::get_uname(),
            fcli_data_directory: dd.clone(),
            fcli_config_file: Self::get_fcli_config_file(dd),
        }
    }
    /// Attempt to retrieve version info for Tendermint by running
    /// `tendermint version`. Depending on deployment, tendermint may not be on the PATH;
    /// it may be in container context that `fcli` doesn't have access to. That's OK:
    /// we'll just report `None` in that case.
    fn get_tendermint_version() -> Option<String> {
        let cmd = Command::new("tendermint").args(["version"]).output();
        match cmd {
            Ok(c) => match std::str::from_utf8(&c.stdout) {
                Ok(o) => Some(o.trim_end().to_string()),
                Err(_) => None,
            },
            Err(_) => None,
        }
    }
    /// Attempt to retrieve version info for CometBFT by running
    /// `cometbft version`. Depending on deployment, cometbft may not be on the PATH;
    /// it may be in container context that `fcli` doesn't have access to. That's OK:
    /// we'll just report `None` in that case.
    fn get_cometbft_version() -> Option<String> {
        let cmd = Command::new("cometbft").args(["version"]).output();
        match cmd {
            Ok(c) => match std::str::from_utf8(&c.stdout) {
                Ok(o) => Some(o.trim_end().to_string()),
                Err(_) => None,
            },
            Err(_) => None,
        }
    }
    /// Return host info, including kernel and architecture. Should work
    /// equally well on Linux or macOS; Windows will return None.
    fn get_uname() -> Option<String> {
        let cmd = Command::new("uname").args(["-a"]).output();
        match cmd {
            Ok(c) => match std::str::from_utf8(&c.stdout) {
                Ok(o) => Some(o.trim_end().to_string()),
                Err(_) => None,
            },
            Err(_) => None,
        }
    }
    /// Return the version for `fcli` baked in at compile time.
    fn get_fcli_version() -> String {
        env!("CARGO_PKG_VERSION").to_string()
    }
    /// Attempt to find `fnsd` on PATH, and return its version number. Depending on deployment,
    /// `fnsd` may not be on the path; it may in a container context elsewhere.
    fn get_fnsd_version() -> Option<String> {
        match Command::new("fnsd").args(["--version"]).output() {
            Ok(c) => match std::str::from_utf8(&c.stdout) {
                Ok(o) => Some(o.trim_end().to_string()),
                Err(_) => None,
            },
            Err(_) => None,
        }
    }

    /// Check whether data dir, as provided by arg-parsing, exists.
    fn get_fcli_data_directory(data_dir: PathBuf) -> Option<PathBuf> {
        match data_dir.exists() {
            true => Some(data_dir),
            false => None,
        }
    }
    /// Check fcli config TOML file exists.
    fn get_fcli_config_file(data_dir: Option<PathBuf>) -> Option<PathBuf> {
        match data_dir {
            Some(dd) => {
                let mut k = dd;
                k.push("config.toml");
                if k.exists() {
                    Some(k)
                } else {
                    None
                }
            }
            None => None,
        }
    }
}
