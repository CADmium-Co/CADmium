export const prerender = true

_setDevelopment(true)

export function _isDevelopment() {
	return (globalThis as any).process.env.NODE_ENV === "development"
}

export function _isProduction() {
	return (globalThis as any).process.env.NODE_ENV !== "development"
}

export function _setDevelopment(shouldSet: boolean): void {
	(globalThis as any).process = (globalThis as any).process ?? {}
	const env = (globalThis as any).process.env ?? {}
	if (shouldSet) (globalThis as any).process.env = { ...env, NODE_ENV: "development" }
	else (globalThis as any).process.env = { ...env, NODE_ENV: "production" }
}

// disable logging
if (_isProduction()) {
	const methods = ["log", "debug", "warn", "info"]
	for (let i = 0; i < methods.length; i++) {
		// @ts-ignore
		console[methods[i]] = function () { }
	}
}
