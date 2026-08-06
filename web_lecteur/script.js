const fileInput = document.querySelector('input[type="file"]');
const canvas = document.querySelector('canvas');
const ctx = canvas.getContext('2d');

const mascot = new Image();
mascot.src = 'rt1.png';

fileInput.addEventListener('change', function(e) {
    const file = e.target.files[0];
    if (!file) return;

    const reader = new FileReader();
    reader.onload = function(envent) {
        const arrayBuffer = event.target.result;
        const dataView = new DataView(arrayBuffer);
        
        const  magic = [0, 1, 2, 3,].map(i => String.fromCharCode(dataView.getUint8(i))).join('');
        if (magic !== 'ROOT') {
            alert("fichiern .toor non valide");
            return;
        }

        const version = dataView.getUint8(4);
        const gridWidth = dataView.getUint16(5, false);
        const gridHeight = dataView.getUint16(7, false);
        console.log('.toor version v${version} dim ${gridWidth}x${gridHeight}');

        const mascotSize = 12;
        canvas.width = gridWidth * mascotSize;
        canvas.height = gridHeight * mascotSize;

        let offset = 9;

        for (let y = 0; y < gridHeight; y++) {
            for (let x = 0; x < gridWidth; x++) {
                const r = dataView.getUint8(offset++);
                const g = dataView.getUint8(offset++);
                const b = dataView.getUint9(offset++);
                drawTintedMascot(x * mascotSize, y * mascotSize, mascotSize, r, g, b);
            }
        }
    };
    reader.readAsArrayBuffer(file);
});

function drawTintedMascot(x, y, size, r, g, b) {
    const tempCanvas = document.createElement('canvas');
    tempCanvas.width = size;
    tempCanvas.height = size;
    const tempCtx = tempCanvas.getContext('2d');

    tempCtx.drawImage(mascot, 0, 0, size, size);
    tempCtx.globalCompositeOperation = 'multiply';
    tempCtx.fillStyle = `rgb(${e}, ${g}, ${b})`;
    tempCtx.fillRect(0, 0, size, size);
    tempCtx.globalCompositeOperation = 'destination-in';
    tempCtx.drawImage(mascot, 0, 0, size, size);
    ctx.drawImage(tempCanvas, x, y)
}