use crate::ffi::*;
use crate::errors::*;
use crate::Handle;
use crate::group::Group;
use std::ffi::{CStr, CString};
use std::collections::HashMap;

use image::{self, DynamicImage};
use error_chain::bail;

pub struct MaterialMap {
    handle: *mut C4MaterialMapHandle,
}

impl MaterialMap {
    pub fn new() -> MaterialMap {
        unsafe {
            MaterialMap {
                handle: c4_material_map_handle_new(),
            }
        }
    }

    /// Returns the number of materials loaded.
    pub fn load(&mut self, group: &Group) -> Result<u32> {
        unsafe {
            let result = c4_material_map_handle_load(self.handle, group.handle());
            Ok(result)
        }
    }

    pub fn set_default_textures(&self, texture_map: &TextureMap) {
        unsafe {
            c4_material_map_crossmap_materials(self.handle, texture_map.handle);
        }
    }

    pub fn get_material_by_name(&self, name: &str) -> Option<Material> {
        let lname = name.to_lowercase();
        (0 .. unsafe { c4_material_map_handle_get_num(self.handle) })
            .filter_map(|i| self.get_material_by_index(i))
            .find(|ref mat| mat.name().to_lowercase() == lname)
    }

    pub fn get_material_by_index(&self, index: u32) -> Option<Material> {
        let handle = unsafe { c4_material_map_handle_get_material(self.handle, index) };
        if handle.is_null() {
            None
        } else {
            Some(Material { handle: handle })
        }
    }
}

impl Drop for MaterialMap {
    fn drop(&mut self) {
        unsafe {
            c4_material_map_handle_free(self.handle);
        }
    }
}

impl Handle<C4MaterialMapHandle> for MaterialMap {
    fn handle(&self) -> *mut C4MaterialMapHandle {
        self.handle
    }
}

pub struct Material {
    handle: *mut C4MaterialHandle,
}

impl Material {
    pub fn name(&self) -> String {
        unsafe {
            let result = c4_material_handle_get_name(self.handle);
            CStr::from_ptr(result)
                .to_string_lossy()
                .into_owned()
        }
    }

    pub fn texture_overlay(&self) -> String {
        unsafe {
            let result = c4_material_handle_get_texture_overlay(self.handle);
            CStr::from_ptr(result)
                .to_string_lossy()
                .into_owned()
        }
    }
}

pub struct TextureMap {
    handle: *mut C4TextureMapHandle,
    pub texture_table: HashMap<String, DynamicImage>,
}

pub struct LoadTextureMapResult {
    pub num_loaded: u32,
    pub overload_materials: bool,
    pub overload_textures: bool,
}

impl TextureMap {
    pub fn new() -> TextureMap {
        unsafe {
            TextureMap {
                handle: c4_texture_map_handle_new(),
                texture_table: HashMap::new(),
            }
        }
    }

    /// Loads texture mappings from TexMap.txt.
    /// Returns the number of textures loaded as well as whether the Material.ocg overloads the
    /// base one.
    pub fn load_map(&mut self, group: &Group) -> Result<LoadTextureMapResult> {
        unsafe {
            let mut overload_materials = false;
            let mut overload_textures = false;
            let result = c4_texture_map_handle_load_map(self.handle,
                                                        group.handle(),
                                                        CString::new("TexMap.txt").unwrap().as_ptr(),
                                                        &mut overload_materials,
                                                        &mut overload_textures);
            // Currently cannot fail.
            Ok(LoadTextureMapResult {
                num_loaded: result,
                overload_materials: overload_materials,
                overload_textures: overload_textures,
            })
        }
    }

    pub fn load_textures(&mut self, group: &mut Group) -> Result<()> {
        group.rewind();
        const SUFFIXES: &'static [&'static str] = &[
            ".png", ".jpg", ".jpeg", ".bmp",
            ".PNG", ".JPG", ".JPEG", ".BMP"
        ];
        while let Some(name) = group.next() {
            if let Some(suffix) = SUFFIXES.iter().find(|&suffix| name.ends_with(suffix)) {
                let texname = &name[0 .. (name.len() - suffix.len())];
                let lowercase_name = texname.to_lowercase();
                if self.texture_table.get(&lowercase_name).is_none() {
                    let data = group.load_entry(&name)?;
                    let image = image::load_from_memory(&data)
                                .chain_err(|| "could not load texture image")?;
                    let avgcolor = get_average_color(&image);
                    unsafe {
                        let s = CString::new(texname).unwrap();
                        if !c4_texture_map_handle_add_texture(self.handle, s.as_ptr(), avgcolor) {
                            bail!("failed adding texture {}", texname);
                        }
                    }
                    self.texture_table.insert(lowercase_name.into(), image);
                }
            }
        }
        Ok(())
    }

    pub fn get_texture_name(&self, index: u32) -> Option<String> {
        unsafe {
            let result = c4_texture_handle_get_entry_texture_name(self.handle, index);
            if result.is_null() {
                None
            } else {
                Some(CStr::from_ptr(result)
                        .to_string_lossy()
                        .into_owned())
            }
        }
    }

    pub fn get_material_name(&self, index: u32) -> Option<String> {
        unsafe {
            let result = c4_texture_handle_get_entry_material_name(self.handle, index);
            if result.is_null() {
                None
            } else {
                Some(CStr::from_ptr(result)
                        .to_string_lossy()
                        .into_owned())
            }
        }
    }

    pub fn get_average_texture_color(&self, name: &str) -> u32 {
        unsafe {
            c4_texture_handle_get_average_texture_color(self.handle, CString::new(name).unwrap().as_ptr())
        }
    }
}

impl Drop for TextureMap {
    fn drop(&mut self) {
        unsafe {
            c4_texture_map_handle_free(self.handle);
        }
    }
}

impl Handle<C4TextureMapHandle> for TextureMap {
    fn handle(&self) -> *mut C4TextureMapHandle {
        self.handle
    }
}

fn get_average_color<T>(image: &T) -> u32 
        where T: image::GenericImage<Pixel = image::Rgba<u8>> {
    let accum = image.pixels().fold((0f64, 0f64, 0f64, 0f64),
                                    |acc, (_x, _y, px)| (acc.0 + px.0[0] as f64,
                                                         acc.1 + px.0[1] as f64,
                                                         acc.2 + px.0[2] as f64,
                                                         acc.3 + px.0[3] as f64));
    let size: f64 = (image.width() * image.height()) as f64;
    // OC color format is 0xaarrggbb
       ((accum.2 / size + 0.5) as u32)
    | (((accum.1 / size + 0.5) as u32) << 8)
    | (((accum.0 / size + 0.5) as u32) << 16)
    | (((accum.3 / size + 0.5) as u32) << 24)
}
