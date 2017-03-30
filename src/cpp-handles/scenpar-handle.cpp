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

#include "C4Include.h"
#include "player/C4ScenarioParameters.h"
#include "scenpar-handle.h"

#define HANDLE_TO_GROUP(handle) (reinterpret_cast<C4Group*>(handle))

struct _C4ScenparHandle {
	C4ScenarioParameterDefs parameter_defs;
	C4ScenarioParameters parameters;
};

extern "C" {

C4ScenparHandle* c4_scenpar_handle_new(void)
{
	return new C4ScenparHandle();
}

void c4_scenpar_handle_free(C4ScenparHandle* handle)
{
	delete handle;
}

bool c4_scenpar_handle_load(C4ScenparHandle* handle, C4GroupHandle* group)
{
	return handle->parameter_defs.Load(*HANDLE_TO_GROUP(group), nullptr);
}

int32_t c4_scenpar_handle_get_value_by_id(C4ScenparHandle* handle, const char* id, int32_t default_value)
{
	return handle->parameters.GetValueByID(id, default_value);
}

void c4_scenpar_handle_set_value(C4ScenparHandle* handle, const char* id, int32_t value, bool only_if_larger)
{
	handle->parameters.SetValue(id, value, only_if_larger);
}


} /* extern "C" */
