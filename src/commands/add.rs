use crate::{
	playbook,
	roles::{self, Play},
};
use itertools::Itertools;
use log::warn;
use std::collections::HashSet;

pub fn main(roles: Vec<String>) {
	let all_plays = roles::get_plays().collect_vec();

	let mut active_roles = roles::get_active_roles();
	let valid_new_roles = filter_invalid_roles(&all_plays, roles);

	let plays = roles::get_plays()
		.filter(|r| valid_new_roles.contains(&r.play_name))
		.collect_vec();

	active_roles.extend(valid_new_roles);

	roles::set_active_roles(&active_roles).unwrap();

	playbook::run_plays(&plays).unwrap();
}

pub fn filter_invalid_roles(all_plays: &[Play], roles: Vec<String>) -> Vec<String> {
	let valid_roles = all_plays.iter().map(|a| &a.play_name).collect::<HashSet<&String>>();

	let (valid, invalid): (Vec<_>, Vec<_>) = roles.into_iter().partition(|r| valid_roles.contains(r));

	for invalid_role in invalid {
		warn!(r#"Skipping unknown role "{}""#, invalid_role)
	}

	valid
}
