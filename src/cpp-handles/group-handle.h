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

#ifndef INC_MAPE_C4_GROUP_HANDLE_H
#define INC_MAPE_C4_GROUP_HANDLE_H

#ifdef __cplusplus
extern "C" {
#endif

typedef struct _C4GroupHandle C4GroupHandle;

C4GroupHandle* c4_group_handle_new(void);
void c4_group_handle_free(C4GroupHandle* handle);

const char* c4_group_handle_get_error(C4GroupHandle* handle);

bool c4_group_handle_open(C4GroupHandle* handle, const char* path, bool create);
bool c4_group_handle_open_as_child(C4GroupHandle* handle, C4GroupHandle* mother, const char* name, bool exclusive, bool create);

const char* c4_group_handle_get_name(C4GroupHandle* handle);
char* c4_group_handle_get_full_name(C4GroupHandle* handle);

void c4_group_handle_reset_search(C4GroupHandle* handle);

bool c4_group_handle_find_next_entry(C4GroupHandle* handle, const char* wildcard, size_t* size, char* filename, bool start_at_filename);
bool c4_group_handle_access_next_entry(C4GroupHandle* handle, const char* wildcard, size_t* size, char* filename, bool start_at_filename);
bool c4_group_handle_access_entry(C4GroupHandle* handle, const char* wildcard, size_t* size, char* filename, bool needs_to_be_a_group);
size_t c4_group_handle_accessed_entry_size(C4GroupHandle* handle);
bool c4_group_handle_read(C4GroupHandle* handle, void* buffer, size_t size);
bool c4_group_handle_is_folder(C4GroupHandle* handle);

#ifdef __cplusplus
}
#endif

#endif /* INC_MAPE_C4_GROUP_HANDLE_H */
