use godot::classes::Object;
use godot::prelude::*;

use argon2::{
    Argon2,
    password_hash::{PasswordHash, PasswordHasher, PasswordVerifier, SaltString, rand_core::OsRng},
};

struct MyExtension;

#[gdextension]
unsafe impl ExtensionLibrary for MyExtension {}

/// This is a Godot extension that provides Argon2 password hashing functionality.
/// It allows you to hash passwords and verify them against stored hashes.
#[derive(GodotClass)]
#[class(base=Object)]
struct ArgonExtension {
    base: Base<Object>,
}

#[godot_api]
impl IObject for ArgonExtension {
    fn init(base: Base<Object>) -> Self {
        Self { base }
    }
}

#[godot_api]
impl ArgonExtension {
    #[func]
    /// Hashes a password using Argon2.
    /// Returns the hashed password as a String.
    fn hash_password(password: GString) -> GString {
        // Ensure the password is not empty
        if password.is_empty() {
            godot_error!("Password cannot be empty");
            return GString::from("");
        }
        // Generate a random salt
        let salt = SaltString::generate(&mut OsRng);
        let argon2 = Argon2::default();
        // Convert GString to String and then to bytes
        let password_string = password.to_string();
        let password_bytes = password_string.as_bytes();
        // Hash the password using Argon2
        let password_hash = argon2
            .hash_password(password_bytes, &salt)
            .unwrap()
            .to_string();
        // Convert the password hash back to GString
        GString::from(password_hash)
    }

    #[func]
    /// Verifies a password against a stored hash.
    /// Returns true if the password matches the hash, false otherwise.
    fn verify_password(hash: GString, password: GString) -> bool {
        // Ensure the hash and password are not empty
        if hash.is_empty() || password.is_empty() {
            godot_error!("Hash and password cannot be empty");
            return false;
        }
        // Convert GString to String and then to bytes
        let hash_string = hash.to_string();
        let password_string = password.to_string();
        let password_bytes = password_string.as_bytes();
        // Parse the password hash
        let parsed_hash = PasswordHash::new(&hash_string).unwrap();
        let argon2 = Argon2::default();
        // Verify the password against the hash
        match argon2.verify_password(password_bytes, &parsed_hash) {
            Ok(_) => true,
            Err(_) => false,
        }
    }
}
