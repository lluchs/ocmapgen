#[macro_use]
extern crate clap;
extern crate image;
extern crate ocmapgen;

use clap::{Arg, App};
use ocmapgen::{Group, MaterialMap, TextureMap, MapGen};

use std::fs::File;
use std::io;
use std::io::prelude::*;

fn main() {
    let matches = App::new("ocmapgen")
        .version(env!("CARGO_PKG_VERSION"))
        .arg(Arg::with_name("root")
             .short("r").long("root")
             .help("OpenClonk “planet” root directory (defaults to current directory)")
             .takes_value(true)
             .default_value("."))
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

    let root = Group::open(matches.value_of("root").unwrap(), false)
              .expect("could not open root group");
    let mapgen = MapGen::init();
    mapgen.set_root_group(&root)
          .expect("could not find Library_Map");
    let mut materials = Group::open_as_child(&root, "Material.ocg", false, false)
                        .expect("could not open Material.ocg");
    let mut material_map = MaterialMap::new();
    material_map.load(&mut materials)
                .expect("could not load materials");
    let mut texture_map = TextureMap::new();
    materials.rewind();
    texture_map.load_textures(&mut materials)
               .expect("could not load texture map");
    texture_map.load_map(&mut materials)
               .expect("could not load texture map");
    material_map.set_default_textures(&texture_map);
    let filename = matches.value_of("INPUT").unwrap();
    let source = read_file(filename)
                 .expect("could not read input file");
    let width = value_t!(matches.value_of("width"), u32)
                .expect("invalid width");
    let height = value_t!(matches.value_of("height"), u32)
                .expect("invalid height");
    let map = match filename.rsplit(".").next() {
        Some("c") => mapgen.render_script(filename, &source, &material_map, &texture_map, width, height),
        Some("txt") => mapgen.render_landscape(filename, &source, "", &material_map, &texture_map, width, height),
        _ => panic!("invalid input file")
    }.expect("map rendering failed");
    map.save(matches.value_of("OUTPUT").unwrap())
       .expect("writing output image failed");

    //for i in 0 .. 256 {
    //    println!("tex {} = {:?} / {:?}", i, texture_map.get_texture_name(i), texture_map.get_material_name(i));
    //}

}

fn read_file(filename: &str) -> io::Result<String> {
    let mut file = File::open(filename)?;
    let mut contents = String::new();
    file.read_to_string(&mut contents)?;
    Ok(contents)
}
