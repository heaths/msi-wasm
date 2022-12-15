// Copyright 2022 Heath Stewart.
// Licensed under the MIT License. See LICENSE.txt in the project root for license information.

use msi_wasm::read_product_info;
use std::env;
use std::fs::File;
use std::io::BufReader;
use std::process;

fn main() {
    let args: Vec<String> = env::args().collect();
    if args.len() != 2 {
        eprintln!("Usage: fileinfo <MSI>");
        process::exit(1);
    }

    let file = File::open(&args[1]).expect("expected file");
    let reader = BufReader::new(file);

    let info = read_product_info(reader).expect("failed to get product info");
    println!("ProductName: {}", info.name());
    println!("ProductVersion: {}", info.version());
}
