file::delete("Cargo.toml");
file::rename("cargo-generate/Cargo.toml", "Cargo.toml");
file::delete(".vscode/launch.json");
file::delete(".vscode/tasks.json");
file::rename("cargo-generate/launch.json", ".vscode/launch.json");
file::rename("cargo-generate/tasks.json", ".vscode/tasks.json");
file::delete("cargo-generate")
