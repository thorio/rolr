use anyhow::Result;
use cli::{CliArgs, Commands};
use std::cmp::max;

mod cli;
mod commands;
mod config;
mod playbook;
mod roles;

fn main() {
	let cli = cli::parse();

	init_logger(cli.verbosity.into());

	run_command(cli).unwrap();
}

fn run_command(cli: CliArgs) -> Result<()> {
	match cli.command {
		Commands::List => commands::list(),
		Commands::Add { roles } => commands::add(roles),
		Commands::Select => commands::select(),
		Commands::Update => commands::update(),
		Commands::Run { roles } => commands::run(roles),
	}
}

fn init_logger(verbosity: usize) {
	stderrlog::new()
		.quiet(verbosity == 0)
		.verbosity(max(1, verbosity) - 1)
		.timestamp(stderrlog::Timestamp::Off)
		.init()
		.expect("logger init failed");
}
