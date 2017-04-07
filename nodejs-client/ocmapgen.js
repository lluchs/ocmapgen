const cbor = require('cbor')
const {spawn} = require('child_process')

const defaultOptions = {
	cmd: 'ocmapgen',
}

class MapGen {
	// Initializes the map generator, but doesn't spawn a subprocess yet.
	//
	// Options:
	//  - cmd: ocmapgen executable. Defaults to "ocmapgen".
	//  - timeout: Time limit for rendering a map.
	//  - width height root players teams seed bg map_type: correspond to ocmapgen options.
	constructor(options) {
		this.options = Object.assign({}, defaultOptions, options)
	}

	// Generates a map with the given source code.
	//
	// Returns an object {fg, bg} with PNG data. `bg` is only set if the `bg` option is true.
	async generate(source) {
		// Only allow a single request at a time.
		if (this.request)
			await alwaysResolve(this.request)
		// Ensure there is a running process.
		if (!this.process)
			this._spawn()
		let req = cbor.encode(['RenderMap', {source}])
		this.process.stdin.write(req)

		// Generation runs until:
		//  - it finishes
		//  - the timeout runs out
		//  - the process exits for some unrelated reason
		let resPromise = new Promise((resolve, reject) => {
			let decoder = new cbor.Decoder()
			decoder.once('data', (d) => resolve(d))
			decoder.once('error', (e) => reject(e))
			this.process.stdout.pipe(decoder)
		})
		alwaysResolve(resPromise).then(() =>
			this.process.stdout.unpipe())
		let timeoutPromise = new Promise((resolve, reject) => {
			if (this.options.timeout) {
				let t = setTimeout(() => {
					if (this.process)
						this.process.kill()
					reject(new Error("timeout passed"))
				}, this.options.timeout)
				Promise.race([resPromise, alwaysResolve(this.exitPromise)]).then(() => {
					clearTimeout(t)
				})
			}
		})

		// Only the resPromise resolves to something.
		this.request = Promise.race([resPromise, timeoutPromise, this.exitPromise])
		let [type, arg] = await this.request
		switch (type) {
		case 'Image':
			this.request = null
			return arg
		case 'Error':
			throw new Error(arg)
		default:
			throw new Error(`unexpected message: ${type}`)
		}
	}

	// Call when done with processing.
	end() {
		if (this.process)
			this.process.kill()
	}

	_spawn() {
		let args = ['--cbor']
		for (let opt of 'width height root players teams seed map_type'.split(' ')) {
			if (opt in this.options)
				args.push(`--${opt.replace('_', '-')}=${this.options[opt]}`)
		}
		if (this.options.bg)
			args.push('--bg=dummy')
		this.process = spawn(this.options.cmd, args, {
			stdio: ['pipe', 'pipe', process.stderr],
		})

		this.exitPromise = new Promise((resolve, reject) => {
			this.process.on('close', (code) => {
				reject(new Error(`ocmapgen exited with status ${code}`))
				this.process = null
				this.exitPromise = null
			})
		})
	}
}

// Returns a new promise which always resolves.
function alwaysResolve(promise) {
	return new Promise((resolve, reject) => {
		promise.then(resolve, resolve)
	})
}

module.exports = {MapGen}
