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
    ($tenv:literal) => {
        if cfg!(debug_assertions) {
            ""
        } else {
            require_field!(option_env!($tenv))
        }
    };
}

fn main() {
    let application_name = require_release!("APPLICATION_NAME", "Volumize");
    let application_icon = require_release!("APPLICATION_ICON");

    expose_env("APPLICATION_NAME", application_name);
    // expose_env("APPLICATION_ICON", application_icon);

    if std::env::var("CARGO_CFG_TARGET_OS").unwrap() == "windows" {
        let mut res = winresource::WindowsResource::new();

        res.set_icon(application_icon);

        // These strings appear in the UAC prompt and file properties
        // Only if it's signed binary.
        res.set("FileDescription", "Volumize Firewall Helper");
        res.set("ProductName", application_name);
        res.set("CompanyName", application_name);
        res.set("LegalCopyright", "Copyright © 2026 {Author}");
        res.set_manifest_file("manifest.xml");

        #[cfg(not(debug_assertions))]
        res.compile().expect("Failed to compile Windows resources");
    }
}

fn expose_env(key: &str, value: &str) {
    println!("cargo:rustc-env={}={}", key, value);
}
