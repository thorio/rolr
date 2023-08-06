use crate::{playbook, roles};
use anyhow::Result;

pub fn main() -> Result<()> {
	let plays = roles::get_plays_for_roles(roles::get_plays(), &roles::get_active_roles());

	playbook::run_plays(&plays)
}
