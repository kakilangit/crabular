# crabular-wasm

WebAssembly bindings for [crabular](https://github.com/kakilangit/crabular) - ASCII table library for JavaScript/TypeScript.

> **Note:** This crate is **npm-only** and should not be published to crates.io. It's built with `wasm-pack` and published to the npm registry. The crate exists in the workspace solely as a build artifact source.

## Install

```bash
npm install crabular
```

## Quick Start

```javascript
import init, { JsTable } from 'crabular';

await init();

const table = new JsTable();
table.style('modern');
table.header(['Name', 'Age']);
table.row(['Alice', '30']);
table.row(['Bob', '25']);
console.log(table.render());
// Output:
// ┌───────┬─────┐
// │ Name  │ Age │
// ├───────┼─────┤
// │ Alice │ 30  │
// │ Bob   │ 25  │
// └───────┴─────┘
```

## Styles

- `classic` - ASCII borders (+ - |)
- `modern` - Unicode box drawing (default)
- `minimal` - Horizontal lines only
- `compact` - No outer frame
- `markdown` - GitHub-flavored Markdown

## API

### JsTable (Builder)

```javascript
const table = new JsTable();
table.style('modern');           // Set style
table.header(['A', 'B']);        // Add headers
table.row(['1', '2']);           // Add row
table.rows([['3', '4']]);        // Add multiple rows
table.align(1, 'right');         // Align column (left/center/right)
table.valign('middle');          // Vertical align (top/middle/bottom)
table.padding(1, 1);             // Cell padding
table.spacing(1);                // Column spacing

const output = table.render();   // Render to string
const built = table.build();     // Get JsTableObject
```

### JsTableObject (Built)

```javascript
const built = table.build();
built.addRow(['C', '3']);        // Add row
built.sort(0);                   // Sort ascending
built.sortDesc(0);               // Sort descending
built.sortNum(1);                // Sort numeric ascending
built.sortNumDesc(1);            // Sort numeric descending
built.filterEq(0, 'value');      // Filter exact match
built.filterHas(0, 'substr');    // Filter substring
built.render();                  // Render to string
```

### Convenience Functions

```javascript
import { createTable, renderRows } from 'crabular';

// Create table with headers
createTable([
  ['Name', 'Score'],
  ['Alice', '95'],
  ['Bob', '87']
], 'markdown');

// Render rows without headers
renderRows([
  ['Item 1', '$10'],
  ['Item 2', '$20']
], 'modern');
```

## Build

```bash
# For bundlers (webpack, rollup, etc.)
wasm-pack build --target bundler

# For Node.js
wasm-pack build --target nodejs

# For web
wasm-pack build --target web
```

## License

MIT
