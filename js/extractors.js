import { wasm } from './wasm-loader.js';

export function extraerTodo(texto) {
  return {
    emails: wasm.extraer_emails(texto),
    urls: wasm.extraer_urls(texto),
    fechas: wasm.extraer_fechas(texto),
    numeros: wasm.extraer_numeros(texto),
    hashtags: wasm.extraer_hashtags_menciones(texto),
    entidades: wasm.extraer_entidades_nombradas(texto),
    autores: wasm.extract_authors_fast(texto),
  };
}

export function keywords(texto, cantidad = 10) {
  return wasm.extract_keywords(texto, cantidad);
}

export function ngramas(texto, top = 10) {
  return wasm.extract_ngramas(texto, top);
}
