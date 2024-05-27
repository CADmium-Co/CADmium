// currently not used since projectUtils & stores were pulled out and placed in packages/shared
// todo

export function isDevelopment() {
	return (globalThis as any).process.env.NODE_ENV === "development"
}

export function isProduction() {
	return (globalThis as any).process.env.NODE_ENV !== "development"
}

export function setDevelopment(shouldSet: boolean): void {
	;(globalThis as any).process = (globalThis as any).process ?? {}
	const env = (globalThis as any).process.env ?? {}
	if (shouldSet) (globalThis as any).process.env = { ...env, NODE_ENV: "development" }
	else (globalThis as any).process.env = { ...env, NODE_ENV: "production" }
}

// disable logging
if (isProduction()) {
	const methods = ["log", "debug", "warn", "info"]
	for (let i = 0; i < methods.length; i++) {
		// @ts-ignore
		console[methods[i]] = function () {}
	}
}
