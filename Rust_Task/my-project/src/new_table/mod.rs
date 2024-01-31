use crate::common::{JsonData, NewCell, NewRow, NewTable, TypeofRow};
use lazy_static::lazy_static;
use std::collections::HashMap;
use std::fs;

///data of all character size given
lazy_static! {
    static ref CHAR_WIDTH: HashMap<String, f64> = {
        let json_data = fs::read_to_string("json_data/NewTableData/fonrData.json")
            .expect("Failed to read the JSON file");

        let json_data: HashMap<String, f64> =
            serde_json::from_str(&json_data).expect("Failed to deserialize JSON");

        let mut width_map = HashMap::new();

        for (key, value) in json_data {
            width_map.insert(key, value);
        }
        width_map
    };
}

///function to set the data in table according to given height,width and font size
pub fn new_table() {
    let data = fs::read_to_string("json_data/NewTableData/data.json")
        .expect("Failed to read the JSON file");
    let my_data: JsonData = serde_json::from_str(&data).expect("Failed to deserialize JSON");
    let padding = 1;
    let cell_width = (my_data.page_width as f64 / my_data.header_row.title.len() as f64);

    let mut rows_data: Vec<NewRow> = Vec::new();

    let mut header_cells = Vec::new();

    for value in &my_data.header_row.title {
        let mut total_height: f64 = my_data.header_row.font_size;
        let mut total_width: f64 = 0.0;
        let mut area = cell_width - (padding * 2) as f64;
        let mut val = String::new();

        for each_char in value.chars() {
            total_width +=
                (CHAR_WIDTH.get(&each_char.to_string()).unwrap() * my_data.header_row.font_size);

            if total_width > area {
                total_height += my_data.header_row.font_size;
                area += cell_width - (padding * 2) as f64;
                val.push('\n');
            }
            val.push(each_char);
        }
        total_width = cell_width;
        total_height += (padding * 2) as f64;
        header_cells.push(NewCell::data_assign(total_height, total_width, val));
    }

    let header_row = NewRow::row_data(header_cells, TypeofRow::HeaderRow);
    rows_data.push(header_row);

    for data_row in &my_data.data_rows.rows {
        let mut cells: Vec<NewCell> = Vec::new();

        for value in data_row {
            let mut total_height: f64 = my_data.data_rows.font_size;
            let mut total_width: f64 = 0.0;
            let mut area = cell_width - (padding * 2) as f64;
            let mut val = String::new();

            for each_char in value.chars() {
                total_width +=
                    CHAR_WIDTH.get(&each_char.to_string()).unwrap() * my_data.data_rows.font_size;
                if total_width > area {
                    total_height += my_data.data_rows.font_size;
                    area += cell_width - (padding * 2) as f64;
                    val.push('\n');
                }
                val.push(each_char);
            }
            total_height += (padding * 2) as f64;
            total_width = cell_width;
            cells.push(NewCell::data_assign(total_height, total_width, val));
        }

        let row = NewRow::row_data(cells, TypeofRow::DataRow);
        rows_data.push(row);
    }

    let table = NewTable::table_data(rows_data);

    let updated_table_json =
        serde_json::to_string_pretty(&table).expect("Failed to serialize JSON");

    fs::write(
        "json_data/NewTableData/updated_table_json.json",
        updated_table_json,
    )
    .expect("Failed to write file");

    println!("File Created!");
}