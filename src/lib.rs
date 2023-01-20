// Copyright 2022 Heath Stewart.
// Licensed under the MIT License. See LICENSE.txt in the project root for license information.

use msi::Select;
use serde::Serialize;
use std::collections::HashMap;
use std::io::Cursor;
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
        let package = msi::Package::open(cursor)?;
        Ok(Package { package })
    }

    #[wasm_bindgen(js_name = "productInfo")]
    pub fn product_info(&mut self) -> Option<ProductInfo> {
        if !self.package.has_table("Property") {
            return None;
        }

        let columns = vec!["Property", "Value"];
        let query = Select::table("Property").columns(&columns);
        let rows = self.package.select_rows(query).ok()?;

        let mut info = ProductInfo::default();
        for row in rows {
            let property = row.index(0).as_str()?;
            let value = row.index(1).as_str()?;

            match property {
                "ProductName" => info.name = value.to_owned(),
                "ProductVersion" => info.version = value.to_owned(),
                "Manufacturer" => info.manufacturer = value.to_owned(),
                "UpgradeCode" => info.upgrade_code = Some(value.to_owned()),
                _ => console_log!("skipping property {}={}", property, value),
            }
        }

        Some(info)
    }

    #[wasm_bindgen]
    pub fn tables(&self) -> Box<[JsValue]> {
        self.package
            .tables()
            .into_iter()
            .map(|t| <Table as Into<JsValue>>::into(Table::new(t)))
            .collect()
    }

    #[wasm_bindgen]
    pub fn rows(&mut self, table: &str) -> Result<Box<[JsValue]>, JsError> {
        if !self.package.has_table(table) {
            return Err(JsError::new(format!("table {} not found", table).as_str()));
        }

        Ok(self
            .package
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
                let serializer = serde_wasm_bindgen::Serializer::json_compatible();
                obj.serialize(&serializer).unwrap_or_default()
            })
            .collect())
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
    pub fn columns(&self) -> Box<[JsValue]> {
        self.columns
            .as_slice()
            .iter()
            .map(|c| <Column as Into<JsValue>>::into(c.clone()))
            .collect()
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
    manufacturer: String,
    upgrade_code: Option<String>,
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

    #[wasm_bindgen(getter)]
    pub fn manufacturer(&self) -> String {
        self.manufacturer.clone()
    }

    #[wasm_bindgen(getter, js_name = "upgradeCode")]
    pub fn upgrade_code(&self) -> Option<String> {
        self.upgrade_code.clone()
    }
}
