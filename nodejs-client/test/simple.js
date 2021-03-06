const test = require('ava')
const {MapGen} = require('../ocmapgen')

let acidGoldMine = `
/**
	Acid Gold Mine
	An acid lake with a cliff leading to a volcanic gold mine.
	
	@author Sven2, Maikel
*/

#include Library_Map

static SCENPAR_MapSize, SCENPAR_Difficulty;

// Called be the engine: draw the complete map here.
protected func InitializeMap(proplist map)
{
	SCENPAR_MapSize = 1;
	SCENPAR_Difficulty = 1;
	// Retrieve the settings according to the MapSize setting.
	// TODO: Change map size and rescale algorithms accordingly.
	var map_size;
	if (SCENPAR_MapSize == 1)
		map_size = [90, 240]; 
	if (SCENPAR_MapSize == 2)
		map_size = [90, 240];
	if (SCENPAR_MapSize == 3)
		map_size = [90, 240];
	
	// Set the map size and define different areas on the vertical axes.
	// The horizontal size is fixed for all map sizes.
	map->Resize(map_size[0], map_size[1]);
	var wdt = map.Wdt;
	var hgt = map.Hgt;
	var acid_level = 32;     // Top of the acid lake.
	var acid_hills = 75;     // Top of the hills in the acid lake.
	var acid_bottom = 106;   // Bottom of the acid lake.
	var ground_middle = 160; // Middle of the solid ground.
	var ground_bottom = 180; // Bottom of the solid ground.
	
	// Acid lake: draw first and let rest overwrite this layer.
	var acid_lake = {Algo = MAPALGO_Rect, X = 0, Y = acid_level, Wdt = wdt, Hgt = hgt};
	map->Draw("Acid:Sky", acid_lake);
	
	// Draw the basic shape of the earth and add different earth types.
	var ground = GetGroundShape(map, acid_level, acid_hills, acid_bottom);
	map->Draw("Earth", ground);
	map->DrawMaterial("Earth-earth", ground, 4, 30);
	map->DrawMaterial("Earth-earth", ground, 4, 30);
	map->DrawMaterial("Earth-earth_root", ground, 4, 30);
	map->DrawMaterial("Earth-earth_spongy", ground, 4, 30);
	var ground_border = {Algo = MAPALGO_Border, Left = [1, -1], Right = [1, -1], Top = -1, Op = ground};
	map->DrawMaterial("Rock-rock", ground_border, 4, 40);
	map->DrawMaterial("Rock-rock", ground_border, 4, 40);
	
	// Fill the basic shape with materials.
	var ground_toplayer = {Algo = MAPALGO_Rect, X = 0, Y = acid_level, Wdt = wdt, Hgt = acid_hills - acid_level};
	ground_toplayer = {Algo = MAPALGO_Turbulence, Amplitude = 12, Scale = 8, Iterations = 4, Seed = Random(65536), Op = ground_toplayer};
	ground_toplayer = {Algo = MAPALGO_And, Op = [ground, ground_toplayer]};
	var ground_middle1layer = {Algo = MAPALGO_Rect, X = 0, Y = acid_hills, Wdt = wdt, Hgt = acid_bottom - acid_hills};
	ground_middle1layer = {Algo = MAPALGO_Turbulence, Amplitude = 12, Scale = 8, Iterations = 4, Seed = Random(65536), Op = ground_middle1layer};
	ground_middle1layer = {Algo = MAPALGO_And, Op = [ground, ground_middle1layer]};
	var ground_middle2layer = {Algo = MAPALGO_Rect, X = 0, Y = acid_bottom, Wdt = wdt, Hgt = ground_middle - acid_bottom};
	ground_middle2layer = {Algo = MAPALGO_Turbulence, Amplitude = 12, Scale = 8, Iterations = 4, Seed = Random(65536), Op = ground_middle2layer};
	ground_middle2layer = {Algo = MAPALGO_And, Op = [ground, ground_middle2layer]};
	var ground_bottomlayer = {Algo = MAPALGO_Rect, X = 0, Y = ground_middle, Wdt = wdt, Hgt = ground_bottom - ground_middle};
	ground_bottomlayer = {Algo = MAPALGO_Turbulence, Amplitude = 12, Scale = 8, Iterations = 4, Seed = Random(65536), Op = ground_bottomlayer};
	ground_bottomlayer = {Algo = MAPALGO_And, Op = [ground, ground_bottomlayer]};
	// Fill the top layer with rock.
	map->DrawMaterial("Rock", ground_toplayer, [6, 2], 6);
	map->DrawMaterial("Rock", ground_toplayer, [6, 2], 6);
	// Fill the first middle layer with materials and rock.
	map->DrawMaterial("Rock", ground_toplayer, [6, 2], 6);
	map->DrawMaterial("Rock", ground_toplayer, [12, 3], 6);
	map->DrawMaterial("Rock", ground_middle1layer, [12, 3], 6);
	map->DrawMaterial("Ore", ground_middle1layer, [12, 3], 8);
	map->DrawMaterial("Coal", ground_middle1layer, [12, 3], 8);
	map->DrawMaterial("Firestone", ground_middle1layer, [12, 3], 8);
	// Fill the second middle layer with materials and acid.
	map->DrawMaterial("Granite", ground_middle2layer, [12, 2], 10);
	map->DrawMaterial("Rock", ground_middle2layer, [12, 2], 8);
	map->DrawMaterial("Tunnel", ground_middle2layer, [10, 4], 10);
	map->DrawMaterial("Ore", ground_middle2layer, [12, 3], 5);
	map->DrawMaterial("Coal", ground_middle2layer, [12, 3], 5);
	map->DrawMaterial("Firestone", ground_middle2layer, [12, 3], 5);
	map->DrawMaterial("Acid", ground_middle2layer, [6, 3], 10);
	// Fill the bottom layer with acid and granite.
	map->DrawMaterial("Granite", ground_bottomlayer, [12, 2], 10);
	map->DrawMaterial("Rock", ground_bottomlayer, [12, 2], 6);
	map->DrawMaterial(["Water", "Acid", "DuroLava"][SCENPAR_Difficulty - 1], ground_bottomlayer, [8, 3], 10);
	map->DrawMaterial("Acid", ground_bottomlayer, [6, 3], 10);
	
	// Reinforce the lower parts of the acid lake, height depends on difficulty.
	var lake_pit = {Algo = MAPALGO_Rect, X = 48, Y = acid_bottom - 12, Wdt = 20, Hgt = 22 - SCENPAR_Difficulty};
	lake_pit = {Algo = MAPALGO_Turbulence, Amplitude = 8, Scale = 6, Iterations = 2, Seed = Random(65536), Op = lake_pit};
	lake_pit = {Algo = MAPALGO_And, Op = [ground, lake_pit]};
	map->Draw("Granite", lake_pit);
	map->DrawMaterial("Rock", lake_pit, 2, 15);
	map->DrawMaterial("Rock", lake_pit, 2, 15);

	// Draw the map sides and the upper part of the volcano out of granite.
	var granite = GetGraniteShape(map, ground_bottom);
	map->Draw("Granite", granite);
	map->DrawMaterial("Rock", granite, 5, 25);
	map->DrawMaterial("Rock", granite, 5, 25);
	map->DrawMaterial("DuroLava", granite, 5, 25);
	map->DrawMaterial("DuroLava", granite, 2, 15);
	map->Draw("Granite", {Algo = MAPALGO_Border, Wdt = 2, Op = granite});
	
	// Draw the volcano using lava, granite and gold.
	var lava = GetLavaShape(map, granite, ground_bottom);
	var lava_gold = {Algo = MAPALGO_And, Op = [lava, {Algo = MAPALGO_Rect, X = 0, Y = ground_bottom + 8 * SCENPAR_Difficulty, Wdt = wdt, Hgt = hgt}]};
	map->Draw("DuroLava", lava);
	map->DrawMaterial("Granite", lava, 5, 25);
	map->DrawMaterial("Gold", lava_gold, 5, 25);
	var lava_bottom = {Algo = MAPALGO_Rect, X = 0, Y = hgt - 8, Wdt = wdt, Hgt = hgt};
	var lava_bottom_rnd = {Algo = MAPALGO_Turbulence, Amplitude = 12, Scale = 8, Iterations = 4, Seed = Random(65536), Op = lava_bottom};
	lava_bottom = {Algo = MAPALGO_And, Op = [lava_bottom, lava_bottom_rnd]};
	map->Draw("DuroLava", lava_bottom);
	
	// Create an underground spot where wind is available.
	var wind_spot = {Algo = MAPALGO_Rect, X = 38, Y = acid_bottom - 4, Wdt = 8 - SCENPAR_Difficulty, Hgt = 8 - SCENPAR_Difficulty};
	wind_spot = {Algo = MAPALGO_Turbulence, Amplitude = 4, Scale = 4, Iterations = 1, Seed = Random(65536), Op = wind_spot};
	map->Draw("Sky", wind_spot);
	var wind_spot_border = {Algo = MAPALGO_Border, Wdt = 1, Op = wind_spot};
	wind_spot_border = {Algo = MAPALGO_Turbulence, Amplitude = 4, Scale = 4, Iterations = 1, Seed = Random(65536), Op = wind_spot_border};
	map->Draw("Tunnel", wind_spot_border);
	
	// Place some additional gold at the bottom of the acid lake.
	var acid_gold = {Algo = MAPALGO_Rect, X = 82, Y = acid_hills + 4, Wdt = 5 - SCENPAR_Difficulty, Hgt = 5 - SCENPAR_Difficulty};
 	acid_gold = {Algo = MAPALGO_Turbulence, Amplitude = 4, Scale = 4, Iterations = 1, Seed = Random(65536), Op = acid_gold};
 	map->Draw("Gold", acid_gold);
 	var acid_gold_border = {Algo = MAPALGO_Border, Wdt = -1, Op = acid_gold};
	map->Draw("Granite", acid_gold_border);
	
	// Fix the liquid borders of the water and acid in the ground.
	FixLiquidBorders();

	// Return true to tell the engine a map has been successfully created.
	return true;
}

public func GetGroundShape(proplist map, int acid_level, int acid_hills, int acid_bottom)
{
	var wdt = map.Wdt;
	var hgt = map.Hgt;
	var ground_x = [0, 18, 23, 45, 54, 68, 77, 90, 90, 0];
	var ground_y = [acid_hills, acid_hills, acid_level - 10, acid_level - 10, acid_bottom, acid_bottom, acid_hills, acid_hills, hgt, hgt];
	var ground = {Algo = MAPALGO_Polygon, X = ground_x, Y = ground_y};
	var ground_rnd = {Algo = MAPALGO_Turbulence, Amplitude = 10, Scale = 10, Iterations = 4, Seed = Random(65536), Op = ground};
	ground = {Algo = MAPALGO_Or, Op = [ground, ground_rnd]};
	return ground;
}

public func GetGraniteShape(proplist map, int ground_bottom)
{
	var wdt = map.Wdt;
	var hgt = map.Hgt;
	var granite_layer = {Algo = MAPALGO_Rect, X = 0, Y = ground_bottom, Wdt = wdt, Hgt = 5};
	var granite_x = [  0,   4,   8,  24,  24, wdt-24, wdt-24, wdt-8, wdt-4, wdt,    wdt,      0];
	var granite_y = [128, 128, 170, 229, hgt,    hgt,    229,   170,   128, 128, hgt+20, hgt+20];
	var granite_side = {Algo = MAPALGO_Polygon, X = granite_x, Y = granite_y};
	var granite = {Algo = MAPALGO_Or, Op = [granite_side, granite_layer]};
	var granite_rnd = {Algo = MAPALGO_Turbulence, Amplitude = 12, Scale = 8, Iterations = 4, Seed = Random(65536), Op = granite};
	granite = {Algo = MAPALGO_Or, Op = [granite, granite_rnd]};	
	return granite;
}

public func GetLavaShape(proplist map, proplist granite, int ground_bottom)
{
	var wdt = map.Wdt;
	var hgt = map.Hgt;	
	var lava = {Algo = MAPALGO_Rect, X = 0, Y = ground_bottom, Wdt = wdt, Hgt = hgt};
	lava = {Algo = MAPALGO_And, Op = [lava, {Algo = MAPALGO_Not, Op = granite}, {Algo = MAPALGO_Not, Op = granite}]};
	return lava;
}
`

