use clap::Parser;
use copypasta_ext::prelude::*;
use copypasta_ext::x11_bin::ClipboardContext;
use owo_colors::OwoColorize;
use pgp::Deserializable;
use pgp::composed::message::Message;
use pgp::composed::signed_key::SignedSecretKey;
use regex::Regex;
use std::fs;
use std::path::PathBuf;
use std::time::Duration;

#[derive(Parser)]
#[command(
	version,
	about = "Small tool to to automatically get Abacus 2fa login code."
)]
struct Args {
	#[arg(short, long, default_value = "./private_key.asc")]
	input_key_file: PathBuf,

	#[arg(short, long, help = "Outputs result to stdout instead of copying to clipboard")]
	output_to_stdout: bool,
}

fn main() {
	let args = Args::parse();

	let mut clip_ctx = ClipboardContext::new().unwrap();
	let clip_contents = clip_ctx
		.get_contents()
		.expect("failed reading clipboard, do you have xsel?");

	let message = Message::from_string(&clip_contents)
		.unwrap_or_else(|_| handle_err("invalid message in clipboard"))
		.0; // this looks fucking stupid lol

	let key_file = fs::read_to_string(args.input_key_file.as_path()).unwrap_or_else(|_| {
		handle_err(&format!(
			"file '{}' not found",
			args.input_key_file.display()
		))
	});

	let key = SignedSecretKey::from_string(&key_file)
		.unwrap_or_else(|_| handle_err("invalid key in 'private_key.asc'"))
		.0;

	let decryptedbytes = message
		.decrypt(|| "".to_string(), &[&key])
		.unwrap_or_else(|_| handle_err("failed to decrypt message, is it the wrong key?"))
		.0
		.get_content()
		.unwrap()
		.unwrap();
	let decrypted = String::from_utf8_lossy(&decryptedbytes);

	let output = Regex::new(r"[0-9a-f]{56}")
		.unwrap()
		.find(&decrypted)
		.unwrap_or_else(|| handle_err("failed to find code in decoded message, is it from abacus?"))
		.as_str()
		.to_owned();

	if args.output_to_stdout {
		print!("{}", output);
	} else {
		clip_ctx.set_contents(output).unwrap();
	}
	println!("{}", "Decoded string copied to clipboard!".bright_green());
	std::thread::sleep(Duration::from_secs(2));
	std::process::exit(0);
}

fn handle_err(err_message: &str) -> ! {
	eprintln!("{}", err_message.red());

	std::thread::sleep(Duration::from_secs(3));
	std::process::exit(1);
}
