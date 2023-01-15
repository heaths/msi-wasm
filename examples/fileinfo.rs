// Copyright 2022 Heath Stewart.
// Licensed under the MIT License. See LICENSE.txt in the project root for license information.

use clap::Parser;
use msi_wasm::read_product_info;
use std::fs::File;
use std::io::BufReader;
use std::path::PathBuf;

fn main() {
    let args = Args::parse();
    let file = File::open(args.path).expect("expected file");
    let reader = BufReader::new(file);

    let info = read_product_info(reader).expect("failed to get product info");
    println!("ProductName: {}", info.name());
    println!("ProductVersion: {}", info.version());
}

#[derive(Parser)]
#[command(author, version, about, long_about = None)]
struct Args {
    /// Path to a Windows Installer package (MSI)
    #[arg(value_name = "MSI")]
    path: PathBuf,
}
