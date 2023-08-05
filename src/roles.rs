use itertools::Itertools;
use lazy_static::lazy_static;
use regex::Regex;
use std::{
	collections::HashSet,
	fs::{self, DirEntry, File},
	io::{BufRead, BufReader},
	path::{Path, PathBuf},
	vec::IntoIter,
};

use crate::config;

lazy_static! {
	static ref PRIORITY_REGEX: Regex = Regex::new(r"^\d+-").expect("invalid regex");
}

pub fn get_active_roles() -> HashSet<String> {
	let Ok(file) = File::open(config::get_active_roles_file()) else {
		return HashSet::default()
	};

	BufReader::new(file)
		.lines()
		.map_while(Result::ok)
		.collect::<HashSet<String>>()
}

pub fn get_roles() -> IntoIter<Role> {
	get_plays()
		.into_group_map_by(|p| get_play_name(&p.path))
		.into_iter()
		.filter_map(get_role)
		.sorted_by(|a, b| str::cmp(&a.name, &b.name))
}

fn get_role(pair: (Option<String>, Vec<Play>)) -> Option<Role> {
	let (Some(name), plays) = pair else {
		return None;
	};

	Some(Role {
		description: get_play_description(&plays.first().unwrap().path),
		name,
		plays,
	})
}

/// Returns the full path of available role files in alphabetical order.
fn get_plays() -> IntoIter<Play> {
	let roles_dir = config::get_roles_dir();

	let Ok(entries) = fs::read_dir(roles_dir) else {
		return IntoIter::default()
	};

	entries
		.filter_map(Result::ok)
		.filter(is_yml_file)
		.map(|r| Play { path: r.path() })
		.sorted_by(|a, b| PathBuf::cmp(&a.path, &b.path))
}

fn is_yml_file(entry: &DirEntry) -> bool {
	let path = entry.path();

	path.is_file() && path.extension().map(|e| e == "yml").unwrap_or(false)
}

fn get_play_name<T: AsRef<Path>>(path: T) -> Option<String> {
	let stem = path.as_ref().file_stem()?.to_str()?;

	Some(PRIORITY_REGEX.replace(stem, "").into_owned())
}

fn get_play_description(path: &Path) -> Option<String> {
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
	pub plays: Vec<Play>,
}

pub struct Play {
	pub path: PathBuf,
}
