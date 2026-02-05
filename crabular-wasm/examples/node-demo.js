const fs = require('fs');
const path = require('path');

// Import the WASM module
const wasmPath = path.join(__dirname, '..', 'pkg', 'crabular_wasm.js');

async function main() {
    // For wasm-pack --target nodejs, the WASM is automatically initialized
    // The module exports are ready to use immediately
    const crabular = await import(wasmPath);
    
    console.log('=== Crabular WASM Node.js Examples ===\n');
    
    // Example 1: Basic usage
    console.log('Example 1: Basic Table');
    console.log('----------------------');
    const table1 = new crabular.JsTable();
    table1.style('modern');
    table1.header(['Name', 'Age', 'City']);
    table1.row(['Alice', '30', 'New York']);
    table1.row(['Bob', '25', 'London']);
    table1.row(['Charlie', '35', 'Tokyo']);
    console.log(table1.render());
    console.log();
    
    // Example 2: Markdown style
    console.log('Example 2: Markdown Style');
    console.log('-------------------------');
    const table2 = new crabular.JsTable();
    table2.style('markdown');
    table2.header(['Product', 'Price', 'Stock']);
    table2.row(['Widget', '$10.00', '50']);
    table2.row(['Gadget', '$25.00', '30']);
    table2.row(['Doohickey', '$15.00', '100']);
    console.log(table2.render());
    console.log();
    
    // Example 3: Right-aligned numbers
    console.log('Example 3: Right-Aligned Numbers');
    console.log('---------------------------------');
    const table3 = new crabular.JsTable();
    table3.style('modern');
    table3.header(['Item', 'Price', 'Quantity', 'Total']);
    table3.row(['Apple', '$1.50', '10', '$15.00']);
    table3.row(['Banana', '$0.75', '20', '$15.00']);
    table3.row(['Cherry', '$2.00', '5', '$10.00']);
    table3.align(1, 'right');
    table3.align(2, 'right');
    table3.align(3, 'right');
    console.log(table3.render());
    console.log();
    
    // Example 4: Sorting
    console.log('Example 4: Sorted by Score (Descending)');
    console.log('---------------------------------------');
    const table4 = new crabular.JsTable();
    table4.style('modern');
    table4.header(['Player', 'Score']);
    table4.row(['Charlie', '850']);
    table4.row(['Alice', '1200']);
    table4.row(['Bob', '920']);
    table4.row(['Diana', '780']);
    const built4 = table4.build();
    built4.sortNumDesc(1);
    console.log(built4.render());
    console.log();
    
    // Example 5: All styles comparison
    console.log('Example 5: Style Comparison');
    console.log('---------------------------');
    const styles = ['classic', 'modern', 'minimal', 'compact', 'markdown'];
    const data = {
        header: ['Style', 'Description'],
        rows: [
            ['classic', 'ASCII borders with + - |'],
            ['modern', 'Unicode box drawing chars'],
            ['minimal', 'Horizontal lines only'],
            ['compact', 'No outer frame'],
            ['markdown', 'GitHub-flavored markdown']
        ]
    };
    
    for (const style of styles) {
        console.log(`${style.toUpperCase()}:`);
        const t = new crabular.JsTable();
        t.style(style);
        t.header(data.header);
        for (const row of data.rows) {
            t.row(row);
        }
        console.log(t.render());
        console.log();
    }
    
    // Example 6: Filtering
    console.log('Example 6: Filtered Table (Active only)');
    console.log('---------------------------------------');
    const table6 = new crabular.JsTable();
    table6.style('modern');
    table6.header(['Name', 'Status']);
    table6.row(['Alice', 'Active']);
    table6.row(['Bob', 'Inactive']);
    table6.row(['Charlie', 'Active']);
    table6.row(['Diana', 'Pending']);
    const built6 = table6.build();
    built6.filterEq(1, 'Active');
    console.log(built6.render());
    console.log();
}

main().catch(console.error);
