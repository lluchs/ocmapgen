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
#include "landscape/C4MapScript.h"
#include "landscape/C4MapCreatorS2.h"
#include "landscape/C4Texture.h"
#include "script/C4ScriptHost.h"
#include "object/C4DefList.h"
#include "object/C4Def.h"
#include "script/C4Aul.h"
#include "script/C4AulDefFunc.h"
#include "lib/StdMeshLoader.h"
#include "lib/StdColors.h"
#include "c4group/C4Components.h"

#include "material-handle.h"
#include "texture-handle.h"
#include "log-handle.h"
#include "mapgen-handle.h"

#define HANDLE_TO_MATERIAL_MAP(handle) (reinterpret_cast<C4MaterialMap*>(handle))
#define HANDLE_TO_TEXTURE_MAP(handle) (reinterpret_cast<C4TextureMap*>(handle))
#define HANDLE_TO_GROUP(handle) (reinterpret_cast<C4Group*>(handle))

#include "player/C4ScenarioParameters.h"
struct _C4ScenparHandle {
	C4ScenarioParameterDefs parameter_defs;
	C4ScenarioParameters parameters;
};

namespace
{

bool HasAlgoScript(C4MCNode* node)
{
  if(node->Type() == MCN_Overlay && static_cast<C4MCOverlay*>(node)->Algorithm == static_cast<C4MCOverlay*>(node)->GetAlgo("script"))
    return true;

  if(node->Child0) return HasAlgoScript(node->Child0);
  if(node->Next) return HasAlgoScript(node->Next);
  return false;
}

class FakeSkeletonLoader: public StdMeshSkeletonLoader
{
public:
	virtual StdMeshSkeleton* GetSkeletonByDefinition(const char* definition) const { return nullptr; }
};

void ClearScriptEngine()
{
	MapScript.Clear();
	GameScript.Clear();
	ScriptEngine.Clear();
}

struct SystemScript {
	std::string filename, source;
};

static std::vector<SystemScript> system_scripts;

static int32_t startup_player_count = 1, startup_team_count = 1;
static int32_t FnGetStartupPlayerCount(C4PropList * _this) { return startup_player_count; }
static int32_t FnGetStartupTeamCount(C4PropList * _this) { return startup_team_count; }

bool SaveMap(CSurface8& map, const char* path, C4MaterialMap& material_map, C4TextureMap& texture_map)
{
	CStdPalette Palette;
	texture_map.StoreMapPalette(&Palette, material_map);
	return map.Save(path, &Palette);
}

}

