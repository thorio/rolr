use crate::{playbook, roles};

pub fn main() {
	let roles = roles::get_active_roles();
	let plays = roles::get_plays().filter(|r| roles.contains(&r.play_name));

	playbook::run_plays(plays).unwrap();
}
