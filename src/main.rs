extern crate log;
use actix_files;
use actix_web::{get, middleware, web, App, HttpResponse, HttpServer, Responder};
use listenfd::ListenFd;
use rand::seq::SliceRandom;
use serde::{Serialize, Deserialize};
use std::collections::HashSet;
use std::{env, fs, thread, time};
use std::path::{Path, PathBuf};
use std::sync::Mutex;

// include generated assets in binary if not in debug mode
#[cfg(feature = "embed-assets")]
use actix_web_static_files;
#[cfg(feature = "embed-assets")]
use std::collections::HashMap;
#[cfg(feature = "embed-assets")]
include!(concat!(env!("OUT_DIR"), "/generated.rs"));

#[derive(Debug, PartialEq, Serialize, Deserialize, Default)]
struct Picture {
    #[serde(default)]
    file_name: String,
    source: String,
    caption: String,
}

struct AppState {
    pictures_dir: PathBuf,
    pictures: Mutex<Vec<Picture>>,
}

fn get_env(key: &str, default: &str) -> String {
    env::var(key).unwrap_or(default.to_string())
}

fn picture_collector_thread(app_state: web::Data<AppState>) {
    let image_suffixes: HashSet<&'static str> = ["jpg", "jpeg", "gif", "png"].iter().cloned().collect();
    let reindex_seconds:u64 = get_env("REINDEX_SECONDS", "60").parse().unwrap_or(60);
    loop {
        // get all images in dir
        let paths = fs::read_dir(&app_state.pictures_dir).unwrap();
        let names = paths.filter_map(|entry| {
            entry.ok()
                .filter(|e| image_suffixes.contains(e.path().extension().map(|s| s.to_str().unwrap_or("")).unwrap_or("")))
                .and_then(|e|
                e.path().file_name().and_then(|s| s.to_str().map(|s| String::from(s)))
            )
        }).collect::<Vec<String>>();

        // load metadata for each image
        let new_pictures = names.iter().map(|name| {
            let mut picture:Picture =
                fs::read_to_string(app_state.pictures_dir.join(name.to_owned()+".yml")).ok()
                    .and_then(|s| serde_yaml::from_str(&s).ok())
                    .unwrap_or(Default::default());
            picture.file_name = name.clone();
            picture
        }).collect();

        // update mutex -- in block so we release before thread::sleep
        {
            *app_state.pictures.lock().unwrap() = new_pictures;
        }

        // sleep
        thread::sleep(time::Duration::from_secs(reindex_seconds));
    }
}

#[get("/api/foo")]
fn get_picture(app_state: web::Data<AppState>) -> impl Responder {
    let pictures = app_state.pictures.lock().unwrap();
    let picture = pictures.choose(&mut rand::thread_rng()).unwrap();
    HttpResponse::Ok().json(picture.clone())
}

fn main() -> std::io::Result<()> {
    println!("Holiday Card v. {}", env!("CARGO_PKG_VERSION"));

    // get pictures dir
    let pictures_dir = Path::new(&env::current_dir().unwrap())
        .join(get_env("PICTURES_DIR", "pictures"));

    // check config
    if !pictures_dir.exists() {
        eprintln!("Error: pictures directory {} does not exist. Use PICTURES_DIR to change location.", pictures_dir.display());
        std::process::exit(1);
    }

    // set up logging
    if env::var("RUST_LOG").is_err() {
        env::set_var("RUST_LOG", "actix_server=info,actix_web=info");
    }
    env_logger::init();

    // set up state
    let app_state = web::Data::new(AppState {
        pictures_dir,
        pictures: Mutex::new(vec![]),
    });

    // get files
    let app_state_clone = app_state.clone();
    thread::spawn(move || picture_collector_thread(app_state_clone));

    // set up app
    let mut server = HttpServer::new(move || {
        let mut app = App::new()
            .register_data(app_state.clone())
            .wrap(middleware::Compress::default())
            .wrap(middleware::Logger::default())
            .wrap(middleware::DefaultHeaders::new().header("Set-Cookie", format!("speed={}", get_env("SPEED", "5"))))
            .service(get_picture)
            .service(actix_files::Files::new("/pictures", &app_state.pictures_dir));
        #[cfg(feature = "embed-assets")]
        {
            let mut generated = generate();
            generated.get("index.html")
                .map(|resource| actix_web_static_files::Resource { data: resource.data, modified: resource.modified, mime_type: resource.mime_type })
                .and_then(|resource| generated.insert("", resource));
            app = app.service(actix_web_static_files::ResourceFiles::new(
                "/" ,
                generated,
            ));
        }
        app
    });

    // bind server
    let mut listenfd = ListenFd::from_env();
    server = if let Some(l) = listenfd.take_tcp_listener(0).unwrap() {
        server.listen(l).unwrap()
    } else {
        let bind_addr = get_env("BIND_ADDR", "127.0.0.1:8000");
        server.bind(bind_addr.clone())
            .unwrap_or_else(|_| {
                eprintln!("Error: cannot bind to {}. Use BIND_ADDR to change address.", bind_addr.clone());
                std::process::exit(1)
            })
    };

    server.run()
}