// Copyright 2022 Heath Stewart.
// Licensed under the MIT License. See LICENSE.txt in the project root for license information.

use msi::{Package, Select};
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
pub fn get_product_info(data: &[u8]) -> Result<ProductInfo, JsValue> {
    let cursor = Cursor::new(data);

    console_log!("opening package from {} bytes", data.len());
    let info = read_product_info(cursor).unwrap_throw();

    Ok(info)
}

pub fn read_product_info<R>(data: R) -> Result<ProductInfo, Box<dyn Error + Send + Sync>>
where
    R: Read + Seek,
{
    let mut package = Package::open(data)?;
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
