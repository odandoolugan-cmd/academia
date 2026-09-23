import { loadWasm } from './wasm-loader.js';
import { analizarTexto, resumir } from './text-analysis.js';
import { detectarTodo } from './detectors.js';
import { extraerTodo, keywords } from './extractors.js';

export async function initUI() {
  await loadWasm();

  const input = document.getElementById('inputText');
  const btn = document.getElementById('analizarBtn');
  const output = document.getElementById('output');

  if (!input || !btn) {
    console.warn('⚠️ Faltan elementos del DOM');
    return;
  }

  btn.addEventListener('click', () => {
    const texto = input.value.trim();
    if (!texto) return;

    const t0 = performance.now();

    const metricas = analizarTexto(texto);
    const detectado = detectarTodo(texto);
    const extraido = extraerTodo(texto);
    const resumen = resumir(texto, 2);
    const kws = keywords(texto, 5);

    const t1 = performance.now();

    output.innerHTML = `
      <h3>📊 Métricas</h3>
      <pre>${JSON.stringify(metricas, null, 2)}</pre>
      <h3>🔍 Detectado</h3>
      <pre>${JSON.stringify(detectado, null, 2)}</pre>
      <h3>📤 Extraído</h3>
      <pre>${JSON.stringify(extraido, null, 2)}</pre>
      <h3>📝 Resumen</h3>
      <p>${resumen}</p>
      <h3>🔑 Keywords</h3>
      <pre>${kws}</pre>
      <p><small>⏱️ Procesado en ${(t1 - t0).toFixed(2)} ms</small></p>
    `;
  });
}
