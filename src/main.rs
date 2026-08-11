use anyhow::Result;
use cli::Commands;
use log::LevelFilter;
use std::process::exit;

mod cli;
mod commands;
mod config;
mod playbook;
mod roles;

fn main() {
	let cli = cli::parse();

	init_logger(cli.verbosity.log_level_filter());

	if let Err(err) = run_command(cli) {
		log::error!("{err}");
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
		Commands::Complete { kind } => commands::complete(kind),
	}
}

fn init_logger(level: LevelFilter) {
	stderrlog::new()
		.verbosity(level)
		.timestamp(stderrlog::Timestamp::Off)
		.init()
		.expect("first call must succeed");
}
