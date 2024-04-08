import { describe, it, expect } from "vitest"
import { isEntity, isEntityType, isProject } from "shared/typeGuards"
import { currentlySelected, project } from "./fixtures"

describe("isEntityType typechecks", () => {
	const types = ["circle", "arc", "face", "line", "plane", "point", "point3D", "meshFace"]
	const circleObject = {}
	const circleFunction = () => "circle"

	it("typechecks all EntityType", () => {
		expect(types.every((entityType) => isEntityType(entityType))).toBe(true)
	})
	it("'circles' type fails", () => {
		expect([...types, "circles"].every((entityType) => isEntityType(entityType))).toBe(false)
	})
	it("'{}' fails", () => {
		expect(["circle", circleObject].every((entityType) => isEntityType(entityType))).toBe(false)
	})
	it("function returning string of correct type succeeds", () => {
		expect(["circle", circleFunction()].every((entityType) => isEntityType(entityType))).toBe(true)
	})
})

describe("currentlySelected is Entity[]", () => {
	it("every currentlySelected item isEntity", () => {
		expect(currentlySelected.every((entity) => isEntity(entity))).toBe(true)
	})

	it("fails on type:'circles'", () => {
		expect([...currentlySelected, { type: "circles", id: "1" }].every((entity) => isEntity(entity))).toBe(false)
	})
	it("fails with id:number", () => {
		expect(
			[
				{ type: "point", id: 3 },
				{ type: "circle", id: "1" }
			].every((entity) => isEntity(entity))
		).toBe(false)
	})
	it("fails with id:object", () => {
		expect(
			[
				{ type: "point", id: { type: "circle", id: "1" } },
				{ type: "circle", id: "1" }
			].every((entity) => isEntity(entity))
		).toBe(false)
	})
	it("fails when missing type", () => {
		expect(
			[
				{ type: "point", id: "3" },
				{ id: "1", typez: "point" }
			].every((entity) => isEntity(entity))
		).toBe(false)
	})
	it("fails when missing id", () => {
		expect(
			[
				{ type: "point", notId: "99" },
				{ type: "circle", id: "1" }
			].every((entity) => isEntity(entity))
		).toBe(false)
	})
	it("fails on extra properties", () => {
		expect(
			[
				{ type: "point", id: "a string" },
				{ type: "circle", id: "1", errantProperty: "string" }
			].every((entity) => isEntity(entity))
		).toBe(false)
	})
})

describe("currentlySelected has malformed entities", () => {
	it("currentlySelected has no duplicates", () => {
		const set = new Set()
		currentlySelected.forEach((entity) => set.add(entity.id))
		expect(currentlySelected.length === set.size).toBe(true)
	})
	it("fails when currentlySelected has duplicates", () => {
		const set = new Set()
		const duplicates = [...currentlySelected, { type: "circle", id: "1" }]
		duplicates.forEach((entity) => set.add(entity.id))
		expect(duplicates.length === set.size).toBe(false)
	})
})

describe("currentlySelected has malformed entities", () => {
	it("currentlySelected has no duplicates", () => {
		const set = new Set()
		currentlySelected.forEach((entity) => set.add(entity.id))
		expect(currentlySelected.length === set.size).toBe(true)
	})
	it("fails when currentlySelected has duplicates", () => {
		const set = new Set()
		const duplicates = [...currentlySelected, { type: "circle", id: "1" }]
		duplicates.forEach((entity) => set.add(entity.id))
		expect(duplicates.length === set.size).toBe(false)
	})
})

describe("isProject typechecks", () => {
	it("project is correct shape", () => {
		expect(isProject(project)).toBe(true)
	})
	it("project is valid when workbenches: []", () => {
		expect(isProject({ ...project, workbenches: [] })).toBe(true)
	})

	it("fails when assemblies: null", () => {
		expect(isProject({ ...project, assemblies: null })).toBe(false)
	})
	it("fails when name: null", () => {
		expect(isProject({ ...project, name: null })).toBe(false)
	})
	it("fails when name: number", () => {
		expect(isProject({ ...project, name: 123 })).toBe(false)
	})
	it("fails when name: object", () => {
		expect(isProject({ ...project, name: {} })).toBe(false)
	})
	it("fails when name: array", () => {
		expect(isProject({ ...project, name: [] })).toBe(false)
	})
	it("fails when workbenches: null", () => {
		expect(isProject({ ...project, workbenches: null })).toBe(false)
	})
	it("fails when workbenches: {}", () => {
		expect(isProject({ ...project, workbenches: {} })).toBe(false)
	})
})
