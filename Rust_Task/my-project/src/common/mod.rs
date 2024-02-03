use serde::{Deserialize, Serialize};
use chrono::Utc;
use chrono::DateTime;

///Student details
#[derive(Debug, Deserialize, Serialize)]
pub struct Student {
    pub name: String,
    pub phone: String,
    pub email: String,
    pub city: String,
    pub address: String,
    pub marks: Vec<u32>,
    pub total: Option<u32>,
    pub percentage: Option<f64>,
    pub grade: Option<String>,
}

///Employee details
#[derive(Debug, Deserialize, Serialize)]
pub struct Employee {
    pub name: String,
    pub age: i32,
    pub skills: Vec<Skill>,
    pub position: Option<Position>,
    pub experience: Option<i32>,
}

///Positions for employee
#[derive(Debug, Deserialize, Serialize, PartialEq)]
pub enum Position {
    #[serde(rename(serialize = "Software Developer", deserialize = "Software Developer"))]
    SoftwareDeveloper,

    #[serde(rename(serialize = "Jr. Software Developer", deserialize = "Jr. Software Developer"))]
    JrSoftwareDeveloper,

    #[serde(rename(serialize = "Sr. Software Developer", deserialize = "Sr. Software Developer"))]
    SrSoftwareDeveloper,

    #[serde(rename(serialize = "Project Manager", deserialize = "Project Manager"))]
    ProjectManager,

    #[serde(rename(serialize = "Team Lead", deserialize = "Team Lead"))]
    TeamLead,
}

///Skills of employee
#[derive(Debug, Deserialize, Serialize, PartialEq)]
pub enum Skill {
    Rust,
    Java,

    #[serde(rename(serialize = "C#", deserialize = "C#"))]
    CSharp,
    Python,
}

/// Table's cell data
#[derive(Debug)]
pub struct Cell {
    pub height: u32,
    pub width: u32,
    pub value: u32,
}

impl Cell {
    /// Function to assign cell data 
    pub fn data_assign(height: u32, width: u32, value: u32) -> Cell {
        Cell { height, width, value }
    }
}

/// Table's row data
#[derive(Debug)]
pub struct Row {
    pub height: u32,
    pub width: u32,
    pub cells: Vec<Cell>,
    pub total_cells: usize,
}

impl Row {
    /// Function to assign row data 
    pub fn row_data(cells: Vec<Cell>) -> Row {
        let mut height: u32 = 0;
        let mut width: u32 = 0;
        let total_cells = cells.len();

        for i in 0..cells.len() {
            if height <= cells[i].height {
                height = cells[i].height;
            }
            width += cells[i].width;
        }

        Row {
            height,
            width,
            cells,
            total_cells,
        }
    }

    /// Function to update row data
    pub fn update_row_data(&mut self) {
        self.height = 0;
        self.width = 0;

        for i in 0..self.cells.len() {
            if self.height <= self.cells[i].height {
                self.height = self.cells[i].height;
            }
            self.width += self.cells[i].width;
        }
    }
}

/// Table  data
#[derive(Debug)]
pub struct Table {
    pub height: u32,
    pub width: u32,
    pub rows: Vec<Row>,
    pub total_rows: usize,
}

impl Table {
    /// Function to assign table data
    pub fn table_data(rows: Vec<Row>) -> Table {
        let mut height: u32 = 0;
        let mut width: u32 = 0;
        let total_rows = rows.len();

        for i in 0..rows.len() {
            if width <= rows[i].width {
                width = rows[i].width;
            }
            height += rows[i].height;
        }

        Table {
            height,
            width,
            rows,
            total_rows,
        }
    }

    /// Function to update table data
    pub fn update_table_data(&mut self) {
        self.height = 0;
        self.width = 0;

        for i in 0..self.rows.len() {
            if self.width <= self.rows[i].width {
                self.width = self.rows[i].width;
            }
            self.height += self.rows[i].height;
        }
    }
}

///structure for new cell
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

///structure for new row
#[derive(Debug, Deserialize, Serialize)]
pub struct NewRow {
    pub row_type: TypeofRow,
    pub height: f64,
    pub width: f64,
    pub cells: Vec<NewCell>,
    pub total_cells: usize,
}

impl NewRow {
    /// Function to assign new row data
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

///structure for new table data
#[derive(Debug, Deserialize, Serialize)]
pub struct NewTable {
    pub height: f64,
    pub width: f64,
    pub rows: Vec<NewRow>,
    pub total_rows: usize,
}

impl NewTable {
    /// Function to assign table data
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

///structure for headerrow in json data
#[derive(Debug, Deserialize, Serialize)]
pub struct HeaderRow {
    #[serde(rename = "fontSize")]
    pub font_size: f64,
    pub title: Vec<String>,
}

///structure for datarow in json data
#[derive(Debug, serde::Deserialize)]
pub struct DataRow {
    // pub row_type: TypeofRow,
    #[serde(rename = "fontSize")]
    pub font_size: f64,
    pub rows: Vec<Vec<String>>,
}

///structure for json data
#[derive(Debug, serde::Deserialize)]
pub struct JsonData {
    #[serde(rename = "headerRow")]
    pub header_row: HeaderRow,
    #[serde(rename = "dataRows")]
    pub data_rows: DataRow,
    #[serde(rename = "pageWidth")]
    pub page_width: f64,
}

///type of row in table
#[derive(Debug, Deserialize, Serialize)]
pub enum TypeofRow {
    HeaderRow,
    DataRow,
}

///structure for data that works on thread
#[derive(Debug)]
pub struct Data{
    pub id: i32,
    pub name: String,
    pub timestamp: DateTime<Utc>
}

impl Data {
    ///function that assign data to structure
    pub fn data_assign(id: i32,name: String,timestamp: DateTime<Utc>) -> Self{
        Self{
            id,
            name,
            timestamp
        }
    }
}