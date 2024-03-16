export const prerender = true

const DEBUG = true
if (!DEBUG) {
	const methods = ["log", "debug", "warn", "info"]
	for (let i = 0; i < methods.length; i++) {
		// @ts-ignore
		console[methods[i]] = function () { }
	}
}
