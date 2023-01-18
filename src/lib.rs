// Copyright 2022 Heath Stewart.
// Licensed under the MIT License. See LICENSE.txt in the project root for license information.

use js_sys::Array;
use msi::Select;
use std::collections::HashMap;
use std::error::Error;
use std::fmt::Display;
use std::io::{Cursor, Read, Seek};
use std::ops::Index;
use wasm_bindgen::prelude::*;

#[wasm_bindgen]
extern "C" {
    #[wasm_bindgen(js_namespace = console)]
    fn log(s: &str);
}

#[cfg(target_family = "wasm")]
macro_rules! console_log {
    ($($t:tt)*) => (log(&format_args!($($t)*).to_string()))
}

#[cfg(not(target_family = "wasm"))]
macro_rules! console_log {
    ($($t:tt)*) => (eprintln!($($t)*))
}

#[wasm_bindgen]
pub struct Package {
    package: msi::Package<Cursor<Vec<u8>>>,
}

#[wasm_bindgen]
impl Package {
    #[wasm_bindgen(constructor)]
    pub fn new(data: Vec<u8>) -> Result<Package, JsError> {
        let cursor = Cursor::new(data);
        let p = msi::Package::open(cursor)?;
        Ok(Package { package: p })
    }

    #[wasm_bindgen]
    pub fn tables(&self) -> JsValue {
        JsValue::from(
            self.package
                .tables()
                .into_iter()
                .map(|t| <Table as Into<JsValue>>::into(Table::new(t)))
                .collect::<Array>(),
        )
    }

    #[wasm_bindgen]
    pub fn rows(&mut self, table: &str) -> Result<JsValue, JsError> {
        if !self.package.has_table(table) {
            return Err(JsError::new(format!("table {} not found", table).as_str()));
        }

        Ok(JsValue::from(
            self.package
                .select_rows(Select::table(table))?
                .into_iter()
                .map(|r| {
                    let mut obj = HashMap::with_capacity(r.len());
                    for i in 0..r.len() {
                        obj.insert(
                            r.columns()[i].name().to_string(),
                            r.index(i).as_str().map(|s| s.to_string()),
                        );
                    }
                    serde_wasm_bindgen::to_value(&obj).unwrap_or_default()
                })
                .collect::<Array>(),
        ))
    }
}

#[wasm_bindgen]
pub struct Table {
    name: String,
    columns: Vec<Column>,
}

#[wasm_bindgen]
impl Table {
    fn new(table: &msi::Table) -> Self {
        Table {
            name: table.name().into(),
            columns: table.columns().iter().map(Column::new).collect(),
        }
    }

    #[wasm_bindgen(getter)]
    pub fn name(&self) -> String {
        self.name.clone()
    }

    #[wasm_bindgen]
    pub fn columns(&self) -> JsValue {
        JsValue::from(
            self.columns
                .as_slice()
                .iter()
                .map(|c| <Column as Into<JsValue>>::into(c.clone()))
                .collect::<Array>(),
        )
    }
}

#[derive(Clone)]
#[wasm_bindgen]
pub struct Column {
    name: String,
    column_type: String,
    category: Option<String>,
    primary_key: bool,
    nullable: bool,
    localizable: bool,
}

#[wasm_bindgen]
impl Column {
    fn new(column: &msi::Column) -> Self {
        Column {
            name: column.name().into(),
            // cspell:ignore coltype
            column_type: format!("{}", column.coltype()),
            category: column.category().map(|c| format!("{}", c)),
            primary_key: column.is_primary_key(),
            nullable: column.is_nullable(),
            localizable: column.is_localizable(),
        }
    }

    #[wasm_bindgen(getter)]
    pub fn name(&self) -> String {
        self.name.clone()
    }

    #[wasm_bindgen(getter, js_name = "columnType")]
    pub fn column_type(&self) -> String {
        self.column_type.clone()
    }

    #[wasm_bindgen(getter)]
    pub fn category(&self) -> Option<String> {
        self.category.clone()
    }

    #[wasm_bindgen(getter, js_name = "primaryKey")]
    pub fn primary_key(&self) -> bool {
        self.primary_key
    }

    #[wasm_bindgen(getter)]
    pub fn nullable(&self) -> bool {
        self.nullable
    }

    #[wasm_bindgen(getter)]
    pub fn localizable(&self) -> bool {
        self.localizable
    }
}

#[derive(Debug, Default)]
#[wasm_bindgen]
pub struct ProductInfo {
    name: String,
    version: String,
}

#[wasm_bindgen]
impl ProductInfo {
    // Use explicit getters since public fields cannot be of type String.
    #[wasm_bindgen(getter)]
    pub fn name(&self) -> String {
        self.name.clone()
    }

    #[wasm_bindgen(getter)]
    pub fn version(&self) -> String {
        self.version.clone()
    }
}

#[wasm_bindgen(js_name = "getProductInfo")]
pub fn get_product_info(data: Vec<u8>) -> Result<ProductInfo, JsError> {
    console_log!("opening package from {} bytes", data.len());
    let cursor = Cursor::new(data);
    let info = read_product_info(cursor).unwrap_throw();

    Ok(info)
}

pub fn read_product_info<R>(data: R) -> Result<ProductInfo, Box<dyn Error + Send + Sync>>
where
    R: Read + Seek,
{
    let mut package = msi::Package::open(data)?;
    if !package.has_table("Property") {
        return Err(Box::new(MsiError::TableNotFound("Property".to_owned())));
    }

    console_log!("enumerating Property table");
    let columns = vec!["Property", "Value"];
    let query = Select::table("Property").columns(&columns);
    let rows = package.select_rows(query)?;

    let mut info = ProductInfo::default();
    for row in rows {
        let property = row.index(0).as_str().unwrap();
        let value = row.index(1).as_str().unwrap();

        match property {
            "ProductName" => info.name = value.to_owned(),
            "ProductVersion" => info.version = value.to_owned(),
            _ => console_log!("skipping property {}={}", property, value),
        }
    }

    Ok(info)
}

#[derive(Debug)]
enum MsiError {
    TableNotFound(String),
}

impl Display for MsiError {
    fn fmt(&self, f: &mut std::fmt::Formatter<'_>) -> std::fmt::Result {
        match self {
            Self::TableNotFound(name) => write!(f, "table '{}' not found", name),
        }
    }
}

impl Error for MsiError {
    fn source(&self) -> Option<&(dyn Error + 'static)> {
        None
    }
}
