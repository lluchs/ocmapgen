/*
 * ocmapgen
 *
 * Copyright (c) 2017, Lukas Werling
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

#ifndef INC_OCMAPGEN_C4_SCENPAR_HANDLE_H
#define INC_OCMAPGEN_C4_SCENPAR_HANDLE_H

#include "group-handle.h"

#ifdef __cplusplus
extern "C" {
#endif

typedef struct _C4ScenparHandle C4ScenparHandle;

C4ScenparHandle* c4_scenpar_handle_new(void);
void c4_scenpar_handle_free(C4ScenparHandle* handle);

bool c4_scenpar_handle_load(C4ScenparHandle* handle, C4GroupHandle* group);

int32_t c4_scenpar_handle_get_value_by_id(C4ScenparHandle* handle, const char* id, int32_t default_value);
void c4_scenpar_handle_set_value(C4ScenparHandle* handle, const char* id, int32_t value, bool only_if_larger);

#ifdef __cplusplus
}
#endif

#endif /* INC_OCMAPGEN_C4_SCENPAR_HANDLE_H */
