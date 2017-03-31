# Use rust-bindgen to generate bindings.
#
# As rust-bindgen has system dependencies on clang, the generated file is
# checked into git and not created in build.rs.

all: src/ffi.rs src/StandaloneCompat.c

src/ffi.rs: src/cpp-handles/*.h
	bindgen --whitelist-function 'c4_.*' --raw-line '#![allow(dead_code)]' -o$@ src/cpp-handles/bindgen.h

exclude_fns := GetStartupPlayerCount|GetStartupTeamCount
script_sources := openclonk/src/game/C4GameScript.cpp openclonk/src/object/C4ObjectScript.cpp
player_controls := openclonk/planet/System.ocg/PlayerControls.txt
src/StandaloneCompat.c: $(script_sources) $(player_controls)
	echo "/* Automatically generated from: $(script_sources) */" > $@
	sed -En '/$(exclude_fns)/!s/^[a-zA-Z0-9_ <>*]*Fn(\w+)\(.*$$/global func \1(...) { FatalError("standalone stub"); }/p' $(script_sources) >> $@
	sed -En 's/^\s*\{\s*"(\w+)"\s*,\s*C4V_Int.*/static const \1 = 0;/p' $(script_sources) >> $@
	echo >> $@
	echo "/* Automatically generated from: $(player_controls) */" >> $@
	sed -En 's/\s*Identifier=(\w+).*/static const CON_\1 = 0;/p' $(player_controls) >> $@

.PHONY: all
