use actix_web_static_files::resource_dir;
use std::env;
use std::process::Command;


fn main() {
    if env::var("DEV_SERVER").is_err() {
        // call "npm run build" to write node assets to frontend/dist/
        Command::new("npm")
            .current_dir("frontend")
            .args(&["run", "build"])
            .status().unwrap();

        // use actix_web_static_files to write node assets to a generated rust file
        resource_dir("./frontend/dist").build().unwrap();

        // tell main.rs to use the generated file
        println!("cargo:rustc-cfg=feature=\"embed-assets\"");
    }
}