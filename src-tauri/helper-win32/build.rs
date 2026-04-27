/// Ensures that an `Option` field is set, otherwise panics with a descriptive message.
///
/// This macro unwraps the given `Option<T>`. If the value is `None`,
/// it panics with a message that includes the field name.
///
/// The field name is automatically derived using `stringify!`, so you
/// do not need to pass it manually.
#[allow(unused_macros)]
macro_rules! require_field {
    ($field:expr) => {
        $field.expect(&format!(
            "Application [{}] MUST be set to properly target main executable",
            stringify!($field)
        ))
    };
}

fn main() {
    #[cfg(debug_assertions)]
    {
        expose_env("APPLICATION_NAME", "Volumize");
    }
    #[cfg(not(debug_assertions))]
    {
        pub const APPLICATION_NAME: Option<&str> = option_env!("APPLICATION_NAME");
        expose_env("APPLICATION_NAME", require_field!(APPLICATION_NAME));
    }
}

fn expose_env(key: &str, value: &str) {
    println!("cargo:rustc-env={}={}", key, value);
}
