use crate::{config, roles::Play};
use anyhow::{anyhow, Result};
use std::{
	fs::File,
	io::{BufWriter, Write},
	os::unix::process::CommandExt,
	path::Path,
	process::Command,
};

#[allow(unused)]
pub fn run_plays(plays: &[Play]) -> Result<()> {
	let playbook_path = config::get_playbook_file();

	if plays.is_empty() {
		return Err(anyhow!("Nothing to do."));
	}

	generate(&playbook_path, plays)
		.map_err(|err| anyhow!("Unable to generate playbook at {playbook_path:?}: {err}"))?;

	run(&playbook_path);
}

fn generate(path: impl AsRef<Path>, plays: &[Play]) -> Result<()> {
	let file = File::create(path)?;
	let mut writer = BufWriter::new(file);

	writeln!(writer, "---")?;
	writeln!(writer, "# This file was automatically generated by rolr")?;

	for play in plays {
		writeln!(writer, "- import_playbook: {}", play.path.display())?;
	}

	Ok(())
}

fn run(path: impl AsRef<Path>) -> ! {
	let err = Command::new("ansible-playbook")
		.args(vec![
			"--connection=local",
			"--inventory=localhost,",
			"--limit=localhost",
			path.as_ref().to_str().unwrap(),
			"-K",
		])
		.exec();

	panic!("exec ansible-playbook didn't work: {err}");
}
