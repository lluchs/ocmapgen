#[macro_use]
extern crate clap;
#[macro_use]
extern crate error_chain;
extern crate image;
extern crate notify;
extern crate ocmapgen;
extern crate ocmapgen_bin;

use clap::{Arg, App};
use notify::{Watcher, RecursiveMode, DebouncedEvent, watcher};
use ocmapgen::easy::{Easy, RenderConfig, MapType, load_scenpar};
use ocmapgen::{openclonk_version, seed_rng};
use ocmapgen_bin::msg;

use std::path::{Path, PathBuf};
use std::sync::mpsc::channel;
use std::time::{Duration, SystemTime, UNIX_EPOCH};
use std::io::prelude::*;

error_chain! { }

quick_main!(run);
fn run() -> Result<()> {
    let matches = App::new("ocmapgen")
        .version(format!("{} with OC {}", env!("CARGO_PKG_VERSION"), openclonk_version()).as_str())
        .arg(Arg::with_name("root")
             .short("r").long("root")
             .help("Base directory. Should be a subdirectory of the OpenClonk “planet” root directory (defaults to directory of input file)")
             .takes_value(true))
        .arg(Arg::with_name("seed")
             .short("s").long("seed")
             .help("Set a fixed RNG seed value (defaults to a random seed)")
             .takes_value(true))
        .arg(Arg::with_name("width")
             .short("w").long("width")
             .help("Width of the output image")
             .takes_value(true)
             .default_value("200"))
        .arg(Arg::with_name("height")
             .short("h").long("height")
             .help("Height of the output image")
             .takes_value(true)
             .default_value("200"))
        .arg(Arg::with_name("players")
             .long("players")
             .help("Set the result of GetStartupPlayerCount()")
             .takes_value(true)
             .default_value("1"))
        .arg(Arg::with_name("teams")
             .long("teams")
             .help("Set the result of GetStartupTeamCount()")
             .takes_value(true)
             .default_value("1"))
        .arg(Arg::with_name("watch")
             .long("watch")
             .help("Watch input file for changes")
             .takes_value(false))
        .arg(Arg::with_name("bg-output")
             .long("bg")
             .help("Write map background to file")
             .takes_value(true))
        .arg(Arg::with_name("map-type")
             .long("map-type")
             .help("Type of map. Inferred from input file name per default")
             .takes_value(true)
             .possible_values(&["Landscape.txt", "Map.c"]))
        .arg(Arg::with_name("cbor")
             .long("cbor")
             .help("Enable cbor interface")
             .hidden(true) // not useful for human users
             .takes_value(false))
        .arg(Arg::with_name("INPUT")
             .help("Input file (e.g. Map.c)")
             .required_unless_all(&["cbor", "root", "map-type"])
             .index(1))
        .arg(Arg::with_name("OUTPUT")
             .help("Output file (e.g. Map.png)")
             .required_unless("cbor")
             .index(2))
        .get_matches();

    let mut mapgen = Easy::new().chain_err(|| "couldn't initialize map generator")?;
    let input_file = match matches.value_of("INPUT") {
        Some(f) => Path::new(f)
                   .canonicalize()
                   .chain_err(|| "couldn't resolve input file path")?,
        None => PathBuf::new() // dummy, won't be used
    };
    let output_file = matches.value_of("OUTPUT").unwrap_or("");
    let base_path = match matches.value_of("root") {
        Some(p) => p.to_owned(),
        None => {
            let mut p = input_file.clone();
            p.pop();
            p.to_str().unwrap().to_owned()
        }
    };
    mapgen.set_base_path(&base_path)
        .chain_err(|| "couldn't find Material.ocg or Objects.ocd")?;

    let seed = value_t!(matches.value_of("seed"), u32)
               .unwrap_or_else(|_| SystemTime::now().duration_since(UNIX_EPOCH)
                                   .expect("failed getting a timestamp")
                                   .subsec_nanos());
    seed_rng(seed);

    let players = value_t!(matches.value_of("players"), i32)
                  .chain_err(|| "invalid --players option")?;
    let teams = value_t!(matches.value_of("teams"), i32)
                  .chain_err(|| "invalid --teams option")?;
    mapgen.mapgen.set_startup_player_count(players);
    mapgen.mapgen.set_startup_team_count(teams);

    let width = value_t!(matches.value_of("width"), u32)
                .chain_err(|| "invalid width")?;
    let height = value_t!(matches.value_of("height"), u32)
                .chain_err(|| "invalid height")?;
    let maybe_scenpar = load_scenpar(&base_path);
    let mut cfg = mapgen.build();
    if let Ok(ref scenpar) = maybe_scenpar {
        cfg.scenpar(scenpar);
    }

    cfg.width(width)
       .height(height);

    if matches.is_present("INPUT") {
        cfg.filename(input_file.to_str().unwrap());
    }

    match matches.value_of("map-type") {
        Some("Landscape.txt") => { cfg.map_type(MapType::LandscapeTxt); },
        Some("Map.c")         => { cfg.map_type(MapType::MapC); },
        _ => () // clap filters invalid values
    }

    let bg_output = matches.value_of("bg-output");

    if matches.is_present("cbor") {
        handle_requests(cfg, bg_output, matches.value_of("seed").map(|_| seed))?;
    } else {
        render(&cfg, output_file, bg_output)?;

        if matches.is_present("watch") {
            watch(&cfg, &input_file, output_file, bg_output, matches.value_of("seed").map(|_| seed))?;
        }
    }

    Ok(())
}

