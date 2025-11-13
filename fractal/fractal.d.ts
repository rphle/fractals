/* tslint:disable */
/* eslint-disable */
export enum FractalType {
  Mandelbrot = 0,
  Julia = 1,
  BurningShip = 2,
}
export class FractalRenderer {
  private constructor();
  free(): void;
  [Symbol.dispose](): void;
  get_buffer_len(): number;
  get_buffer_ptr(): number;
  set_fractal_type(f_type: FractalType): void;
  set_julia_constant(re: number, im: number): void;
  static new(width: number, height: number): FractalRenderer;
  render(x_min: number, x_max: number, y_min: number, y_max: number, max_iter: number): void;
  resize(width: number, height: number): void;
}

export type InitInput = RequestInfo | URL | Response | BufferSource | WebAssembly.Module;

export interface InitOutput {
  readonly memory: WebAssembly.Memory;
  readonly __wbg_fractalrenderer_free: (a: number, b: number) => void;
  readonly fractalrenderer_get_buffer_len: (a: number) => number;
  readonly fractalrenderer_get_buffer_ptr: (a: number) => number;
  readonly fractalrenderer_new: (a: number, b: number) => number;
  readonly fractalrenderer_render: (a: number, b: number, c: number, d: number, e: number, f: number) => void;
  readonly fractalrenderer_resize: (a: number, b: number, c: number) => void;
  readonly fractalrenderer_set_fractal_type: (a: number, b: number) => void;
  readonly fractalrenderer_set_julia_constant: (a: number, b: number, c: number) => void;
  readonly __wbindgen_externrefs: WebAssembly.Table;
  readonly __wbindgen_start: () => void;
}

export type SyncInitInput = BufferSource | WebAssembly.Module;
/**
* Instantiates the given `module`, which can either be bytes or
* a precompiled `WebAssembly.Module`.
*
* @param {{ module: SyncInitInput }} module - Passing `SyncInitInput` directly is deprecated.
*
* @returns {InitOutput}
*/
export function initSync(module: { module: SyncInitInput } | SyncInitInput): InitOutput;

/**
* If `module_or_path` is {RequestInfo} or {URL}, makes a request and
* for everything else, calls `WebAssembly.instantiate` directly.
*
* @param {{ module_or_path: InitInput | Promise<InitInput> }} module_or_path - Passing `InitInput` directly is deprecated.
*
* @returns {Promise<InitOutput>}
*/
export default function __wbg_init (module_or_path?: { module_or_path: InitInput | Promise<InitInput> } | InitInput | Promise<InitInput>): Promise<InitOutput>;
