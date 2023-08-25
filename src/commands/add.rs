use crate::{playbook, roles};
use anyhow::Result;
use itertools::Itertools;
use std::collections::HashSet;

pub fn main(roles: Vec<String>) -> Result<()> {
	let all_plays = roles::get_plays().collect_vec();
	let mut active_roles = roles::get_active_roles();
	let valid_new_roles = roles::filter_invalid_roles(&all_plays, roles, true);
	let inactive_new_roles = roles::filter_active_roles(&active_roles, valid_new_roles, true)
		.into_iter()
		.collect::<HashSet<_>>();

	let plays = roles::get_plays_for_roles(all_plays.into_iter(), &inactive_new_roles);

	active_roles.extend(inactive_new_roles);
	roles::set_active_roles(&active_roles)?;

	playbook::run_plays(&plays)?;

	Ok(())
}
