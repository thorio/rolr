use clap::{Parser, Subcommand};

pub fn parse_args() -> Cli {
	Cli::parse()
}

#[derive(Parser)]
#[command(author, version, about, long_about = None)]
#[command(propagate_version = true)]
pub struct Cli {
	#[command(subcommand)]
	pub command: Commands,
}

#[derive(Subcommand)]
pub enum Commands {
	/// select roles
	Add { roles: Vec<String> },

	/// select roles in a graphical menu
	Select,

	/// update all selected roles
	Update,

	/// list available roles
	List,

	/// run roles without selecting them
	Run { roles: Vec<String> },

	/// open the role's playbook in a pager
	Show { role: String },
}
