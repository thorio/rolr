use cli::Commands;
use std::cmp::max;

mod cli;
mod commands;
mod config;
mod playbook;
mod roles;

fn main() {
	let cli = cli::parse();

	init_logger(cli.verbosity.into());

	match cli.command {
		Commands::List => commands::list(),
		Commands::Add { roles } => commands::add(roles),
		// Commands::Select => (),
		Commands::Update => commands::update(),
		Commands::Run { roles } => commands::run(roles),
		// Commands::Show { role } => (),
		_ => (),
	};
}

fn init_logger(verbosity: usize) {
	stderrlog::new()
		.quiet(verbosity == 0)
		.verbosity(max(1, verbosity) - 1)
		.timestamp(stderrlog::Timestamp::Off)
		.init()
		.expect("logger init failed");
}
