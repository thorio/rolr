use crate::{
	playbook,
	roles::{self, Role},
};
use anyhow::{anyhow, Result};
use dialoguer::MultiSelect;
use itertools::Itertools;
use std::collections::HashSet;

pub fn main() -> Result<()> {
	let mut active_roles = roles::get_active_roles();

	let inactive_roles = roles::get_roles()
		.filter(|r| !active_roles.contains(&r.name))
		.collect_vec();

	if inactive_roles.is_empty() {
		return Err(anyhow!("No inactive roles to select"));
	}

	let new_roles = HashSet::from_iter(select_roles(inactive_roles)?);

	let plays = roles::get_plays_for_roles(roles::get_plays(), &new_roles);

	active_roles.extend(new_roles);
	roles::set_active_roles(&active_roles)?;

	playbook::run_plays(&plays)
}

pub fn select_roles(roles: Vec<Role>) -> Result<Vec<String>> {
	let items = roles.iter().map(|r| r.display(20).to_string()).collect_vec();

	let chosen_indices = MultiSelect::new().items(&items).interact_opt()?.unwrap_or_default();

	let chosen_roles = roles
		.into_iter()
		.enumerate()
		.filter(|(i, _)| chosen_indices.contains(i))
		.map(|(_, role)| role.name)
		.collect_vec();

	Ok(chosen_roles)
}
