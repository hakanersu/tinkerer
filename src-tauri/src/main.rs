#![cfg_attr(
  all(not(debug_assertions), target_os = "windows"),
  windows_subsystem = "windows"
)]
use std::io::prelude::*;
use std::process::{Command, Stdio};

#[tauri::command]
fn tinker(command: String, path: String, bin: String) -> String {
	let artisan_path = path + "/artisan";
	let b = std::path::Path::new(&artisan_path).exists();
	if !b {
		return String::from("Artisan file not found. Please check your path");
	}
	let process = match Command::new(bin)
		.arg(artisan_path)
		.arg("tinker")
		.stdin(Stdio::piped())
		.stdout(Stdio::piped())
		.stderr(Stdio::piped())
		.spawn() {
			Err(why) => panic!("couldn't spawn tinker: {}", why),
			Ok(process) => process,
		    };
	
	match process.stdin.unwrap().write_all(command.as_bytes()) {
		Err(why) => panic!("couldn't write to tinker stdin: {}", why),
		Ok(r) => r,
	}

	let mut s = String::new();
	
	match process.stdout.unwrap().read_to_string(&mut s) {
		Err(why) => panic!("couldn't read tinker stdout: {}", why),
	    	Ok(_) => (),
	}

	match process.stderr.unwrap().read_to_string(&mut s) {
		Err(why) => panic!("couldn't read tinker stderr: {}", why),
		Ok(_) => (),
	}

	return s
}

fn main() {
  tauri::Builder::default()
    .invoke_handler(tauri::generate_handler![tinker])
    .run(tauri::generate_context!())
    .expect("error while running tauri application");
}
