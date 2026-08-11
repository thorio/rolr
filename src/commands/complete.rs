use crate::cli::CompleteKind;
use crate::roles::{self, Role};
use anyhow::Result;
use itertools::Itertools;

pub fn main(kind: CompleteKind) -> Result<()> {
	let roles = roles::get_roles().collect_vec();
	let active_roles = roles::get_active_roles();

	match kind {
		CompleteKind::AllRoles => print_roles(roles.iter()),
		CompleteKind::ActiveRoles => print_roles(roles.iter().filter(|r| active_roles.contains(&r.name))),
		CompleteKind::InactiveRoles => print_roles(roles.iter().filter(|r| !active_roles.contains(&r.name))),
	}

	Ok(())
}

fn print_roles<'a>(roles: impl Iterator<Item = &'a Role>) {
	for role in roles {
		println!("{}:{}", role.name, role.description.as_deref().unwrap_or(""));
	}
}
