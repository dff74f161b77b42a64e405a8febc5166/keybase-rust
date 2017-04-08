mod keybase {
	use std::process::Command;

	pub fn ensure_running() -> bool {
		Command::new("keybase")
					.arg("ctl")
					.arg("start")
					.status()
					.expect("failed to execute keybase ctl start")
					.success()
	}

	pub fn version() -> String {
		let raw = Command::new("keybase")
					.arg("--version")
					.output()
					.expect("failed to execute keybase --version")
					.stdout;

		return String::from_utf8(raw).expect("invalid output");
	}
}