extern "C" {

struct _C4MapgenHandle {
	unsigned int width;
	unsigned int height;
	unsigned int rowstride;
	StdCopyStrBuf error_message;
	std::unique_ptr<CSurface8> fg, bg;
};

void c4_mapgen_handle_init_script_engine()
{
	InitCoreFunctionMap(&ScriptEngine);
	::MapScript.InitFunctionMap(&ScriptEngine);

	// Load System.ocg scripts.
	for (const auto& script : system_scripts)
	{
		// host will be destroyed by script engine, so drop the references
		C4ScriptHost *scr = new C4ExtraScriptHost();
		scr->Reg2List(&ScriptEngine);
		scr->LoadData(script.filename.c_str(), script.source.c_str(), nullptr);
	}

	// Define special script functions.
	C4PropListStatic * p = ScriptEngine.GetPropList();
#define F(f) ::AddFunc(p, #f, Fn##f)
	F(GetStartupPlayerCount);
	F(GetStartupTeamCount);
#undef F
}

void c4_mapgen_handle_deinit_script_engine()
{
	ClearScriptEngine();
	system_scripts.clear();
	startup_player_count = 1;
	startup_team_count = 1;
}

void c4_mapgen_handle_set_map_library(C4GroupHandle* group_handle)
{
	::Definitions.Clear();

	C4Def* libmap = new C4Def;
	libmap->id = C4ID(std::string("Library_Map"));
	libmap->SetName(libmap->id.ToString());
	libmap->Category = C4D_StaticBack;
	FakeSkeletonLoader loader;
	if(!libmap->Load(*HANDLE_TO_GROUP(group_handle), loader, C4D_Load_Script, nullptr, nullptr))
	{
		fprintf(stderr, "Failed to load Library_Map script\n");
		delete libmap;
	}
	else
	{
		::Definitions.Add(libmap, false);
	}
}

void c4_mapgen_handle_load_system(C4GroupHandle* group_handle)
{
	auto File = HANDLE_TO_GROUP(group_handle);
	char fn[_MAX_FNAME+1] = { 0 };
	File->ResetSearch();
	while (File->FindNextEntry(C4CFN_ScriptFiles, fn, nullptr, !!fn[0]))
	{
		StdStrBuf buf;
		if (File->LoadEntryString(fn, &buf))
			system_scripts.push_back(SystemScript {std::string(fn), std::string(buf.getData())});
	}
	// Scripts will be loaded on next init.
}

void c4_mapgen_handle_load_script(const char* filename, const char* source)
{
	system_scripts.push_back(SystemScript {std::string(filename), std::string(source)});
}

void c4_mapgen_handle_set_startup_player_count(int32_t count)
{
	startup_player_count = count;
}

void c4_mapgen_handle_set_startup_team_count(int32_t count)
{
	startup_team_count = count;
}

C4MapgenHandle* c4_mapgen_handle_new_script(const char* filename, const char* source, C4ScenparHandle* scenpar, C4MaterialMapHandle* material_map, C4TextureMapHandle* texture_map, unsigned int map_width, unsigned int map_height)
{
	// Re-initialize script engine. Otherwise, we get a warning when the user
	// changes the value of a constant, since it is defined already from the
	// previous map rendering.  Note that we do not need to re-load the map library.
	ClearScriptEngine();
	c4_mapgen_handle_init_script_engine();

	if (!scenpar)
	{
		static C4ScenparHandle default_scenpar;
		scenpar = &default_scenpar;
	}
	scenpar->parameter_defs.RegisterScriptConstants(scenpar->parameters);

	try
	{
		// TODO: Could also re-use an existing CSurface8,
		// saving unnecessary malloc/free between map renderings
		C4SLandscape landscape;
		landscape.Default();

		landscape.MapWdt.Set(map_width, 0, map_width, map_width);
		landscape.MapHgt.Set(map_height, 0, map_height, map_height);
		landscape.MapPlayerExtend = 0;

		c4_log_handle_clear();
		::MapScript.LoadData(filename, source, nullptr);
		// If InitializeMap() returns false, the map creator wants to
		// call a fallback in the scenario script. This crashes if no
		// scenario script is loaded, so simply load an empty script
		// here:
		::GameScript.LoadData("Script.c", "", nullptr);

		const char* parse_error = c4_log_handle_get_first_log_message();
		if(parse_error)
			throw std::runtime_error(parse_error);

		// Link script engine (resolve includes/appends, generate code)
		c4_log_handle_clear();
		ScriptEngine.Link(&::Definitions);
		if(c4_log_handle_get_n_log_messages() > 1)
			throw std::runtime_error(c4_log_handle_get_first_log_message());

		// Generate map, fail if return error occurs
		c4_log_handle_clear();
		std::unique_ptr<CSurface8> out_ptr_fg, out_ptr_bg;
		const bool result = ::MapScript.InitializeMap(
			&landscape,
			HANDLE_TO_TEXTURE_MAP(texture_map),
			HANDLE_TO_MATERIAL_MAP(material_map),
			1,
			&out_ptr_fg, &out_ptr_bg);

		// Don't show any map if there was a script runtime error
		const char* runtime_error = c4_log_handle_get_first_log_message();
		if(runtime_error)
			throw std::runtime_error(runtime_error);

		if(!result)
			throw std::runtime_error("No InitializeMap() function present in the script, or it returns false");

		C4MapgenHandle* handle = new C4MapgenHandle;
		handle->width = out_ptr_fg->Wdt;
		handle->height = out_ptr_fg->Hgt;
		handle->rowstride = out_ptr_fg->Wdt;
		handle->error_message = nullptr;
		handle->fg = std::move(out_ptr_fg);
		handle->bg = std::move(out_ptr_bg);

		return handle;
	}
	catch(const std::exception& ex)
	{
		C4MapgenHandle* handle = new C4MapgenHandle;
		handle->width = 0;
		handle->height = 0;
		handle->error_message.Copy(ex.what());
		handle->fg = nullptr;
		handle->bg = nullptr;
		return handle;
	}
}

C4MapgenHandle* c4_mapgen_handle_new(const char* filename, const char* source, const char* script_path, C4MaterialMapHandle* material_map, C4TextureMapHandle* texture_map, unsigned int map_width, unsigned int map_height)
{
	try
	{
		C4SLandscape landscape;
		landscape.Default();

		landscape.MapWdt.Set(map_width, 0, map_width, map_width);
		landscape.MapHgt.Set(map_height, 0, map_height, map_height);
		landscape.MapPlayerExtend = 0;

		C4MapCreatorS2 mapgen(
			&landscape,
			HANDLE_TO_TEXTURE_MAP(texture_map),
			HANDLE_TO_MATERIAL_MAP(material_map),
			1
		);

		C4MCParser parser(&mapgen);
		parser.ParseMemFile(source, filename);

		C4MCMap* map = mapgen.GetMap(nullptr);
		if(!map) throw std::runtime_error("No map definition in source file");

		// Setup the script engine if there is an algo=script overlay in the
		// Landscape.txt file
		if(HasAlgoScript(mapgen.GetMap(nullptr)))
		{
			// Re-initialize script engine. Otherwise, we get a warning when the user
			// changes the value of a constant, since it is defined already from the
			// previous map rendering.  Note that we do not need to re-load the map library.
			ClearScriptEngine();
			c4_mapgen_handle_init_script_engine();

			if(script_path == nullptr)
				throw std::runtime_error("For algo=script overlays to work, save the file first at the location of the Script.c file");

			char dirname[_MAX_PATH]; GetParentPath(script_path, dirname);
			const char* basename = GetFilename(script_path);

			C4Group File;
			if(!File.Open(dirname))
			{
				StdStrBuf error_msg = FormatString("Failed to open directory '%s': %s", dirname, File.GetError());
				throw std::runtime_error(error_msg.getData());
			}

			// get scripts
			File.ResetSearch();
			if(!File.FindNextEntry(basename, (char*)nullptr))
			{
				StdStrBuf error_msg = FormatString("Failed to load '%s': No such file", script_path);
				throw std::runtime_error(error_msg.getData());
			}

			c4_log_handle_clear();
			GameScript.Load(File, basename, nullptr, nullptr);

			const char* parse_error = c4_log_handle_get_first_log_message();
			if(parse_error)
				throw std::runtime_error(parse_error);

			// Link script engine (resolve includes/appends, generate code)
			c4_log_handle_clear();
			ScriptEngine.Link(&::Definitions);
			if(c4_log_handle_get_n_log_messages() > 1)
				throw std::runtime_error(c4_log_handle_get_first_log_message());
		}

		c4_log_handle_clear();
		CSurface8 *_out_ptr_fg = nullptr, *_out_ptr_bg = nullptr;
		mapgen.Render(nullptr, _out_ptr_fg, _out_ptr_bg);
		std::unique_ptr<CSurface8> out_ptr_fg(_out_ptr_fg), out_ptr_bg(_out_ptr_bg);

		// Don't show any map if there was a script runtime error
		const char* runtime_error = c4_log_handle_get_first_log_message();
		if(runtime_error)
		{
			throw std::runtime_error(runtime_error);
		}

		C4MapgenHandle* handle = new C4MapgenHandle;
		handle->width = out_ptr_fg->Wdt;
		handle->height = out_ptr_fg->Hgt;
		handle->rowstride = out_ptr_fg->Wdt;
		handle->error_message = nullptr;
		handle->fg = std::move(out_ptr_fg);
		handle->bg = std::move(out_ptr_bg);
		return handle;
	}
	catch(const C4MCParserErr& err)
	{
		C4MapgenHandle* handle = new C4MapgenHandle;
		handle->width = 0;
		handle->height = 0;
		handle->error_message.Copy(err.Msg);
		handle->fg = nullptr;
		handle->bg = nullptr;
		return handle;
	}
	catch(const std::exception& ex)
	{
		C4MapgenHandle* handle = new C4MapgenHandle;
		handle->width = 0;
		handle->height = 0;
		handle->error_message.Copy(ex.what());
		handle->fg = nullptr;
		handle->bg = nullptr;
		return handle;
	}
}

void c4_mapgen_handle_free(C4MapgenHandle* mapgen)
{
	delete mapgen;
}

const unsigned char* c4_mapgen_handle_get_map(C4MapgenHandle* mapgen)
{
	return reinterpret_cast<unsigned char*>(mapgen->fg->Bits);
}

const unsigned char* c4_mapgen_handle_get_bg(C4MapgenHandle* mapgen)
{
	return reinterpret_cast<unsigned char*>(mapgen->bg->Bits);
}

bool c4_mapgen_handle_save_map(C4MapgenHandle* mapgen, const char* path, C4MaterialMapHandle* material_map, C4TextureMapHandle* texture_map)
{
	auto matmap = HANDLE_TO_MATERIAL_MAP(material_map);
	auto texmap = HANDLE_TO_TEXTURE_MAP(texture_map);
	return SaveMap(*mapgen->fg, path, *matmap, *texmap);
}

bool c4_mapgen_handle_save_bg(C4MapgenHandle* mapgen, const char* path, C4MaterialMapHandle* material_map, C4TextureMapHandle* texture_map)
{
	auto matmap = HANDLE_TO_MATERIAL_MAP(material_map);
	auto texmap = HANDLE_TO_TEXTURE_MAP(texture_map);
	return SaveMap(*mapgen->bg, path, *matmap, *texmap);
}

unsigned int c4_mapgen_handle_get_width(C4MapgenHandle* mapgen)
{
	assert(mapgen->fg);
	return mapgen->width;
}

unsigned int c4_mapgen_handle_get_height(C4MapgenHandle* mapgen)
{
	assert(mapgen->fg);
	return mapgen->height;
}

unsigned int c4_mapgen_handle_get_rowstride(C4MapgenHandle* mapgen)
{
	assert(mapgen->fg);
	return mapgen->rowstride;
}

const char* c4_mapgen_handle_get_error(C4MapgenHandle* mapgen)
{
	if(mapgen->fg)
		return nullptr;
	return mapgen->error_message.getData();
}

} // extern "C"
