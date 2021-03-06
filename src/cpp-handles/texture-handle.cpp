/*
 * mape - C4 Landscape.txt editor
 *
 * Copyright (c) 2005-2009, Armin Burgmeier
 *
 * Distributed under the terms of the ISC license; see accompanying file
 * "COPYING" for details.
 *
 * "Clonk" is a registered trademark of Matthes Bender, used with permission.
 * See accompanying file "TRADEMARK" for details.
 *
 * To redistribute this file separately, substitute the full license texts
 * for the above references.
 */

#include "C4Include.h"
#include "landscape/C4Texture.h"
#include "texture-handle.h"

#define TEXTURE_MAP_TO_HANDLE(texture_map) (reinterpret_cast<C4TextureMapHandle*>(texture_map))
#define HANDLE_TO_TEXTURE_MAP(handle) (reinterpret_cast<C4TextureMap*>(handle))

#define GROUP_TO_HANDLE(group) (reinterpret_cast<C4GroupHandle*>(group))
#define HANDLE_TO_GROUP(handle) (reinterpret_cast<C4Group*>(handle))

extern "C" {

C4TextureMapHandle* c4_texture_map_handle_new(void)
{
  // Simply return a pointer to the global texture map. This is a bit stupid,
  // but some functions in C4Landscape use the global texture map when looking
  // up textures. This should be changed to get rid of the global variable,
  //but yeah...
  C4TextureMap* map = &::TextureMap;
  map->Clear();
  map->Init();
  return TEXTURE_MAP_TO_HANDLE(map); //new C4TextureMap);
}

void c4_texture_map_handle_free(C4TextureMapHandle* texture_map)
{
  //delete HANDLE_TO_TEXTURE_MAP(texture_map);
}

unsigned int c4_texture_map_handle_load_map(C4TextureMapHandle* texture_map, C4GroupHandle* group, const char* entry_name, bool* overload_materials, bool* overload_textures)
{
  bool fOverloadMaterials = false;
  bool fOverloadTextures = false;
  uint32_t retval = HANDLE_TO_TEXTURE_MAP(texture_map)->LoadMap(*HANDLE_TO_GROUP(group), entry_name, &fOverloadMaterials, &fOverloadTextures);
  if(overload_materials) *overload_materials = fOverloadMaterials;
  if(overload_textures) *overload_textures = fOverloadTextures;
  return retval;
}

bool c4_texture_map_handle_add_texture(C4TextureMapHandle* texture_map, const char* texture, uint32_t avg_color)
{
  bool result = HANDLE_TO_TEXTURE_MAP(texture_map)->AddTexture(texture, nullptr);
  if(!result) return false;
  HANDLE_TO_TEXTURE_MAP(texture_map)->GetTexture(texture)->SetAverageColor(avg_color);
  return true;
}

const char* c4_texture_map_handle_get_texture(C4TextureMapHandle* texture_map, unsigned int index)
{
  return HANDLE_TO_TEXTURE_MAP(texture_map)->GetTexture(index);
}

uint32_t c4_texture_handle_get_average_texture_color(C4TextureMapHandle* texture_map, const char* name)
{
  return HANDLE_TO_TEXTURE_MAP(texture_map)->GetTexture(name)->GetAverageColor();
}

const char* c4_texture_handle_get_entry_material_name(C4TextureMapHandle* texture_map, unsigned int index)
{
  const C4TexMapEntry* entry = HANDLE_TO_TEXTURE_MAP(texture_map)->GetEntry(index);
  if(!entry) return nullptr;
  return entry->GetMaterialName();
}

const char* c4_texture_handle_get_entry_texture_name(C4TextureMapHandle* texture_map, unsigned int index)
{
  const C4TexMapEntry* entry = HANDLE_TO_TEXTURE_MAP(texture_map)->GetEntry(index);
  if(!entry) return nullptr;
  return entry->GetTextureName();
}

} /* extern "C" */
