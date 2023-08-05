use std::{
	env::{self, consts::ARCH},
	path::PathBuf,
};
use sys_info::linux_os_release;

const APP_NAME: &str = env!("CARGO_PKG_NAME");
const ENV_ROLR_CONFIG_PATH: &str = "ROLR_CONFIG_PATH";
const ENV_ROLR_DISTRO: &str = "ROLR_DISTRO";

pub fn get_active_roles_file() -> PathBuf {
	get_config_dir().join("active.txt")
}

pub fn get_roles_dir() -> PathBuf {
	get_config_dir().join("roles").join(ARCH).join(get_os_release_id())
}

pub fn get_config_dir() -> PathBuf {
	if let Ok(path) = env::var(ENV_ROLR_CONFIG_PATH) {
		return path.into();
	}

	get_xdg_config_home().join(APP_NAME)
}

pub fn get_xdg_config_home() -> PathBuf {
	if let Ok(path) = env::var("XDG_CONFIG_HOME") {
		return path.into();
	}

	let home = env::var("HOME").expect("$HOME is undefined");

	PathBuf::from(home).join(".config")
}

pub fn get_os_release_id() -> String {
	if let Ok(distro) = env::var(ENV_ROLR_DISTRO) {
		return distro;
	}

	let os_release = linux_os_release().expect("os-release not found");
	os_release.id.expect("os-release id not found")
}