let shiverPeak = `
/*-- ShiverPeak --*/

// A peak formed mountain, with lots of materials.
map ShiverPeak
{
	overlay {
		algo=poly; mask=1;
		point { x=18%; y=100%; };
		point { x=82%; y=100%; };
		point { x=74%; y=41%; };
		point { x=50%; y=0%; };
		point { x=26%; y=41%; };
		overlay {
			algo=border; mat=Ice; invert=1; a=7; b=7; turbulence=100;
			overlay {algo=rndchecker; turbulence=100; mat=Earth; tex=earth_root;};
			overlay { algo=rndchecker; a=5; zoomX=-30; zoomY=-30; turbulence=100; mat=Ore; };
			overlay { algo=rndchecker; a=2; zoomX=30; zoomY=30; turbulence=100; mat=Granite; };
			overlay { algo=rndchecker; a=19; zoomX=-30; zoomY=-30; turbulence=100; mat=Gold; };
			overlay {
				algo=rndchecker; turbulence=190; a=2; invert=1; mask=1;
				overlay {algo=rndchecker; mat=Rock; a=2; turbulence=100;};
				overlay {
					algo=border; a=10; b=10; invert=1; turbulence=100; mask=1;
					overlay { algo=rndchecker; a=3; zoomX=-50; zoomY=-50; turbulence=100; mat=Granite; };
					overlay { algo=rndchecker; a=6; zoomX=-20; zoomY=-20; turbulence=100; mat=Gold; }
					| overlay { algo=rndchecker; a=8; zoomX=10; zoomY=10; turbulence=100; mat=Gold; };
           			 overlay { algo=rndchecker; a=4; zoomX=-40; zoomY=-40; turbulence=100; mat=Firestone; }
					| overlay { algo=rndchecker; a=4; zoomX=-40; zoomY=-40; turbulence=100; mat=Firestone; };
					overlay { algo=rndchecker; a=3; zoomX=30; zoomY=30; turbulence=100; mat=Earth; tex=earth_spongy;};
					overlay {
						algo=rndchecker; a=2; zoomX=50; zoomY=50; turbulence=100; mat=Tunnel;
						overlay {algo=border; b=1; mat=Rock; turbulence=2;};
					}
   					| overlay {
						algo=rndchecker; a=1; zoomX=50; zoomY=50; turbulence=100; mat=Tunnel;
						overlay {algo=border; b=1; mat=Rock; turbulence=2;};
					};
				};
				overlay {algo=border; invert=1; turbulence=80; a=4; b=4;}
				& overlay {
					algo=lines; a=22; b=24; rotate = 90; mask=1;
					overlay { 
						algo=border; invert=1; a=2; b=2; turbulence=20; mask=1;
						overlay {
							algo=solid; mask=1;
							overlay {
								algo=solid; wdt=45; hgt=90; mask=1;
								overlay {algo=border; mat=Tunnel; invert=1; a=2; b=1; turbulence=60; };
							};
							overlay {
								algo=solid; x=55; hgt=90; mask=1;
								overlay {algo=border; mat=Tunnel; invert=1; a=2; b=1; turbulence=60;};
							};
						}; 
					};
				};
			};
			overlay {
				x=37; wdt=26; y=86; hgt=12; mask=1; 
				overlay {algo=border; invert=1; a=2; b=2; turbulence=100; mat=Tunnel;};
			};
		};
	};
};
`

