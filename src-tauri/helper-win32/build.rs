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
            "[Helper] [{}] MUST be set to properly target main executable",
            stringify!($field)
        ))
    };
}

macro_rules! require_release {
    ($tenv:literal, $uenv:expr) => {
        if cfg!(debug_assertions) {
            $uenv
        } else {
            require_field!(option_env!($tenv))
        }
    };
}

fn main() {
    let application_name = require_release!("APPLICATION_NAME", "Volumize");
    let application_exe = require_release!("APPLICATION_EXE", "volumize.exe");
    let application_icon = require_release!("APPLICATION_ICON", "");

    expose_env("APPLICATION_NAME", application_name);
    expose_env("APPLICATION_EXE", application_exe);
    // expose_env("APPLICATION_ICON", application_icon);

    #[cfg(windows)]
    {
        let mut res = winresource::WindowsResource::new();

        res.set_icon(application_icon);

        // These strings appear in the UAC prompt and file properties
        res.set("FileDescription", "Volumize Firewall Helper");
        res.set("ProductName", application_name);
        res.set("CompanyName", "Firewall Helper");
        res.set("LegalCopyright", "Copyright © 2026 {Author}");

        // Embed the manifest
        res.set_manifest_file("manifest.xml");

        #[cfg(not(debug_assertions))]
        res.compile().expect("Failed to compile Windows resources");
    }
}

fn expose_env(key: &str, value: &str) {
    println!("cargo:rustc-env={}={}", key, value);
}
