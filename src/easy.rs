use crate::{Group, MaterialMap, TextureMap, Scenpar, MapGen, MapGenHandle};
use crate::errors::*;

use std::io::prelude::*;
use std::io;
use std::fs::File;
use std::path::Path;

use error_chain::bail;
use regex::bytes::Regex;

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
    pub fn new() -> Result<Easy> {
        let easy = Easy {
            mapgen: MapGen::init()?,
            material_map: MaterialMap::new(),
            texture_map: TextureMap::new(),
        };
        Ok(easy)
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
                // Load the top-level System.ocg
                self.load_system(&system_ocg)?;
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
        let mut mat_sum = 0;
        mat_sum += self.material_map.load(&mut base_group)?;
        for mut overloaded_group in &mut overloaded_groups {
            mat_sum += self.material_map.load(&mut overloaded_group)?;
        }
        if mat_sum == 0 {
            bail!(ErrorKind::NothingLoaded);
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

    fn load_system(&mut self, path: &Path) -> Result<()> {
        let system_group = Group::open(path.to_str().unwrap(), false)?;
        // Try to load player controls to generate CON_ constants.
        if let Ok(data) = system_group.load_entry("PlayerControls.txt") {
            let re = Regex::new(r"(?m)^\s*Identifier=(\w+)").unwrap();
            let mut script = String::new();
            for cap in re.captures_iter(&data) {
                // The regex will only match UTF-8, so from_utf8_lossy won't have to convert
                // anything. I don't think OC will accept special characters there anyways.
                script.push_str(&format!("static const CON_{} = 0;\n", String::from_utf8_lossy(&cap[1])));
            }
            self.mapgen.load_script("PlayerControlsCompat.c", &script)?;
        }
        self.mapgen.load_system(&system_group)
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
            scenpar: None,
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
    scenpar: Option<&'a Scenpar>,
}

impl<'a> RenderConfig<'a> {
    /// Set the map type. Per default, this is inferred from the file name.
    pub fn map_type<'b>(&'b mut self, map_type: MapType) -> &'b mut RenderConfig<'a> {
        self.map_type = Some(map_type);
        self
    }

    /// Set the file name. The file name is used in error messages.
    pub fn filename<'b>(&'b mut self, filename: &str) -> &'b mut RenderConfig<'a> {
        self.filename = Some(filename.into());
        self
    }

    /// Set the map source code. If unset, tries to read the file path set using `filename`.
    pub fn source<'b>(&'b mut self, source: &str) -> &'b mut RenderConfig<'a> {
        self.source = Some(source.into());
        self
    }

    /// Set the output map width. Note that script Map.c can override the map size.
    pub fn width<'b>(&'b mut self, width: u32) -> &'b mut RenderConfig<'a> {
        self.width = width;
        self
    }

    /// Set the output map height. Note that script Map.c can override the map size.
    pub fn height<'b>(&'b mut self, height: u32) -> &'b mut RenderConfig<'a> {
        self.height = height;
        self
    }

    /// Sets the script path for Algo=Script Landscape.txt maps.
    pub fn algo_script_path<'b>(&'b mut self, algo_script_path: &str) -> &'b mut RenderConfig<'a> {
        self.algo_script_path = Some(algo_script_path.into());
        self
    }

    /// Sets scenario parameters to load for script Map.c.
    pub fn scenpar<'b>(&'b mut self, scenpar: &'a Scenpar) -> &'b mut RenderConfig<'a> {
        self.scenpar = Some(scenpar);
        self
    }

    /// Renders the map!
    pub fn render(&self) -> Result<MapGenHandle> {
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
            MapType::MapC => self.easy.mapgen.render_script(filename,
                                                            &source,
                                                            self.scenpar.clone(),
                                                            &self.easy.material_map,
                                                            &self.easy.texture_map,
                                                            self.width,
                                                            self.height),
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

/// Tries to load a ParameterDefs.txt from the given directory.
///
/// Returns a NoParameterDefs error if there is no such file.
pub fn load_scenpar<P: AsRef<Path>>(path: P) -> Result<Scenpar> {
    let path = path.as_ref().canonicalize()?;
    if path.join("ParameterDefs.txt").exists() {
        let group = Group::open(&path.to_str().unwrap(), false)?;
        let mut scenpar = Scenpar::new();
        scenpar.load(&group)?;
        Ok(scenpar)
    } else {
        bail!(ErrorKind::NoParameterDefs)
    }
}