function createMapGen(opts = {}) {
	return new MapGen(Object.assign({
		root: __dirname + '/../../openclonk/planet',
		map_type: 'Map.c',
		timeout: 5000,
	}, opts))
}

test('Acid Gold Mine (Map.c)', async t => {
	let mapgen = t.context.mapgen = createMapGen();
	let {fg, bg} = await mapgen.generate(acidGoldMine)
	t.is(fg.toString('ascii', 1, 4), 'PNG')
	t.is(bg, null)
})

test('Acid Gold Mine with bg', async t => {
	let mapgen = t.context.mapgen = createMapGen({
		bg: true,
	})

	let {fg, bg} = await mapgen.generate(acidGoldMine)
	t.is(fg.toString('ascii', 1, 4), 'PNG')
	t.is(bg.toString('ascii', 1, 4), 'PNG')
})

test('Shiver Peak (Landscape.txt)', async t => {
	let mapgen = t.context.mapgen = createMapGen({
		map_type: 'Landscape.txt',
	})

	let {fg, bg} = await mapgen.generate(shiverPeak)
	t.is(fg.toString('ascii', 1, 4), 'PNG')
	t.is(bg, null)
})


test('Timeout', async t => {
	let mapgen = t.context.mapgen = createMapGen({
		timeout: 100,
	})

	let infiniteLoop = `
protected func InitializeMap(proplist map)
{
	while (true) { }
}
	`

	const error = await t.throws(mapgen.generate(infiniteLoop))
	t.is(error.message, 'timeout passed')
})

test('Warnings', async t => {
	let mapgen = t.context.mapgen = createMapGen()
	let script = `
protected func InitializeMap(proplist map)
{
	var foo = "\\p";
	return true;
}
	`

	let {warnings} = await mapgen.generate(script)
	t.regex(warnings, /^WARNING: unknown escape sequence/)
})

test('Script output', async t => {
	let mapgen = t.context.mapgen = createMapGen()
	let script = `
protected func InitializeMap(proplist map)
{
	Log("foobar");
	return true;
}
	`

	let {script_output} = await mapgen.generate(script)
	t.is(script_output, 'foobar');
})

test.afterEach(t => {
	t.context.mapgen.end()
})
