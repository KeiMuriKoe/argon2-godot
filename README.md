# Godot Argon2 GDExtension

A simple [Godot](https://godotengine.org/) plugin (GDExtension) that adds password hashing and verification using the [Argon2](https://crates.io/crates/argon2) algorithm.  
Under the hood, it's a Rust binding compiled into a dynamic library.

## Features
- Password hashing with Argon2.
- Password verification against an existing hash.
- Cross-platform (Windows, Linux, macOS(need build)).

## Installation
1. Download the archive for your operating system from the **Releases** section.
2. Extract it into the `addons` folder in the root of your Godot project.

<a href="https://ibb.co/0jDHqWDV"><img src="https://i.ibb.co/gMjkyXjF/BA7-F71-CA-EE18-4-D03-822-C-F4456-C1312-D4.png" alt="BA7-F71-CA-EE18-4-D03-822-C-F4456-C1312-D4" border="0"></a>

## Usage
In GDScript you can call the plugin methods directly:

```gdscript
var hash = ArgonExtension.hash_password("my_secret_password")
var is_valid = ArgonExtension.verify_password(hash,"my_secret_password")
print("Password valid? ", is_valid)
