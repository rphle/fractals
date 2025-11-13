<script>
import { base } from '$app/paths';
  import { onMount, onDestroy } from 'svelte';
  import { fade } from 'svelte/transition';
  import { ZoomOut, ZoomIn, RotateCcw } from '@lucide/svelte';

  let fractalCanvas;
  let overlayCanvas;
  let wasm;
  let renderer;

  const width = 1000;
  const height = 750;
  const aspectRatio = width / height;

  let bounds = { xMin: -2.5, xMax: 1.0, yMin: -1.25, yMax: 1.25 };
  let maxIter = 256;
  let rendering = false;
  let renderTime = 0;

  let fractalType = 0;
  let juliaParams = { re: -0.7, im: 0.27015 };

  let selectionSize = 200;
  let mousePos = { x: width / 2, y: height / 2 };

  onMount(async () => {
    try {
      const moduleUrl = `${base}/fractal/fractal.js`;
      const wasmUrl = `${base}/fractal/fractal_bg.wasm`;

      const wasmModule = await import(/* @vite-ignore */ moduleUrl);
      const wasmInstance = await wasmModule.default({
        module: fetch(wasmUrl).then(r => r.arrayBuffer()),
      });

      wasm = wasmInstance;
      renderer = wasmModule.FractalRenderer.new(width, height);

      window.addEventListener("keydown", onKeyDown);

      render();
      drawOverlay();
    } catch (err) {
      console.error("Failed to load WASM:", err);
    }
  });

  onDestroy(() => {
    window.removeEventListener("keydown", onKeyDown);
  });

  function render() {
    if (!fractalCanvas || !wasm || !renderer) return;
    rendering = true;

    requestAnimationFrame(() => {
      const ctx = fractalCanvas.getContext("2d", { alpha: false });
      const t0 = performance.now();

      const b = ensureAspect(bounds);
      renderer.set_fractal_type(fractalType);
      if (fractalType === 1) renderer.set_julia_constant(juliaParams.re, juliaParams.im);

      renderer.render(b.xMin, b.xMax, b.yMin, b.yMax, maxIter);

      const ptr = renderer.get_buffer_ptr();
      const len = renderer.get_buffer_len();
      const memoryBuffer = wasm.memory.buffer;
      const pixelView = new Uint8ClampedArray(memoryBuffer, ptr, len);
      const imageData = new ImageData(pixelView, width, height);

      ctx.putImageData(imageData, 0, 0);
      renderTime = performance.now() - t0;
      rendering = false;
    });
  }

  function ensureAspect(b) {
    const dx = b.xMax - b.xMin, dy = b.yMax - b.yMin;
    const ar = dx / dy;
    if (Math.abs(ar - aspectRatio) < 1e-9) return b;

    const cx = (b.xMax + b.xMin) / 2, cy = (b.yMax + b.yMin) / 2;
    if (ar > aspectRatio) {
      const newDy = dx / aspectRatio;
      return { xMin: b.xMin, xMax: b.xMax, yMin: cy - newDy / 2, yMax: cy + newDy / 2 };
    }
    const newDx = dy * aspectRatio;
    return { xMin: cx - newDx / 2, xMax: cx + newDx / 2, yMin: b.yMin, yMax: b.yMax };
  }

  function zoom(factor) {
    const { xMin, xMax, yMin, yMax } = bounds;
    const cx = (xMax + xMin) / 2, cy = (yMax + yMin) / 2;
    const dx = (xMax - xMin) * factor, dy = (yMax - yMin) * factor;

    bounds = ensureAspect({
      xMin: cx - dx / 2,
      xMax: cx + dx / 2,
      yMin: cy - dy / 2,
      yMax: cy + dy / 2
    });

    maxIter = factor < 1 ? Math.floor(maxIter * 1.1) : Math.max(64, Math.floor(maxIter / 1.3));
    render();
  }

  const zoomIn = () => zoom(0.6);
  const zoomOut = () => zoom(1.5);
  const reset = () => {
    maxIter = 256;
    handleChange();
  };

  const handleChange = () => {
    if (fractalType === 1) { // Julia default view
      bounds = { xMin: -2.0, xMax: 2.0, yMin: -1.5, yMax: 1.5 };
    } else {
      bounds = { xMin: -2.5, xMax: 1.0, yMin: -1.25, yMax: 1.25 };
    }
    render();
  };

  function onKeyDown(e) {
    const dx = bounds.xMax - bounds.xMin;
    const dy = bounds.yMax - bounds.yMin;
    const step = 0.1;

    const keys = {
      ArrowLeft: [-(dx * step), 0],
      ArrowRight: [dx * step, 0],
      ArrowUp: [0, dy * step],
      ArrowDown: [0, -(dy * step)]
    };

    if (keys[e.key]) {
      const [x, y] = keys[e.key];
      [bounds.xMin, bounds.xMax].forEach(b => b += x);
      [bounds.yMin, bounds.yMax].forEach(b => b += y);
      e.preventDefault();
      render();
    }
  }

  function drawOverlay() {
    const ctx = overlayCanvas?.getContext("2d");
    if (!ctx) return;

    ctx.clearRect(0, 0, width, height);
    const hw = selectionSize / 2, hh = hw / aspectRatio;

    ctx.strokeStyle = "rgba(255, 255, 255, 0.8)";
    ctx.lineWidth = 2;
    ctx.setLineDash([6, 3]);
    ctx.strokeRect(mousePos.x - hw, mousePos.y - hh, hw * 2, hh * 2);

    ctx.strokeStyle = "rgba(0, 0, 0, 0.5)";
    ctx.setLineDash([]);
    ctx.strokeRect(mousePos.x - hw - 1, mousePos.y - hh - 1, hw * 2 + 2, hh * 2 + 2);
  }

  function canvasToComplex(x, y) {
    const sX = (bounds.xMax - bounds.xMin) / width;
    const sY = (bounds.yMax - bounds.yMin) / height;
    return { cx: bounds.xMin + x * sX, cy: bounds.yMax - y * sY };
  }

  function onMouseMove(e) {
    const r = overlayCanvas.getBoundingClientRect();
    mousePos = { x: e.clientX - r.left, y: e.clientY - r.top };
    drawOverlay();
  }

  function onWheel(e) {
    e.preventDefault();
    selectionSize = Math.max(20, Math.min(selectionSize * (e.deltaY < 0 ? 1.1 : 0.9), Math.min(width, height)));
    drawOverlay();
  }

  function onClick() {
    const hw = selectionSize / 2, hh = hw / aspectRatio;
    const left = mousePos.x - hw, right = mousePos.x + hw;
    const top = mousePos.y - hh, bottom = mousePos.y + hh;

    const { cx: xMin, cy: yMin } = canvasToComplex(left, bottom);
    const { cx: xMax, cy: yMax } = canvasToComplex(right, top);

    bounds = ensureAspect({ xMin, xMax, yMin, yMax });
    maxIter = Math.floor(maxIter * 1.1);
    render();
  }
