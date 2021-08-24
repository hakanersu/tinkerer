#![cfg_attr(
  all(not(debug_assertions), target_os = "windows"),
  windows_subsystem = "windows"
)]
use std::io::prelude::*;
use std::process::{Command, Stdio};

#[tauri::command]
fn tinker(command: String) -> String {
	let process = match Command::new("php")
		.arg("/home/xuma/Sites/imsec/artisan")
		.arg("tinker")
		.stdin(Stdio::piped())
		.stdout(Stdio::piped())
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
	    Ok(_) => () ,
	}

	return s
}

fn main() {

  tauri::Builder::default()
    .invoke_handler(tauri::generate_handler![tinker])
    .run(tauri::generate_context!())
    .expect("error while running tauri application");
}
