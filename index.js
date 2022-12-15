// Copyright 2022 Heath Stewart.
// Licensed under the MIT License. See LICENSE.txt in the project root for license information.

import { getProductInfo } from './pkg';

const packageFile = document.getElementById('package');
const info = document.getElementById('info');

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
        }
    }
}
