use parsing::{parse_args, Commands};

mod commands;
mod config;
mod parsing;
mod roles;

fn main() {
	let cli = parse_args();

	match &cli.command {
		// Commands::Add { roles } => (),
		Commands::Select => (),
		Commands::Update => (),
		Commands::List => commands::list(),
		// Commands::Run { roles } => (),
		// Commands::Show { role } => (),
		_ => (),
	};
}
