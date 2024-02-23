const { process_table_data } = require('../wasm_task/pkg/wasm_task');

const fs = require('fs');

function processData() {
    fs.readFile('./json_data/data.json', 'utf-8', (err, jsonData) => {
        if (err) {
            console.error('Error reading file:', err);
            return;
        }
        const updatedTableJson = process_table_data(jsonData);
        
        fs.writeFileSync('./json_data/updated_table_json.json', updatedTableJson);
        
        console.log('File Created!');
    });
}

processData();