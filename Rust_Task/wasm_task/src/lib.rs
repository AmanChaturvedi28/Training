use lazy_static::lazy_static;
use serde::{Deserialize, Serialize};
use wasm_bindgen::prelude::*;
use std::collections::HashMap;

#[derive(Debug, Deserialize, Serialize)]
pub struct NewCell {
    pub height: f64,
    pub width: f64,
    pub value: String,
}

impl NewCell {
    /// Function to assign new cell data
    pub fn data_assign(height: f64, width: f64, value: String) -> NewCell {
        NewCell {
            height,
            width,
            value,
        }
    }
}

#[derive(Debug, Deserialize, Serialize)]
pub struct NewRow {
    pub row_type: TypeofRow,
    pub height: f64,
    pub width: f64,
    pub cells: Vec<NewCell>,
    pub total_cells: usize,
}

impl NewRow {
    pub fn row_data(mut cells: Vec<NewCell>, row_type: TypeofRow) -> NewRow {
        let mut height: f64 = 0.0;
        let mut width: f64 = 0.0;
        let total_cells = cells.len();

        for i in 0..cells.len() {
            if height <= cells[i].height {
                height = cells[i].height;
            }
            width += cells[i].width;
        }

        let cell_max_height = &height;

        for i in 0..cells.len() {
            cells[i].height = *cell_max_height;
        }
        NewRow {
            height,
            width,
            cells,
            total_cells,
            row_type,
        }
    }
}

#[derive(Debug, Deserialize, Serialize)]
pub struct NewTable {
    pub height: f64,
    pub width: f64,
    pub rows: Vec<NewRow>,
    pub total_rows: usize,
}

impl NewTable {
    pub fn table_data(rows: Vec<NewRow>) -> NewTable {
        let mut height: f64 = 0.0;
        let mut width: f64 = 0.0;
        let total_rows = rows.len();

        for i in 0..rows.len() {
            if width <= rows[i].width {
                width = rows[i].width;
            }
            height += rows[i].height;
        }

        NewTable {
            height,
            width,
            rows,
            total_rows,
        }
    }
}

#[derive(Debug, Deserialize, Serialize)]
pub struct HeaderRow {
    #[serde(rename = "fontSize")]
    pub font_size: f64,
    pub title: Vec<String>,
}

#[derive(Debug, serde::Deserialize)]
pub struct DataRow {
    // pub row_type: TypeofRow,
    #[serde(rename = "fontSize")]
    pub font_size: f64,
    pub rows: Vec<Vec<String>>,
}

#[derive(Debug, serde::Deserialize)]
pub struct JsonData {
    #[serde(rename = "headerRow")]
    pub header_row: HeaderRow,
    #[serde(rename = "dataRows")]
    pub data_rows: DataRow,
    #[serde(rename = "pageWidth")]
    pub page_width: f64,
}

#[derive(Debug, Deserialize, Serialize)]
pub enum TypeofRow {
    HeaderRow,
    DataRow,
}

