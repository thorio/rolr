use clap::{Parser, Subcommand};

pub fn parse() -> Cli {
	Cli::parse()
}

#[derive(Parser)]
#[command(author, version, about, long_about = None)]
#[command(propagate_version = true)]
pub struct Cli {
	#[command(subcommand)]
	pub command: Commands,

	/// Set log level
	#[arg(short, default_value = "3", value_parser = clap::value_parser!(u8).range(0..6))]
	pub verbosity: u8,
}

#[derive(Subcommand)]
pub enum Commands {
	/// Select roles
	Add { roles: Vec<String> },

	/// Select roles in a graphical menu
	Select,

	/// Update all selected roles
	Update,

	/// List available roles
	List,

	/// Run roles without selecting them
	Run { roles: Vec<String> },

	/// Open the role's playbook in a pager
	Show { role: String },
}
