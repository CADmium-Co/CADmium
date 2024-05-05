#!/usr/bin/env node

const packagejson = require("./pkg/package.json");
const { writeFileSync } = require("fs");
const { join } = require("path");

writeFileSync(
  join(__dirname, "pkg", "package.json"),
  JSON.stringify({ ...packagejson, type: "module" }, null, 2)
);
