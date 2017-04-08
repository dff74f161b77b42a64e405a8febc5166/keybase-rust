extern crate rustc_serialize;

mod keybase {
	use rustc_serialize::json::Json;
	use std::process::Command;
	use std::env;
	use std::path::Path;
	use std::fs::File;
	use std::io::Read;

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

	pub fn config_file() -> String {
		match env::home_dir() {
			Some(home) => Path::new(home.as_os_str())
								.join(".config/keybase/config.json")
								.to_str().unwrap().to_string(),
			None => panic!("no home dir")
		}
	}

	pub fn config_json() -> Json {
		let mut file = File::open(config_file()).unwrap();
		let mut data = String::new();
		file.read_to_string(&mut data).unwrap();

		Json::from_str(&data).unwrap()
	}
}
