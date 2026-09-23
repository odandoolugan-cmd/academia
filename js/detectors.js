import { wasm } from './wasm-loader.js';

export function detectarTodo(texto) {
  return {
    idioma: wasm.detectar_idioma(texto),
    idiomaAvanzado: wasm.detectar_idioma_avanzado(texto),
    sentimiento: wasm.sentimiento_espanol(texto),
    clickbait: wasm.detectar_clickbait(texto),
    discursoOdio: wasm.detectar_discurso_odio(texto),
    muletillasIA: wasm.detectar_muletillas_ia(texto),
    pii: wasm.detectar_pii(texto),
    topicos: wasm.detectar_topicos(texto),
  };
}

export function detectarPII(texto) {
  return wasm.detectar_pii(texto);
}

export function detectarClickbait(texto) {
  return wasm.detectar_clickbait(texto);
}

export function detectarMuletillasIA(texto) {
  return wasm.detectar_muletillas_ia(texto);
}
