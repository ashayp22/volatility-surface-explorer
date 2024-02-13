/* tslint:disable */
/* eslint-disable */
/**
* @param {OptionDir} option_dir
* @param {Float32Array} price
* @param {Float32Array} spot
* @param {Float32Array} strike
* @param {Float32Array} risk_free_rate
* @param {Float32Array} dividend_yield
* @param {Float32Array} years_to_expiry
* @param {number} max_iterations
* @param {number} threshold
* @returns {Float32Array}
*/
export function implied_vol(option_dir: OptionDir, price: Float32Array, spot: Float32Array, strike: Float32Array, risk_free_rate: Float32Array, dividend_yield: Float32Array, years_to_expiry: Float32Array, max_iterations: number, threshold: number): Float32Array;
/**
* @param {Float32Array} call_price
* @param {Float32Array} put_price
* @param {Float32Array} spot
* @param {Float32Array} strike
* @param {Float32Array} years_to_expiry
* @returns {number}
*/
export function parity_interest_rate(call_price: Float32Array, put_price: Float32Array, spot: Float32Array, strike: Float32Array, years_to_expiry: Float32Array): number;
/**
* Specify whether an option is put or call
*/
export enum OptionDir {
  CALL = 2,
  PUT = 1,
}

export type InitInput = RequestInfo | URL | Response | BufferSource | WebAssembly.Module;

export interface InitOutput {
  readonly memory: WebAssembly.Memory;
  readonly implied_vol: (a: number, b: number, c: number, d: number, e: number, f: number, g: number, h: number, i: number, j: number, k: number, l: number, m: number, n: number, o: number, p: number) => void;
  readonly parity_interest_rate: (a: number, b: number, c: number, d: number, e: number, f: number, g: number, h: number, i: number, j: number) => number;
  readonly __wbindgen_add_to_stack_pointer: (a: number) => number;
  readonly __wbindgen_malloc: (a: number, b: number) => number;
  readonly __wbindgen_free: (a: number, b: number, c: number) => void;
}

export type SyncInitInput = BufferSource | WebAssembly.Module;
/**
* Instantiates the given `module`, which can either be bytes or
* a precompiled `WebAssembly.Module`.
*
* @param {SyncInitInput} module
*
* @returns {InitOutput}
*/
export function initSync(module: SyncInitInput): InitOutput;

/**
* If `module_or_path` is {RequestInfo} or {URL}, makes a request and
* for everything else, calls `WebAssembly.instantiate` directly.
*
* @param {InitInput | Promise<InitInput>} module_or_path
*
* @returns {Promise<InitOutput>}
*/
export default function __wbg_init (module_or_path?: InitInput | Promise<InitInput>): Promise<InitOutput>;
