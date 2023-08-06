use itertools::Itertools;

use crate::{playbook, roles};
use std::collections::HashSet;

pub fn main(roles: Vec<String>) {
	let all_plays = roles::get_plays().collect_vec();

	// not strictly necessary, but warns the user when they enter bogus roles
	let roles = roles::filter_invalid_roles(&all_plays, roles, true);

	let roles: HashSet<String> = HashSet::from_iter(roles);
	let plays = roles::get_plays_for_roles(roles::get_plays(), &roles);

	playbook::run_plays(&plays).unwrap();
}
