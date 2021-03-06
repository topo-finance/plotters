// If you only use `npm` you can simply
// import { Chart } from "wasm-demo" and remove `setup` call from `bootstrap.js`.
class Chart {}

const canvas = document.getElementById("canvas");
const coord = document.getElementById("coord");
const plotType = document.getElementById("plot-type");
const status = document.getElementById("status");

let chart = null;

/** Main entry point */
export function main() {
    setupUI();
    setupCanvas();
}

/** This function is used in `bootstrap.js` to setup imports. */
export function setup(WasmChart) {
    Chart = WasmChart;
}

/** Add event listeners. */
function setupUI() {
    status.innerText = "WebAssembly loaded!";
    plotType.addEventListener("change", updatePlot);
    window.addEventListener("resize", setupCanvas);
    window.addEventListener("mousemove", onMouseMove);
}

/** Setup canvas to properly handle high DPI and redraw current plot. */
function setupCanvas() {
    const dpr = window.devicePixelRatio || 1;
    const aspectRatio = canvas.width / canvas.height;
    const size = Math.min(canvas.width, canvas.parentNode.offsetWidth);
    canvas.style.width = size + "px";
    canvas.style.height = size / aspectRatio + "px";
    canvas.width = size * dpr;
    canvas.height = size / aspectRatio * dpr;
    canvas.getContext("2d").scale(dpr, dpr);
    updatePlot();
}

/** Update displayed coordinates. */
function onMouseMove(event) {
    if (chart) {
        const point = chart.coord(event.offsetX, event.offsetY);
        coord.innerText = (point)
            ? `(${point.x.toFixed(3)}, ${point.y.toFixed(3)})`
            : "Mouse pointer is out of range";
    }
}

/** Redraw currently selected plot. */
function updatePlot() {
    const selected = plotType.selectedOptions[0];
    status.innerText = `Rendering ${selected.innerText}...`;
    chart = null;
    const start = performance.now();
    chart = (selected.value == "mandelbrot")
        ? Chart.mandelbrot(canvas)
        : Chart.power("canvas", Number(selected.value));
    const end = performance.now();
    status.innerText = `Rendered ${selected.innerText} in ${Math.ceil(end - start)}ms`;
}
