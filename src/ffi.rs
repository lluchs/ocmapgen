/* automatically generated by rust-bindgen */

#![allow(dead_code)]

#[repr(C)]
#[derive(Debug, Copy, Clone)]
pub struct _C4GroupHandle([u8; 0]);
pub type C4GroupHandle = _C4GroupHandle;
extern "C" {
    pub fn c4_group_handle_new() -> *mut C4GroupHandle;
}
extern "C" {
    pub fn c4_group_handle_free(handle: *mut C4GroupHandle);
}
extern "C" {
    pub fn c4_group_handle_get_error(handle: *mut C4GroupHandle)
     -> *const ::std::os::raw::c_char;
}
extern "C" {
    pub fn c4_group_handle_open(handle: *mut C4GroupHandle,
                                path: *const ::std::os::raw::c_char,
                                create: bool) -> bool;
}
extern "C" {
    pub fn c4_group_handle_open_as_child(handle: *mut C4GroupHandle,
                                         mother: *mut C4GroupHandle,
                                         name: *const ::std::os::raw::c_char,
                                         exclusive: bool, create: bool)
     -> bool;
}
extern "C" {
    pub fn c4_group_handle_get_name(handle: *mut C4GroupHandle)
     -> *const ::std::os::raw::c_char;
}
extern "C" {
    pub fn c4_group_handle_get_full_name(handle: *mut C4GroupHandle)
     -> *mut ::std::os::raw::c_char;
}
extern "C" {
    pub fn c4_group_handle_reset_search(handle: *mut C4GroupHandle);
}
extern "C" {
    pub fn c4_group_handle_find_next_entry(handle: *mut C4GroupHandle,
                                           wildcard:
                                               *const ::std::os::raw::c_char,
                                           size: *mut usize,
                                           filename:
                                               *mut ::std::os::raw::c_char,
                                           start_at_filename: bool) -> bool;
}
extern "C" {
    pub fn c4_group_handle_access_next_entry(handle: *mut C4GroupHandle,
                                             wildcard:
                                                 *const ::std::os::raw::c_char,
                                             size: *mut usize,
                                             filename:
                                                 *mut ::std::os::raw::c_char,
                                             start_at_filename: bool) -> bool;
}
extern "C" {
    pub fn c4_group_handle_access_entry(handle: *mut C4GroupHandle,
                                        wildcard:
                                            *const ::std::os::raw::c_char,
                                        size: *mut usize,
                                        filename: *mut ::std::os::raw::c_char,
                                        needs_to_be_a_group: bool) -> bool;
}
extern "C" {
    pub fn c4_group_handle_accessed_entry_size(handle: *mut C4GroupHandle)
     -> usize;
}
extern "C" {
    pub fn c4_group_handle_read(handle: *mut C4GroupHandle,
                                buffer: *mut ::std::os::raw::c_void,
                                size: usize) -> bool;
}
extern "C" {
    pub fn c4_group_handle_is_folder(handle: *mut C4GroupHandle) -> bool;
}
#[repr(C)]
#[derive(Debug, Copy, Clone)]
pub struct _C4MapgenHandle([u8; 0]);
pub type C4MapgenHandle = _C4MapgenHandle;
extern "C" {
    pub fn c4_log_handle_clear();
}
extern "C" {
    pub fn c4_log_handle_get_log_messages() -> *const ::std::os::raw::c_char;
}
extern "C" {
    pub fn c4_log_handle_get_n_log_messages() -> ::std::os::raw::c_uint;
}
extern "C" {
    pub fn c4_log_handle_has_error() -> bool;
}
#[repr(C)]
#[derive(Debug, Copy, Clone)]
pub struct _C4TextureMapHandle([u8; 0]);
pub type C4TextureMapHandle = _C4TextureMapHandle;
extern "C" {
    pub fn c4_texture_map_handle_new() -> *mut C4TextureMapHandle;
}
extern "C" {
    pub fn c4_texture_map_handle_free(texture_map: *mut C4TextureMapHandle);
}
extern "C" {
    pub fn c4_texture_map_handle_load_map(texture_map:
                                              *mut C4TextureMapHandle,
                                          group: *mut C4GroupHandle,
                                          entry_name:
                                              *const ::std::os::raw::c_char,
                                          overload_materials: *mut bool,
                                          overload_textures: *mut bool)
     -> ::std::os::raw::c_uint;
}
extern "C" {
    pub fn c4_texture_map_handle_add_texture(texture_map:
                                                 *mut C4TextureMapHandle,
                                             texture:
                                                 *const ::std::os::raw::c_char,
                                             avg_color: u32) -> bool;
}
extern "C" {
    pub fn c4_texture_map_handle_get_texture(texture_map:
                                                 *mut C4TextureMapHandle,
                                             index: ::std::os::raw::c_uint)
     -> *const ::std::os::raw::c_char;
}
extern "C" {
    pub fn c4_texture_handle_get_entry_material_name(texture_map:
                                                         *mut C4TextureMapHandle,
                                                     index:
                                                         ::std::os::raw::c_uint)
     -> *const ::std::os::raw::c_char;
}
extern "C" {
    pub fn c4_texture_handle_get_entry_texture_name(texture_map:
                                                        *mut C4TextureMapHandle,
                                                    index:
                                                        ::std::os::raw::c_uint)
     -> *const ::std::os::raw::c_char;
}
extern "C" {
    pub fn c4_texture_handle_get_average_texture_color(texture_map:
                                                           *mut C4TextureMapHandle,
                                                       name:
                                                           *const ::std::os::raw::c_char)
     -> u32;
}
#[repr(C)]
#[derive(Debug, Copy, Clone)]
pub struct _C4MaterialHandle([u8; 0]);
pub type C4MaterialHandle = _C4MaterialHandle;
#[repr(C)]
#[derive(Debug, Copy, Clone)]
pub struct _C4MaterialMapHandle([u8; 0]);
pub type C4MaterialMapHandle = _C4MaterialMapHandle;
extern "C" {
    pub fn c4_material_map_handle_new() -> *mut C4MaterialMapHandle;
}
extern "C" {
    pub fn c4_material_map_handle_free(material_map:
                                           *mut C4MaterialMapHandle);
}
extern "C" {
    pub fn c4_material_map_handle_load(material_map: *mut C4MaterialMapHandle,
                                       group: *mut C4GroupHandle)
     -> ::std::os::raw::c_uint;
}
extern "C" {
    pub fn c4_material_map_crossmap_materials(material_map:
                                                  *mut C4MaterialMapHandle,
                                              texture_map:
                                                  *mut C4TextureMapHandle);
}
extern "C" {
    pub fn c4_material_map_handle_get_num(material_map:
                                              *mut C4MaterialMapHandle)
     -> ::std::os::raw::c_uint;
}
extern "C" {
    pub fn c4_material_map_handle_get_material(material_map:
                                                   *mut C4MaterialMapHandle,
                                               index: ::std::os::raw::c_uint)
     -> *mut C4MaterialHandle;
}
extern "C" {
    pub fn c4_material_handle_get_name(material: *mut C4MaterialHandle)
     -> *const ::std::os::raw::c_char;
}
extern "C" {
    pub fn c4_material_handle_get_texture_overlay(material:
                                                      *mut C4MaterialHandle)
     -> *const ::std::os::raw::c_char;
}
#[repr(C)]
#[derive(Debug, Copy, Clone)]
pub struct _C4ScenparHandle([u8; 0]);
pub type C4ScenparHandle = _C4ScenparHandle;
extern "C" {
    pub fn c4_scenpar_handle_new() -> *mut C4ScenparHandle;
}
extern "C" {
    pub fn c4_scenpar_handle_free(handle: *mut C4ScenparHandle);
}
extern "C" {
    pub fn c4_scenpar_handle_load(handle: *mut C4ScenparHandle,
                                  group: *mut C4GroupHandle) -> bool;
}
extern "C" {
    pub fn c4_scenpar_handle_get_value_by_id(handle: *mut C4ScenparHandle,
                                             id:
                                                 *const ::std::os::raw::c_char,
                                             default_value: i32) -> i32;
}
extern "C" {
    pub fn c4_scenpar_handle_set_value(handle: *mut C4ScenparHandle,
                                       id: *const ::std::os::raw::c_char,
                                       value: i32, only_if_larger: bool);
}
extern "C" {
    pub fn c4_mapgen_handle_init_script_engine();
}
extern "C" {
    pub fn c4_mapgen_handle_deinit_script_engine();
}
extern "C" {
    pub fn c4_mapgen_handle_set_map_library(group_handle: *mut C4GroupHandle);
}
extern "C" {
    pub fn c4_mapgen_handle_load_system(group_handle: *mut C4GroupHandle);
}
extern "C" {
    pub fn c4_mapgen_handle_load_script(filename:
                                            *const ::std::os::raw::c_char,
                                        source:
                                            *const ::std::os::raw::c_char);
}
extern "C" {
    pub fn c4_mapgen_handle_set_startup_player_count(count: i32);
}
extern "C" {
    pub fn c4_mapgen_handle_set_startup_team_count(count: i32);
}
extern "C" {
    pub fn c4_mapgen_handle_new_script(filename:
                                           *const ::std::os::raw::c_char,
                                       source: *const ::std::os::raw::c_char,
                                       scenpar: *mut C4ScenparHandle,
                                       material_map: *mut C4MaterialMapHandle,
                                       texture_map: *mut C4TextureMapHandle,
                                       map_width: ::std::os::raw::c_uint,
                                       map_height: ::std::os::raw::c_uint)
     -> *mut C4MapgenHandle;
}
extern "C" {
    pub fn c4_mapgen_handle_new(filename: *const ::std::os::raw::c_char,
                                source: *const ::std::os::raw::c_char,
                                script_path: *const ::std::os::raw::c_char,
                                material_map: *mut C4MaterialMapHandle,
                                texture_map: *mut C4TextureMapHandle,
                                map_width: ::std::os::raw::c_uint,
                                map_height: ::std::os::raw::c_uint)
     -> *mut C4MapgenHandle;
}
extern "C" {
    pub fn c4_mapgen_handle_free(mapgen: *mut C4MapgenHandle);
}
extern "C" {
    pub fn c4_mapgen_handle_get_map(mapgen: *mut C4MapgenHandle)
     -> *const ::std::os::raw::c_uchar;
}
extern "C" {
    pub fn c4_mapgen_handle_get_bg(mapgen: *mut C4MapgenHandle)
     -> *const ::std::os::raw::c_uchar;
}
extern "C" {
    pub fn c4_mapgen_handle_save_map(mapgen: *mut C4MapgenHandle,
                                     path: *const ::std::os::raw::c_char,
                                     material_map: *mut C4MaterialMapHandle,
                                     texture_map: *mut C4TextureMapHandle)
     -> bool;
}
extern "C" {
    pub fn c4_mapgen_handle_save_bg(mapgen: *mut C4MapgenHandle,
                                    path: *const ::std::os::raw::c_char,
                                    material_map: *mut C4MaterialMapHandle,
                                    texture_map: *mut C4TextureMapHandle)
     -> bool;
}
extern "C" {
    pub fn c4_mapgen_handle_get_width(mapgen: *mut C4MapgenHandle)
     -> ::std::os::raw::c_uint;
}
extern "C" {
    pub fn c4_mapgen_handle_get_height(mapgen: *mut C4MapgenHandle)
     -> ::std::os::raw::c_uint;
}
extern "C" {
    pub fn c4_mapgen_handle_get_rowstride(mapgen: *mut C4MapgenHandle)
     -> ::std::os::raw::c_uint;
}
extern "C" {
    pub fn c4_mapgen_handle_get_error(mapgen: *mut C4MapgenHandle)
     -> *const ::std::os::raw::c_char;
}
extern "C" {
    pub fn c4_mapgen_handle_get_warnings(mapgen: *mut C4MapgenHandle)
     -> *const ::std::os::raw::c_char;
}
extern "C" {
    pub fn c4_mapgen_handle_get_script_output(mapgen: *mut C4MapgenHandle)
     -> *const ::std::os::raw::c_char;
}
extern "C" {
    pub fn c4_random_handle_seed(seed: ::std::os::raw::c_uint);
}
extern "C" {
    pub fn c4_version_get() -> *const ::std::os::raw::c_char;
}
