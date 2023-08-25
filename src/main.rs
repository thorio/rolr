use anyhow::Result;
use cli::Commands;
use log::error;
use std::{cmp::max, process::exit};

mod cli;
mod commands;
mod config;
mod playbook;
mod roles;

fn main() {
	let cli = cli::parse();

	init_logger(cli.verbosity.into());

	if let Err(err) = run_command(cli) {
		error!("{}", err);
		exit(1);
	}
}

fn run_command(cli: cli::Args) -> Result<()> {
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
		.expect("logger already initialized");
}