fn render(cfg: &RenderConfig, output_file: &str, output_file_bg: Option<&str>) -> Result<()> {
    let map_handle = cfg.render().chain_err(|| "map rendering failed")?;
    // write foreground map...
    if is_bmp(output_file) {
        map_handle.save_map(output_file)
                  .chain_err(|| "writing output image failed")
    } else {
        map_handle.map_as_image()
                  .save(output_file)
                  .chain_err(|| "writing output image failed")
    }?;
    // ...and optionally background map
    if let Some(output_file_bg) = output_file_bg {
        if is_bmp(output_file_bg) {
            map_handle.save_map_bg(output_file_bg)
                      .chain_err(|| "writing bg output image failed")
        } else {
            map_handle.map_bg_as_image()
                      .save(output_file_bg)
                      .chain_err(|| "writing bg output image failed")
        }?;
    }
    Ok(())
}

fn is_bmp(path: &str) -> bool {
    path.ends_with(".bmp")
}

fn watch(cfg: &RenderConfig, input_file: &Path, output_file: &str, output_file_bg: Option<&str>, seed: Option<u32>) -> Result<()> {
    let (tx, rx) = channel();
    let mut watcher = watcher(tx, Duration::from_millis(100))
        .chain_err(|| "could not initialize watcher")?;

    // Watch the parent directory as the file may be removed temporarily on write.
    let dir = input_file.parent().unwrap();
    watcher.watch(dir, RecursiveMode::NonRecursive)
        .chain_err(|| "could not start watcher")?;

    println!("Waiting for file changes…");

    loop {
        let event = rx.recv().chain_err(|| "watch error")?;
        let rerender = match event {
            DebouncedEvent::Create(f) | DebouncedEvent::Write(f)
                => f == input_file,
            _   => false
        };
        if rerender {
            println!("File changed, rendering map…");
            if let Some(seed) = seed {
                seed_rng(seed);
            }
            report_error(render(cfg, output_file, output_file_bg.clone()));
        }
    }
}

fn report_error<T>(res: Result<T>) {
    match res {
        Ok(_) => (),
        Err(ref e) => {
            write!(&mut std::io::stderr(), "{}", error_chain::ChainedError::display(e))
                .expect("Error writing to stderr");
        }
    }
}

fn handle_requests(mut cfg: RenderConfig, output_file_bg: Option<&str>, seed: Option<u32>) -> Result<()> {
    loop {
        let req = msg::read_request().chain_err(|| "couldn't read request")?;
        let res = match req {
            msg::Request::RenderMap { source } => {
                cfg.source(&source);
                match cfg.render() {
                    Ok(map_handle) => msg::Response::Image {
                        fg: to_png(map_handle.map_as_image())?.into(),
                        bg: match output_file_bg {
                                Some(_) => Some(to_png(map_handle.map_bg_as_image())?.into()),
                                None => None,
                            }
                    },
                    Err(err) => msg::Response::Error(error_chain::ChainedError::display(&err).to_string()),
                }
            },
        };
        msg::write_response(&res).chain_err(|| "couldn't write response")?;

        if let Some(seed) = seed {
            seed_rng(seed);
        }
    }
}

fn to_png(img: image::RgbImage) -> Result<Vec<u8>> {
    use image::Pixel;

    let mut result = Vec::new();
    {
        let encoder = image::png::PNGEncoder::new(&mut result);
        encoder.encode(&img, img.width(), img.height(), image::Rgb::<u8>::color_type())
            .chain_err(|| "PNG encoding failed")?;
    }
    Ok(result)
}