</script>
<div class="h-full w-fit mx-auto flex flex-col py-8 font-sans text-gray-900 select-none">

  <div class="bg-gray-100 p-4 border-2 border-gray-800 mb-6 flex flex-wrap gap-9 items-start justify-start max-w-5xl w-full">

    <div class="flex gap-1 h-full">
      <button
        on:click={zoomIn}
        disabled={rendering}
      >
        <ZoomIn size={18} />
      </button>
      <button
        on:click={zoomOut}
        disabled={rendering}
      >
        <ZoomOut size={18} />
      </button>
      <button
        on:click={reset}
        disabled={rendering}
      >
        <RotateCcw size={18} />
      </button>
    </div>

    <div class="flex items-center gap-2 border-2 border-gray-800 bg-gray-50 px-2 py-1">
      <select
        bind:value={fractalType}
        on:change={handleChange}
        class="bg-transparent text-gray-800 text-sm font-mono border-none outline-none cursor-pointer appearance-none"
      >
        <option value={0}>Mandelbrot</option>
        <option value={1}>Julia</option>
        <option value={2}>Burning Ship</option>
      </select>
    </div>

    <div class="flex gap-2 h-full items-center border-2 border-gray-800 bg-gray-50 px-3 py-1"
        transition:fade={{ duration: 150 }}>
      <span class="text-[10px] uppercase font-bold text-gray-700">iter</span>
      <input
        type="number"
        step="10"
        min="1"
        bind:value={maxIter}
        on:input={render}
        class="w-16 text-sm font-mono text-center bg-gray-50 border-2 border-gray-800 p-1 text-gray-800 outline-none focus:outline-none"
      />

    </div>

    {#if fractalType === 1}
      <div class="flex gap-2 h-full items-center border-2 border-gray-800 bg-gray-50 px-3 py-1"
          transition:fade={{ duration: 150 }}>
        <span class="text-[10px] uppercase font-bold text-gray-700">Re</span>
        <input
          type="number"
          step="0.01"
          bind:value={juliaParams.re}
          on:input={render}
          class="w-16 text-sm font-mono bg-gray-50 border-2 border-gray-800 p-1 text-gray-800 outline-none focus:outline-none"
        />
        <span class="text-[10px] uppercase font-bold text-gray-700 ml-1">Im</span>
        <input
          type="number"
          step="0.01"
          bind:value={juliaParams.im}
          on:input={render}
          class="w-22 text-sm font-mono bg-gray-50 border-2 border-gray-800 p-1 text-gray-800 outline-none focus:outline-none"
        />
      </div>
    {/if}
  </div>

  <div class="relative border-2 border-gray-800 bg-gray-900 overflow-hidden">
    <canvas bind:this={fractalCanvas} {width} {height} class="block touch-none"></canvas>
    <canvas
      bind:this={overlayCanvas}
      {width}
      {height}
      class="absolute top-0 left-0 cursor-crosshair touch-none"
      on:mousemove={onMouseMove}
      on:wheel={onWheel}
      on:click={onClick}
    ></canvas>
  </div>

  <div class="mt-2 flex flex-row text-sm font-mono text-gray-300 justify-between">
    <a href="https://github.com/rphle/fractals" target="_blank"  class="underline uppercase hover:text-gray-400">Source</a>
    <div class="uppercase ">Click to zoom | Scroll to change zoom level | Arrow keys to move</div>
    <div>
        {#if rendering}
            ...
        {:else}
            {renderTime.toFixed(1)} ms
        {/if}
    </div>
  </div>

</div>
