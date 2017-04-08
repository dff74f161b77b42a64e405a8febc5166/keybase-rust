extern crate rustc_serialize;

mod keybase {
	use rustc_serialize::json;
	use std::process::Command;
	use std::env;
	use std::path::Path;
	use std::fs::File;
	use std::io::Read;
	use std::collections::HashMap;

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

	#[derive(RustcDecodable)]
	pub struct Config {
		pub current_user: String,
		pub users: HashMap<String, String>
	}

	impl Config {
		pub fn new() -> Config {
			let string = Config::config_json();
			let config: Config = json::decode(&string).unwrap();

			config
		}

		fn config_file() -> String {
			match env::home_dir() {
				Some(home) => Path::new(home.as_os_str())
									.join(".config/keybase/config.json")
									.to_str().unwrap().to_string(),
				None => panic!("no home dir")
			}
		}

		fn config_json() -> String {
			let mut file = File::open(Config::config_file()).unwrap();
			let mut data = String::new();
			file.read_to_string(&mut data).unwrap();

			data
		}
	}
}
