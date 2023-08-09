use crate::{
	playbook,
	roles::{self, Role},
};
use anyhow::Result;
use dialoguer::MultiSelect;
use itertools::Itertools;
use log::warn;
use std::collections::HashSet;

pub fn main() -> Result<()> {
	let mut active_roles = roles::get_active_roles();

	let inactive_roles = roles::get_roles()
		.filter(|r| !active_roles.contains(&r.name))
		.collect_vec();

	if inactive_roles.is_empty() {
		warn!("No inactive roles to select");
		return Ok(());
	}

	let new_roles = HashSet::from_iter(select_roles(inactive_roles)?);

	if new_roles.is_empty() {
		warn!("No roles selected");
		return Ok(());
	}

	let plays = roles::get_plays_for_roles(roles::get_plays(), &new_roles);

	active_roles.extend(new_roles);
	roles::set_active_roles(&active_roles)?;

	playbook::run_plays(&plays)?;

	Ok(())
}

pub fn select_roles(roles: Vec<Role>) -> Result<Vec<String>> {
	let items = roles.iter().map(|r| r.display(20).to_string()).collect_vec();

	let chosen: Vec<usize> = MultiSelect::new().items(&items).interact()?;

	Ok(chosen.into_iter().map(|i| roles[i].name.clone()).collect_vec())
}
