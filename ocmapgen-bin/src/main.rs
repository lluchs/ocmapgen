#[macro_use]
extern crate clap;
#[macro_use]
extern crate error_chain;
extern crate image;
extern crate notify;
extern crate ocmapgen;

use clap::{Arg, App};
use notify::{Watcher, RecursiveMode, DebouncedEvent, watcher};
use ocmapgen::easy::{Easy, RenderConfig, load_scenpar};

use std::path::Path;
use std::sync::mpsc::channel;
use std::time::Duration;
use std::io::prelude::*;

error_chain! { }

quick_main!(run);
fn run() -> Result<()> {
    let matches = App::new("ocmapgen")
        .version(env!("CARGO_PKG_VERSION"))
        .arg(Arg::with_name("root")
             .short("r").long("root")
             .help("Base directory. Should be a subdirectory of the OpenClonk “planet” root directory (defaults to directory of input file)")
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
        .arg(Arg::with_name("INPUT")
             .help("Input file (e.g. Map.c)")
             .required(true)
             .index(1))
        .arg(Arg::with_name("OUTPUT")
             .help("Output file (e.g. Map.png)")
             .required(true)
             .index(2))
        .get_matches();

    let mut mapgen = Easy::new().chain_err(|| "couldn't initialize map generator")?;
    let input_file = Path::new(matches.value_of("INPUT").unwrap())
        .canonicalize()
        .chain_err(|| "couldn't resolve input file path")?;
    let output_file = matches.value_of("OUTPUT").unwrap();
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
    cfg.filename(input_file.to_str().unwrap())
       .width(width)
       .height(height);

    render(&cfg, output_file)?;

    if matches.is_present("watch") {
        watch(&cfg, &input_file, output_file)?;
    }

    Ok(())
}

fn render(cfg: &RenderConfig, output_file: &str) -> Result<()> {
    let map = cfg.render().chain_err(|| "map rendering failed")?;
    map.save(output_file)
       .chain_err(|| "writing output image failed")?;
    Ok(())
}

fn watch(cfg: &RenderConfig, input_file: &Path, output_file: &str) -> Result<()> {
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
            report_error(render(cfg, output_file));
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
