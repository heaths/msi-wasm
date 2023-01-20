// Copyright 2022 Heath Stewart.
// Licensed under the MIT License. See LICENSE.txt in the project root for license information.

import { getProductInfo, Package } from './pkg';

const packageFile = document.getElementById('package');
const info = document.getElementById('info');
const tables = document.getElementById('tables');
const rows = document.getElementById('properties');

packageFile.onchange = () => {
    const reader = new FileReader();
    const file = packageFile.files[0];
    reader.readAsArrayBuffer(file);
    reader.onloadend = (e) => {
        if (e.target.readyState === FileReader.DONE) {
            const arrayBuffer = e.target.result;
            const array = new Uint8Array(arrayBuffer);
            try {
                const pkg = new Package(array);
                render(pkg);
            } catch (e) {
                window.alert(`Failed to load the package: ${e}`);
            }
        }
    }
}

function render(pkg) {
    const productInfo = pkg.productInfo();
    if (productInfo) {
        info.innerHTML = `<b>ProductName:</b> <code>${productInfo.name}</code><br>` +
                        `<b>ProductVersion:</b> <code>${productInfo.version}</code><br>` +
                        `<b>Manufacturer:</b> <code>${productInfo.manufacturer}</code>`;
        if (productInfo.upgradeCode) {
            info.innerHTML += `<br><b>UpgradeCode:</b> <code>${productInfo.upgradeCode}</code>`;
        }
        info.style = "display: block";
    } else {
        info.style = "display: none";
    }

    const _tables = pkg.tables();
    let names = "<ul>";
    for (let i in _tables) {
        const _table = _tables[i];
        const _columns = _table.columns().map((c) => {
            if (c.primaryKey) {
               return `<b>${c.name}</b>`;
            }

            if (c.nullable) {
                return `${c.name}?`;
            }

            return c.name
        });
        names += `<li>${_table.name} (${_columns.join(", ")})</li>`
    }
    names += "</ul>";
    tables.innerHTML = "<p><b>Tables</b></p>" + names;
    tables.style = "display: block";

    try {
        const _props = pkg.rows("Property").sort((a, b) => a['Property'] > b['Property'] ? 1 : -1);
        let _properties = "<table><tr><th>Property</th><th>Value</th></tr>";
        for (let i in _props) {
            const prop = _props[i];
            _properties += `<tr><td>${prop['Property']}</td><td>${prop['Value']}</td></tr>`;
        }
        _properties += "</table>";
        rows.innerHTML = "<p><b>Properties</b></p>" + _properties;
        rows.style = "display: block";
    } catch {
        console.log("'Property' table not found");
        rows.style = "display: none";
    }
}
