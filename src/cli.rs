use clap::{Parser, Subcommand};
use clap_verbosity_flag::{InfoLevel, Verbosity};

pub fn parse() -> Args {
	Args::parse()
}

#[derive(Parser)]
#[command(author, version, about, long_about = None)]
#[command(propagate_version = true)]
pub struct Args {
	#[command(subcommand)]
	pub command: Commands,

	#[command(flatten)]
	pub verbosity: Verbosity<InfoLevel>,
}

#[derive(Subcommand)]
pub enum Commands {
	/// Activate roles
	Add { roles: Vec<String> },

	/// Activate roles in a TUI menu
	Select,

	/// Update all active roles
	Update,

	/// List available roles
	List,

	/// Run roles without activating them
	Run { roles: Vec<String> },

	/// Used for shell completions. Not stable!
	#[command(hide = true)]
	Complete { kind: CompleteKind },
}

#[expect(clippy::enum_variant_names)]
#[derive(Copy, Clone, clap::ValueEnum)]
pub enum CompleteKind {
	AllRoles,
	ActiveRoles,
	InactiveRoles,
}
