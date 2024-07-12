use crate::config;
use anyhow::{anyhow, Result};
use itertools::Itertools;
use std::fmt::{self, Display, Formatter};
use std::fs::{self, DirEntry, File};
use std::io::{BufRead, BufReader, BufWriter, Write};
use std::path::{Path, PathBuf};
use std::{collections::HashSet, vec::IntoIter};

pub fn get_active_roles() -> HashSet<String> {
	let Ok(file) = File::open(config::get_active_roles_file()) else {
		return HashSet::default();
	};

	BufReader::new(file)
		.lines()
		.map_while(Result::ok)
		.collect::<HashSet<String>>()
}

pub fn set_active_roles(roles: &HashSet<String>) -> Result<()> {
	fn inner(roles: &HashSet<String>) -> Result<()> {
		let file = File::create(config::get_active_roles_file())?;
		let mut writer = BufWriter::new(file);

		for role in roles.iter().sorted() {
			writeln!(writer, "{}", &role)?;
		}

		Ok(())
	}

	inner(roles).map_err(|err| anyhow!("Failed to update active roles: {err}"))
}

pub fn get_roles() -> IntoIter<Role> {
	get_plays()
		.into_group_map_by(|p| p.play_name.clone())
		.into_iter()
		.map(get_role)
		.sorted_by(|a, b| str::cmp(&a.name, &b.name))
}

fn get_role((name, plays): (String, Vec<Play>)) -> Role {
	let description = plays.first().expect("cannot pass empty group").description.clone();

	Role { name, description }
}

/// Returns the full path of available role files in alphabetical order.
pub fn get_plays() -> IntoIter<Play> {
	let roles_dir = config::get_roles_dir();

	let Ok(entries) = fs::read_dir(roles_dir) else {
		return IntoIter::default();
	};

	entries
		.filter_map(Result::ok)
		.filter(is_yml_file)
		.filter_map(|p| Play::new(p.path()))
		.sorted_by(|a, b| PathBuf::cmp(&a.path, &b.path))
}

pub fn get_plays_for_roles(plays: IntoIter<Play>, roles: &HashSet<String>) -> Vec<Play> {
	plays.filter(|r| roles.contains(&r.play_name)).collect_vec()
}

pub fn filter_invalid_roles(all_plays: &[Play], roles: Vec<String>, warn: bool) -> Vec<String> {
	let valid_roles = all_plays.iter().map(|a| &a.play_name).collect::<HashSet<&String>>();

	let (valid, invalid): (Vec<_>, Vec<_>) = roles.into_iter().partition(|r| valid_roles.contains(r));

	if warn {
		for invalid_role in invalid {
			log::warn!(r#"Skipping unknown role "{}""#, invalid_role);
		}
	}

	valid
}

pub fn filter_active_roles(active_roles: &HashSet<String>, roles: Vec<String>, warn: bool) -> Vec<String> {
	let (active, inactive): (Vec<_>, Vec<_>) = roles.into_iter().partition(|r| active_roles.contains(r));

	if warn {
		for active_role in active {
			log::warn!(r#"Skipping active role "{}""#, active_role);
		}
	}

	inactive
}

fn is_yml_file(entry: &DirEntry) -> bool {
	let path = entry.path();

	path.is_file() && path.extension().map_or(false, |e| e == "yml")
}

fn get_play_name(path: &Path) -> Option<&str> {
	let stem = path.file_stem()?.to_str()?;

	Some(stem.split_once('-').map(|(_, rest)| rest).unwrap_or(stem))
}

fn get_play_description(path: impl AsRef<Path>) -> Option<String> {
	let file = File::open(path).ok()?;
	let lines = BufReader::new(file).lines().take(2).filter_map(Result::ok);

	for line in lines {
		if line.starts_with('#') {
			return Some(line.trim_start_matches('#').trim().to_owned());
		}
	}

	None
}

pub struct Role {
	pub name: String,
	pub description: Option<String>,
}

impl Role {
	pub fn display(&self, padding: usize) -> RoleDisplay<'_> {
		RoleDisplay { role: self, padding }
	}
}

pub struct Play {
	pub path: PathBuf,
	pub play_name: String,
	pub description: Option<String>,
}

impl Play {
	fn new(path: PathBuf) -> Option<Self> {
		Some(Self {
			play_name: get_play_name(&path)?.to_owned(),
			description: get_play_description(&path),
			path,
		})
	}
}

pub struct RoleDisplay<'a> {
	pub role: &'a Role,
	pub padding: usize,
}

impl Display for RoleDisplay<'_> {
	fn fmt(&self, f: &mut Formatter<'_>) -> fmt::Result {
		write!(
			f,
			"{:<padding$}  {}",
			&self.role.name,
			&self.role.description.as_deref().unwrap_or_default(),
			padding = self.padding
		)
	}
}