lazy_static! {
    static ref CHAR_WIDTH: HashMap<char, f64> = {
        let mut char_width_map = HashMap::new();
        char_width_map.insert('0', 0.5);
        char_width_map.insert('1', 0.5);
        char_width_map.insert('2', 0.5);
        char_width_map.insert('3', 0.5);
        char_width_map.insert('4', 0.5);
        char_width_map.insert('5', 0.5);
        char_width_map.insert('6', 0.5);
        char_width_map.insert('7', 0.5);
        char_width_map.insert('8', 0.5);
        char_width_map.insert('9', 0.5);
        char_width_map.insert(' ', 0.25);
        char_width_map.insert('!', 0.333);
        char_width_map.insert('\"', 0.555);
        char_width_map.insert('#', 0.5);
        char_width_map.insert('$', 0.5);
        char_width_map.insert('%', 1.0);
        char_width_map.insert('&', 0.83300006);
        char_width_map.insert('\'', 0.27800003);
        char_width_map.insert('(', 0.333);
        char_width_map.insert(')', 0.333);
        char_width_map.insert('*', 0.5);
        char_width_map.insert('+', 0.57000005);
        char_width_map.insert(':', 0.25);
        char_width_map.insert('-', 0.333);
        char_width_map.insert('.', 0.25);
        char_width_map.insert('/', 0.27800003);
        char_width_map.insert(',', 0.333);
        char_width_map.insert(';', 0.333);
        char_width_map.insert('<', 0.57000005);
        char_width_map.insert('=', 0.57000005);
        char_width_map.insert('>', 0.57000005);
        char_width_map.insert('?', 0.5);
        char_width_map.insert('@', 0.93000007);
        char_width_map.insert('A', 0.72200006);
        char_width_map.insert('B', 0.66700006);
        char_width_map.insert('C', 0.72200006);
        char_width_map.insert('D', 0.72200006);
        char_width_map.insert('E', 0.66700006);
        char_width_map.insert('F', 0.611);
        char_width_map.insert('G', 0.77800006);
        char_width_map.insert('H', 0.77800006);
        char_width_map.insert('I', 0.38900003);
        char_width_map.insert('J', 0.5);
        char_width_map.insert('K', 0.77800006);
        char_width_map.insert('L', 0.66700006);
        char_width_map.insert('M', 0.94400007);
        char_width_map.insert('N', 0.72200006);
        char_width_map.insert('O', 0.77800006);
        char_width_map.insert('P', 0.611);
        char_width_map.insert('Q', 0.77800006);
        char_width_map.insert('R', 0.72200006);
        char_width_map.insert('S', 0.55600005);
        char_width_map.insert('T', 0.66700006);
        char_width_map.insert('U', 0.72200006);
        char_width_map.insert('V', 0.72200006);
        char_width_map.insert('W', 1.0);
        char_width_map.insert('X', 0.72200006);
        char_width_map.insert('Y', 0.72200006);
        char_width_map.insert('Z', 0.66700006);
        char_width_map.insert('[', 0.333);
        char_width_map.insert('\\', 0.27800003);
        char_width_map.insert(']', 0.333);
        char_width_map.insert('^', 0.58100003);
        char_width_map.insert('_', 0.5);
        char_width_map.insert('`', 0.333);
        char_width_map.insert('a', 0.5);
        char_width_map.insert('b', 0.55600005);
        char_width_map.insert('c', 0.44400004);
        char_width_map.insert('d', 0.55600005);
        char_width_map.insert('e', 0.44400004);
        char_width_map.insert('f', 0.333);
        char_width_map.insert('g', 0.5);
        char_width_map.insert('h', 0.55600005);
        char_width_map.insert('i', 0.27800003);
        char_width_map.insert('j', 0.333);
        char_width_map.insert('k', 0.55600005);
        char_width_map.insert('l', 0.27800003);
        char_width_map.insert('m', 0.83300006);
        char_width_map.insert('n', 0.55600005);
        char_width_map.insert('o', 0.5);
        char_width_map.insert('p', 0.55600005);
        char_width_map.insert('q', 0.55600005);
        char_width_map.insert('r', 0.44400004);
        char_width_map.insert('s', 0.38900003);
        char_width_map.insert('t', 0.333);
        char_width_map.insert('u', 0.55600005);
        char_width_map.insert('v', 0.5);
        char_width_map.insert('w', 0.72200006);
        char_width_map.insert('x', 0.5);
        char_width_map.insert('y', 0.5);
        char_width_map.insert('z', 0.44400004);
        char_width_map.insert('{', 0.39400002);
        char_width_map.insert('|', 0.22000001);
        char_width_map.insert('}', 0.39400002);
        char_width_map.insert('~', 0.52000004);
        char_width_map
    };
}

#[wasm_bindgen]
pub fn process_table_data(data:String) -> String {
    let my_data: JsonData = serde_json::from_str(&data).expect("Failed to deserialize JSON");
    let padding = 1;
    let cell_width = my_data.page_width as f64 / my_data.header_row.title.len() as f64;

    let mut rows_data: Vec<NewRow> = Vec::new();

    let mut header_cells = Vec::new();

    for value in &my_data.header_row.title {
        let mut total_height: f64 = my_data.header_row.font_size;
        let mut total_width: f64 = 0.0;
        let mut area = cell_width - (padding * 2) as f64;
        let mut val = String::new();

        for each_char in value.chars() {
            total_width +=
                CHAR_WIDTH.get(&each_char).unwrap() * my_data.header_row.font_size;

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
                    CHAR_WIDTH.get(&each_char).unwrap() * my_data.data_rows.font_size;
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

   updated_table_json

}