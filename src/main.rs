use cli::Commands;

mod cli;
mod commands;
mod config;
mod playbook;
mod roles;

fn main() {
	let cli = cli::parse();

	match cli.command {
		// Commands::Add { roles } => (),
		// Commands::Select => (),
		Commands::Update => commands::update(),
		Commands::List => commands::list(),
		Commands::Run { roles } => commands::run(roles),
		// Commands::Show { role } => (),
		_ => (),
	};
}
