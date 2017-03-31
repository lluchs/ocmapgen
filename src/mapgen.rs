use ffi::*;
use errors::*;
use group::Group;
use mattex::{MaterialMap, TextureMap};
use scenpar::Scenpar;
use super::Handle;

use std::ffi::{CStr, CString};
use std::slice;
use std::cell::RefCell;
use std::ptr;
use image::{self, ImageBuffer, RgbImage};

pub struct MapGen {
}

struct MapGenHandle<'a> {
    handle: *mut C4MapgenHandle,
    material_map: &'a MaterialMap,
    texture_map: &'a TextureMap,
}

impl MapGen {
    /// Initializes the map generator.
    ///
    /// Note: Will break if you create more than one instance at the same time.
    pub fn init() -> Result<MapGen> {
        unsafe {
            c4_mapgen_handle_init_script_engine();
        }
        let mut mapgen = MapGen {};
        let compat_source = include_str!("StandaloneCompat.c");
        mapgen.load_script("StandaloneCompat.c", compat_source)?;
        Ok(mapgen)
    }

    /// Sets root group to look up Library_Map.
    pub fn set_root_group(&self, group: &Group) -> Result<()> {
        let objects = Group::open_as_child(&group, "Objects.ocd", false, false)?;
        let libraries = Group::open_as_child(&objects, "Libraries.ocd", false, false)?;
        let map = Group::open_as_child(&libraries, "Map.ocd", false, false)?;
        unsafe { c4_mapgen_handle_set_map_library(map.handle()); }
        Ok(())
    }

    /// Load a System.ocg group.
    pub fn load_system(&mut self, group: &Group) -> Result<()> {
        unsafe { c4_mapgen_handle_load_system(group.handle()); }
        Ok(())
    }

    /// Load a script file.
    pub fn load_script(&mut self, filename: &str, source: &str) -> Result<()> {
        unsafe {
            c4_mapgen_handle_load_script(
                CString::new(filename).unwrap().as_ptr(),
                CString::new(source).unwrap().as_ptr());
        }
        Ok(())
    }

    pub fn render_landscape(&self, filename: &str, source: &str, script_path: &str, material_map: &MaterialMap, texture_map: &TextureMap, map_width: u32, map_height: u32) -> Result<RgbImage> {
        let mapgen = unsafe {
            MapGenHandle {
                handle: c4_mapgen_handle_new(
                            CString::new(filename).unwrap().as_ptr(),
                            CString::new(source).unwrap().as_ptr(),
                            CString::new(script_path).unwrap().as_ptr(),
                            material_map.handle(),
                            texture_map.handle(),
                            map_width,
                            map_height
                        ),
                material_map: material_map,
                texture_map: texture_map,
            }
        };
        mapgen.error()?;
        Ok(mapgen.get_map())
    }

    pub fn render_script(&self, filename: &str, source: &str, scenpar: Option<&Scenpar>, material_map: &MaterialMap, texture_map: &TextureMap, map_width: u32, map_height: u32) -> Result<RgbImage> {
        let mapgen = unsafe {
            MapGenHandle {
                handle: c4_mapgen_handle_new_script(
                            CString::new(filename).unwrap().as_ptr(),
                            CString::new(source).unwrap().as_ptr(),
                            scenpar.map(|s| s.handle()).unwrap_or_else(|| ptr::null_mut()),
                            material_map.handle(),
                            texture_map.handle(),
                            map_width,
                            map_height
                        ),
                material_map: material_map,
                texture_map: texture_map,
            }
        };
        mapgen.error()?;
        Ok(mapgen.get_map())
    }
}

impl Drop for MapGen {
    fn drop(&mut self) {
        unsafe {
            c4_mapgen_handle_deinit_script_engine();
        }
    }
}

impl<'a> MapGenHandle<'a> {
    pub fn error(&self) -> Result<()> {
        unsafe {
            let error_message = c4_mapgen_handle_get_error(self.handle);
            if error_message.is_null() {
                Ok(())
            } else {
                bail!(ErrorKind::MapGen(CStr::from_ptr(error_message).to_string_lossy().into_owned()));
            }
        }
    }

    pub fn width(&self) -> u32 {
        unsafe {
            c4_mapgen_handle_get_width(self.handle)
        }
    }

    pub fn height(&self) -> u32 {
        unsafe {
            c4_mapgen_handle_get_height(self.handle)
        }
    }

    pub fn rowstride(&self) -> u32 {
        unsafe {
            c4_mapgen_handle_get_rowstride(self.handle)
        }
    }

    pub fn get_map(&self) -> RgbImage {
        let width = self.width();
        let height = self.height();
        let rowstride = self.rowstride();
        let data: &[u8] = unsafe { slice::from_raw_parts(c4_mapgen_handle_get_map(self.handle), (width * height) as usize) };
        let mat_colors = RefCell::new(vec![None; 256]);
        ImageBuffer::from_fn(width, height, move |x, y| {
            let mut mat_colors = mat_colors.borrow_mut();
            let mat_idx = data[(x + y * rowstride) as usize];
            if let Some(color) = mat_colors[mat_idx as usize] {
                color
            } else {
                let color = self.get_mat_color(mat_idx);
                mat_colors[mat_idx as usize] = Some(color);
                color
            }
        })
    }

    fn get_mat_color(&self, mat_idx: u8) -> image::Rgb<u8> {
        const SKY: image::Rgb<u8> = image::Rgb { data: [100, 100, 255] };
        if mat_idx == 0 {
            SKY
        } else {
            if let Some(texture_name) = self.texture_map.get_texture_name(mat_idx as u32) {
                // Comment powered by mape source:
                /* When the texture is animated, the texture name consists of more than
                 * one texture, separated with a '-' character. In this case, we simply
                 * use the first one for display. */
                let texture_name = texture_name.split("-").next().unwrap();
                let texture_name = if !self.texture_map.texture_table.contains_key(&texture_name.to_lowercase()) {
                    match self.texture_map.get_material_name(mat_idx as u32)
                              .and_then(|name| self.material_map.get_material_by_name(&name))
                              .and_then(|mat| Some(mat.texture_overlay())) {
                        Some(name) => name,
                        None => { return SKY; }
                    }
                } else { texture_name.to_owned() };
                let color = self.texture_map.get_average_texture_color(&texture_name);
                image::Rgb { data: [(color & 0xff) as u8,
                                    ((color >> 8) & 0xff) as u8,
                                    ((color >> 16) & 0xff) as u8] }
            } else {
                println!("no texture name for {}", mat_idx);
                // This really shouldn't happen and I think mape doesn't even handle this case.
                SKY
            }
        }
    }
}

impl<'a> Drop for MapGenHandle<'a> {
    fn drop(&mut self) {
        unsafe {
            c4_mapgen_handle_free(self.handle);
        }
    }
}
