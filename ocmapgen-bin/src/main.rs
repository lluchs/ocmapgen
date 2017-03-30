#[macro_use]
extern crate clap;
#[macro_use]
extern crate error_chain;
extern crate image;
extern crate ocmapgen;

use clap::{Arg, App};
use ocmapgen::easy::{Easy, load_scenpar};

use std::path::Path;

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
        .arg(Arg::with_name("INPUT")
             .help("Input file (e.g. Map.c)")
             .required(true)
             .index(1))
        .arg(Arg::with_name("OUTPUT")
             .help("Output file (e.g. Map.png)")
             .required(true)
             .index(2))
        .get_matches();

    let mut mapgen = Easy::new();
    let input_file = matches.value_of("INPUT").unwrap();
    let base_path = match matches.value_of("root") {
        Some(p) => p.to_owned(),
        None => {
            let mut p = Path::new(input_file).canonicalize()
                        .chain_err(|| "couldn't resolve input file path")?;
            p.pop();
            p.to_str().unwrap().to_owned()
        }
    };
    mapgen.set_base_path(&base_path)
        .chain_err(|| "couldn't find Material.ocg or Objects.ocd")?;

    let width = value_t!(matches.value_of("width"), u32)
                .chain_err(|| "invalid width")?;
    let height = value_t!(matches.value_of("height"), u32)
                .chain_err(|| "invalid height")?;
    let maybe_scenpar = load_scenpar(&base_path);
    let mut cfg = mapgen.build();
    if let Ok(ref scenpar) = maybe_scenpar {
        cfg.scenpar(scenpar);
    }
    let map = cfg.filename(input_file)
       .width(width)
       .height(height)
       .render().chain_err(|| "map rendering failed")?;
    map.save(matches.value_of("OUTPUT").unwrap())
       .chain_err(|| "writing output image failed")?;

    Ok(())
}
