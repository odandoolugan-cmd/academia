import { wasm } from './wasm-loader.js';

export function analizarTexto(texto) {
  return {
    palabras: wasm.contar_palabras(texto),
    oraciones: wasm.contar_oraciones(texto),
    caracteres: wasm.contar_caracteres_sin_espacios(texto),
    silabas: wasm.contar_silabas_espanol(texto),
    densidad: wasm.densidad_lexica(texto),
    flesch: wasm.indice_flesch_espanol(texto),
    gunning: wasm.indice_gunning_fog(texto),
    longitudMedia: wasm.longitud_media_oraciones(texto),
  };
}

export function analisisCompleto(texto) {
  return JSON.parse(wasm.process_text_full(texto));
}

export function resumir(texto, numOraciones = 3) {
  return wasm.resumir_tfidf(texto, numOraciones);
}

export function similitud(texto1, texto2) {
  return wasm.similitud_coseno(texto1, texto2);
}
