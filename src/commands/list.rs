use crate::roles;
use console::style;
use itertools::Itertools;

pub fn main() {
	let roles = roles::get_roles().collect_vec();
	let active_roles = roles::get_active_roles();

	let max_name_len = roles.iter().map(|a| a.name.len()).max().unwrap_or_default();

	for role in roles {
		print_checkmark(active_roles.contains(&role.name));

		println!(
			"{:<padding$}  {}",
			role.name,
			role.description.unwrap_or_default(),
			padding = max_name_len
		);
	}
}

fn print_checkmark(show: bool) {
	if show {
		print!("{}", style("\u{2714} ").green());
	} else {
		print!("  ");
	}
}
