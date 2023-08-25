use clap::{Parser, Subcommand};

pub fn parse() -> CliArgs {
	CliArgs::parse()
}

#[derive(Parser)]
#[command(author, version, about, long_about = None)]
#[command(propagate_version = true)]
pub struct CliArgs {
	#[command(subcommand)]
	pub command: Commands,

	/// Set log level
	#[arg(short, default_value = "3", value_parser = clap::value_parser!(u8).range(0..6))]
	pub verbosity: u8,
}

#[derive(Subcommand)]
pub enum Commands {
	/// Activate roles
	Add { roles: Vec<String> },

	/// Activate roles in a graphical menu
	Select,

	/// Update all active roles
	Update,

	/// List available roles
	List,

	/// Run roles without activating them
	Run { roles: Vec<String> },
}
