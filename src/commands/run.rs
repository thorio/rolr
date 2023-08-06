use itertools::Itertools;

use crate::{playbook, roles};
use std::collections::HashSet;

pub fn main(roles: Vec<String>) {
	let roles: HashSet<String> = HashSet::from_iter(roles);
	let plays = roles::get_plays()
		.filter(|r| roles.contains(&r.play_name))
		.collect_vec();

	playbook::run_plays(&plays).unwrap();
}
