/* tslint:disable */
/* eslint-disable */

export function analizar_argumento(texto: string): string;

export function aristo_stats(): string;

export function aristo_version(): string;

export function calcular_indice_complejidad(texto: string): number;

export function cita_filosofica(): string;

export function comparar_filosofos(a: string, b: string): string;

export function detectar_escuela_predominante(texto: string): string;

export function detectar_estructura_imryd(texto: string): string;

export function detectar_falacias(texto: string): string;

export function detectar_metodologia(texto: string): string;

export function detectar_temas_filosoficos(texto: string): string;

export function entropia_shannon(texto: string): number;

export function extraer_citas_apa(texto: string): string;

export function extraer_dois(texto: string): string;

export function glosario_filosofico(termino: string): string;

export function indice_guiraud(texto: string): number;

export function indice_herdan(texto: string): number;

export function listar_escuelas(): string;

export function listar_filosofos(): string;

export function pregunta_socratica(tema: string): string;

export function profundidad_filosofica(texto: string): number;

export function resumen_filosofico(texto: string): string;

export function ttr_type_token_ratio(texto: string): number;

export function validar_coherencia(texto: string): number;

export function validar_formato_apa(texto: string): string;

export type InitInput = RequestInfo | URL | Response | BufferSource | WebAssembly.Module;

export interface InitOutput {
    readonly memory: WebAssembly.Memory;
    readonly analizar_argumento: (a: number, b: number) => [number, number];
    readonly aristo_stats: () => [number, number];
    readonly aristo_version: () => [number, number];
    readonly calcular_indice_complejidad: (a: number, b: number) => number;
    readonly cita_filosofica: () => [number, number];
    readonly comparar_filosofos: (a: number, b: number, c: number, d: number) => [number, number];
    readonly detectar_escuela_predominante: (a: number, b: number) => [number, number];
    readonly detectar_estructura_imryd: (a: number, b: number) => [number, number];
    readonly detectar_falacias: (a: number, b: number) => [number, number];
    readonly detectar_metodologia: (a: number, b: number) => [number, number];
    readonly detectar_temas_filosoficos: (a: number, b: number) => [number, number];
    readonly entropia_shannon: (a: number, b: number) => number;
    readonly extraer_citas_apa: (a: number, b: number) => [number, number];
    readonly extraer_dois: (a: number, b: number) => [number, number];
    readonly glosario_filosofico: (a: number, b: number) => [number, number];
    readonly indice_guiraud: (a: number, b: number) => number;
    readonly indice_herdan: (a: number, b: number) => number;
    readonly listar_escuelas: () => [number, number];
    readonly listar_filosofos: () => [number, number];
    readonly pregunta_socratica: (a: number, b: number) => [number, number];
    readonly profundidad_filosofica: (a: number, b: number) => number;
    readonly resumen_filosofico: (a: number, b: number) => [number, number];
    readonly ttr_type_token_ratio: (a: number, b: number) => number;
    readonly validar_coherencia: (a: number, b: number) => number;
    readonly validar_formato_apa: (a: number, b: number) => [number, number];
    readonly __wbindgen_externrefs: WebAssembly.Table;
    readonly __wbindgen_malloc: (a: number, b: number) => number;
    readonly __wbindgen_realloc: (a: number, b: number, c: number, d: number) => number;
    readonly __wbindgen_free: (a: number, b: number, c: number) => void;
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
