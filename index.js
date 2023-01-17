// Copyright 2022 Heath Stewart.
// Licensed under the MIT License. See LICENSE.txt in the project root for license information.

import { getProductInfo, Package } from './pkg';

const packageFile = document.getElementById('package');
const info = document.getElementById('info');
const tables = document.getElementById('tables');

packageFile.onchange = () => {
    const reader = new FileReader();
    const file = packageFile.files[0];
    reader.readAsArrayBuffer(file);
    reader.onloadend = (e) => {
        if (e.target.readyState === FileReader.DONE) {
            const arrayBuffer = e.target.result;
            const array = new Uint8Array(arrayBuffer);
            const productInfo = getProductInfo(array);
            info.innerHTML = `<b>ProductName:</b> <code>${productInfo.name}</code><br><b>ProductVersion:</b> <code>${productInfo.version}</code>`;
            info.style = "display: block";

            const pkg = new Package(array);
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
            tables.innerHTML = names;
            tables.style = "display: block";
        }
    }
}
