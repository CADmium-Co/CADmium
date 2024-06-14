import {expect, test} from "@playwright/test"

test("index page has expected title", async ({page}) => {
  await page.goto("")
  // await expect(page.getByRole('heading', { name: 'Welcome to SvelteKit' })).toBeVisible()
  await expect(page).toHaveTitle("CADmium")
})

test("has history pane", async ({page}) => {
  await page.goto("")
  await expect(page.getByText("History")).toBeVisible()
})

test("has origin in history panel", async ({page}) => {
  await page.goto("")
  await expect(page.getByText("origin")).toBeVisible()
})
