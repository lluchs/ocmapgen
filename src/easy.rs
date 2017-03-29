use super::{Group, MaterialMap, TextureMap, MapGen};
use super::errors::*;
use image;

use std::io::prelude::*;
use std::io;
use std::fs::File;
use std::path::Path;

/// Provides an easy-to-use API for rendering mape maps.
pub struct Easy {
    pub material_map: MaterialMap,
    pub texture_map: TextureMap,
    pub mapgen: MapGen,
}

impl Easy {
    /// Create and initialize the map generator.
    ///
    /// Note that due to global stage in the generator, there can be only instance at any time.
    pub fn new() -> Easy {
        Easy {
            mapgen: MapGen::init(),
            material_map: MaterialMap::new(),
            texture_map: TextureMap::new(),
        }
    }

    /// Sets the base path for loading materials and scripts.
    ///
    /// The path has to be the OpenClonk base directory (containing Material.ocg, System.ocg, Objects.ocd) or a subdirectory.
    pub fn set_base_path<P: AsRef<Path>>(&mut self, path: P) -> Result<()> {
        let mut path = path.as_ref().canonicalize()?;
        let mut base_group = None;
        let mut overloaded_groups = Vec::new();
        let mut search_for_overload = false;
        loop {
            // Search closest Material.ocg as well as the base directory with System.ocg and
            // Objects.ocd.
            if base_group.is_none() || search_for_overload {
                let material_ocg = path.join("Material.ocg");
                if material_ocg.exists() {
                    let material_group = Group::open(material_ocg.to_str().unwrap(), false)?;
                    let result = self.texture_map.load_map(&material_group)?;
                    search_for_overload = result.overload_materials || result.overload_textures;
                    if base_group.is_none() {
                        base_group = Some(material_group);
                    } else {
                        overloaded_groups.push(material_group);
                    }
                }
            }

            let system_ocg = path.join("System.ocg");
            let objects_ocd = path.join("Objects.ocd");
            if system_ocg.exists() && objects_ocd.exists() {
                // Done, we found the root.
                let root_group = Group::open(path.to_str().unwrap(), false)?;
                self.mapgen.set_root_group(&root_group)?;
                // TODO: Load scripts from System.ocg?
                break;
            }

            if !path.pop() {
                bail!("couldn't find base path");
            }
        }
        let mut base_group = match base_group {
            Some(g) => g,
            None => bail!("couldn't find Material.ocg")
        };
        self.material_map.load(&mut base_group)?;
        for mut overloaded_group in &mut overloaded_groups {
            self.material_map.load(&mut overloaded_group)?;
        }
        base_group.rewind();
        self.texture_map.load_textures(&mut base_group)?;
        for mut overloaded_group in &mut overloaded_groups {
            self.texture_map.load_textures(&mut overloaded_group)?;
        }
        // Load the texture map a second time, now with textures.
        let result = self.texture_map.load_map(&mut base_group)?;
        if result.num_loaded == 0 {
            bail!(ErrorKind::NothingLoaded);
        }
        self.material_map.set_default_textures(&self.texture_map);

        Ok(())
    }

    /// Entry point for rendering the map.
    pub fn build(&self) -> RenderConfig {
        RenderConfig {
            easy: self,
            map_type: None,
            filename: None,
            source: None,
            width: 200,
            height: 200,
            algo_script_path: None,
        }
    }
}

#[derive(Clone, Copy)]
pub enum MapType {
    LandscapeTxt,
    MapC,
}

/// Render configuration builder.
pub struct RenderConfig<'a> {
    easy: &'a Easy,
    map_type: Option<MapType>,
    filename: Option<String>,
    source: Option<String>,
    width: u32,
    height: u32,
    algo_script_path: Option<String>,
}

impl<'a> RenderConfig<'a> {
    /// Set the map type. Per default, this is inferred from the file name.
    pub fn map_type(&'a mut self, map_type: MapType) -> &'a mut RenderConfig {
        self.map_type = Some(map_type);
        self
    }

    /// Set the file name. The file name is used in error messages.
    pub fn filename(&'a mut self, filename: &str) -> &'a mut RenderConfig {
        self.filename = Some(filename.into());
        self
    }

    /// Set the map source code. If unset, tries to read the file path set using `filename`.
    pub fn source(&'a mut self, source: &str) -> &'a mut RenderConfig {
        self.source = Some(source.into());
        self
    }

    /// Set the output map width. Note that script Map.c can override the map size.
    pub fn width(&'a mut self, width: u32) -> &'a mut RenderConfig {
        self.width = width;
        self
    }

    /// Set the output map height. Note that script Map.c can override the map size.
    pub fn height(&'a mut self, height: u32) -> &'a mut RenderConfig {
        self.height = height;
        self
    }

    /// Sets the script path for Algo=Script Landscape.txt maps.
    pub fn algo_script_path(&'a mut self, algo_script_path: &str) -> &'a mut RenderConfig {
        self.algo_script_path = Some(algo_script_path.into());
        self
    }

    /// Renders the map!
    pub fn render(&self) -> Result<image::RgbImage> {
        let map_type = match self.map_type {
            Some(t) => t,
            None => self.autodetect_map_type()?
        };
        let filename = self.filename.as_ref().map(|f| f.as_str()).unwrap_or("<no filename>");
        let source = match self.source {
            Some(ref s) => s.clone(),
            None => if self.filename.is_some() {
                        read_file(filename)
                            .chain_err(|| "couldn't read input file")?
                    } else {
                        bail!("neither source nor filename set")
                    }
        };
        let algo_script_path = || self.algo_script_path.as_ref().map(|f| f.as_str()).unwrap_or("");
        match map_type {
            MapType::MapC => self.easy.mapgen.render_script(filename, &source, &self.easy.material_map, &self.easy.texture_map, self.width, self.height),
            MapType::LandscapeTxt => self.easy.mapgen.render_landscape(filename, &source, algo_script_path(), &self.easy.material_map, &self.easy.texture_map, self.width, self.height),
        }
    }

    fn autodetect_map_type(&self) -> Result<MapType> {
        match self.filename {
            Some(ref filename) => match filename.rsplit(".").next() {
                                  Some("c")   => Ok(MapType::MapC),
                                  Some("txt") => Ok(MapType::LandscapeTxt),
                                  _ => bail!(ErrorKind::MapTypeDetectionFailed(filename.clone()))
                              },
            None => bail!(ErrorKind::MapTypeDetectionFailed("<no filename>".into()))
        }
    }
}

fn read_file(path: &str) -> io::Result<String> {
    let mut contents = String::new();
    File::open(path)?.read_to_string(&mut contents)?;
    Ok(contents)
}
