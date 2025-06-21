{ pkgs, lib, config, inputs, ... }:

{
	languages.rust = {
		enable = true;
		channel = "nightly";
    targets = [ "x86_64-unknown-linux-musl" ];
		components = [
			"rustc"
			"cargo"
			"clippy"
			"rustfmt"
			"rust-analyzer"
		];
	};
	packages = with pkgs; [
		pkg-config
    gcc
    musl
	];
}
