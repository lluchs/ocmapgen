const test = require('ava')
const {MapGen} = require('../ocmapgen')

let empty = `
protected func InitializeMap(proplist map)
{
	return true;
}
`

test('Multiple sequential', async t => {
	let mapgen = t.context.mapgen = new MapGen({
		root: __dirname + '/../../openclonk/planet',
		map_type: 'Map.c',
	})

	let {fg: fg1} = await mapgen.generate(empty)
	t.true(isPNG(fg1))
	let {fg: fg2} = await mapgen.generate(empty)
	t.true(isPNG(fg2))
	t.deepEqual(fg1, fg2)
})

test('Multiple parallel calls (but sequential execution)', async t => {
	let mapgen = t.context.mapgen = new MapGen({
		root: __dirname + '/../../openclonk/planet',
		map_type: 'Map.c',
	})

	let m1 = mapgen.generate(empty)
	let m2 = mapgen.generate(empty)
	let {fg: fg1} = await m1
	t.true(isPNG(fg1))
	let {fg: fg2} = await m2
	t.true(isPNG(fg2))
	t.deepEqual(fg1, fg2)
})


test.afterEach(t => {
	t.context.mapgen.end()
})

function isPNG(buf) {
	return buf.toString('ascii', 1, 4) == 'PNG'
}
