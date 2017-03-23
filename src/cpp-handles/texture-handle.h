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

#ifndef INC_MAPE_C4_TEXTURE_HANDLE_H
#define INC_MAPE_C4_TEXTURE_HANDLE_H


#include "mape/cpp-handles/group-handle.h"


typedef struct _C4TextureMapHandle C4TextureMapHandle;

C4TextureMapHandle* c4_texture_map_handle_new(void);
void c4_texture_map_handle_free(C4TextureMapHandle* texture_map);

unsigned int c4_texture_map_handle_load_map(C4TextureMapHandle* texture_map, C4GroupHandle* group, const char* entry_name, bool* overload_materials, bool* overload_textures);
bool c4_texture_map_handle_add_texture(C4TextureMapHandle* texture_map, const char* texture, unsigned int32 avg_color);
const char* c4_texture_map_handle_get_texture(C4TextureMapHandle* texture_map, unsigned int index);

const char* c4_texture_handle_get_entry_material_name(C4TextureMapHandle* texture_map, unsigned int index);
const char* c4_texture_handle_get_entry_texture_name(C4TextureMapHandle* texture_map, unsigned int index);

unsigned int32 c4_texture_handle_get_average_texture_color(C4TextureMapHandle* texture_map, const char* name);


#endif /* INC_MAPE_C4_TEXTURE_HANDLE_H */
