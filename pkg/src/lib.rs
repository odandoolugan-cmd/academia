use wasm_bindgen::prelude::*;
use serde::{Serialize, Deserialize};
use std::collections::HashMap;

#[derive(Serialize, Deserialize)]
pub struct Keyword {
    pub palabra: String,
    pub frecuencia: usize,
}

#[derive(Serialize, Deserialize)]
pub struct ContextoAgente {
    pub autores: Vec<String>,
    pub keywords: Vec<Keyword>,
    pub resumen: String,
    pub idioma: String,
    pub total_palabras: usize,
    pub total_citas: usize,
    pub wasm_rust: bool,
    pub tiempo_us: u128,
}

#[wasm_bindgen]
pub fn contar_palabras(texto: &str) -> usize {
    texto.split_whitespace()
        .filter(|p| p.chars().any(|c| c.is_alphabetic()))
        .count()
}

#[wasm_bindgen]
pub fn contar_caracteres_sin_espacios(texto: &str) -> usize {
    texto.chars().filter(|c| !c.is_whitespace()).count()
}

#[wasm_bindgen]
pub fn contar_oraciones(texto: &str) -> usize {
    texto.chars().filter(|c| matches!(c, '.' | '!' | '?')).count()
}

#[wasm_bindgen]
pub fn process_text_full(texto: &str) -> String {
    let inicio = now_us();
    let palabras: Vec<&str> = texto.split_whitespace().collect();
    let total = palabras.len();
    
    let mut unicas = std::collections::HashSet::new();
    for p in &palabras {
        unicas.insert(p.to_lowercase().chars()
            .filter(|c| c.is_alphabetic() || *c == '-' || *c == '\'')
            .collect::<String>());
    }
    
    let largas = palabras.iter().filter(|p| p.chars().count() > 7).count();
    let cortas = palabras.iter().filter(|p| p.chars().count() <= 3).count();
    let oraciones = texto.chars().filter(|c| matches!(c, '.' | '!' | '?')).count();
    let parrafos = texto.split("\n\n").filter(|p| !p.trim().is_empty()).count().max(1);
    let caracteres = texto.chars().filter(|c| !c.is_whitespace()).count();
    let citas = contar_citas_apa_interno(texto);
    let riqueza = if total > 0 { (unicas.len() as f64 / total as f64) * 100.0 } else { 0.0 };
    let tiempo_lectura = total as f64 / 200.0;
    
    let tiempo = now_us() - inicio;
    
    format!(
        "{{\"palabras\":{},\"unicas\":{},\"caracteres\":{},\"oraciones\":{},\"parrafos\":{},\"palabras_largas\":{},\"palabras_cortas\":{},\"citas_apa\":{},\"riqueza_vocabulario\":{},\"tiempo_lectura_min\":{},\"tiempo_us\":{}}}",
        total, unicas.len(), caracteres, oraciones, parrafos, largas, cortas, citas,
        (riqueza * 10.0).round() / 10.0, (tiempo_lectura * 10.0).round() / 10.0, tiempo
    )
}

#[wasm_bindgen]
pub fn extract_keywords(texto: &str, cantidad: usize) -> String {
    let stopwords: std::collections::HashSet<&str> = [
        "el","la","los","las","un","una","unos","unas","y","o","pero","que","es","en",
        "por","para","con","como","más","menos","sobre","sin","desde","hasta","de","del",
        "al","lo","le","les","se","me","te","nos","os","su","sus","mi","mis","tu","tus",
        "nuestro","nuestra","vuestro","vuestra","este","esta","estos","estas","ese","esa",
        "esos","esas","aquel","aquella","aquellos","aquellas","the","and","or","but","of",
        "to","for","with","on","at","from","by","in","an","that","this","these","those",
        "der","die","das","und","oder","aber","von","für","mit","auf","bei","aus","nach",
        "dass","dies"
    ].iter().cloned().collect();
    
    let mut frecuencias: HashMap<String, usize> = HashMap::new();
    for palabra in texto.split_whitespace() {
        let limpia: String = palabra.chars()
            .filter(|c| c.is_alphabetic())
            .collect::<String>()
            .to_lowercase();
        if limpia.len() > 3 && !stopwords.contains(limpia.as_str()) {
            *frecuencias.entry(limpia).or_insert(0) += 1;
        }
    }
    
    let mut vec: Vec<(String, usize)> = frecuencias.into_iter().collect();
    vec.sort_by(|a, b| b.1.cmp(&a.1));
    vec.truncate(cantidad);
    
    let keywords: Vec<Keyword> = vec.into_iter()
        .map(|(p, f)| Keyword { palabra: p, frecuencia: f })
        .collect();
    
    serde_json::to_string(&keywords).unwrap_or_else(|_| "[]".to_string())
}

#[wasm_bindgen]
pub fn extract_authors_fast(texto: &str) -> String {
    let mut autores: std::collections::HashSet<String> = std::collections::HashSet::new();
    
    for prefijo in &["Según ", "según ", "Como dice ", "como dice ", "De acuerdo con ", "de acuerdo con ", "Citando a ", "citando a "] {
        let mut resto = texto;
        while let Some(pos) = resto.find(prefijo) {
            let inicio = pos + prefijo.len();
            let despues = &resto[inicio..];
            if let Some(nombre) = extraer_nombre_propio(despues) {
                autores.insert(nombre);
            }
            resto = &resto[inicio..];
            if resto.is_empty() { break; }
        }
    }
    
    let chars: Vec<char> = texto.chars().collect();
    let mut i = 0;
    while i < chars.len() {
        if chars[i] == '(' {
            if let Some(cierre) = encontrar_cierre(&chars, i) {
                let contenido: String = chars[i+1..cierre].iter().collect();
                if let Some(autor) = parsear_cita_apa(&contenido) {
                    autores.insert(autor);
                }
                i = cierre;
            }
        }
        i += 1;
    }
    
    let lista: Vec<String> = autores.into_iter().take(15).collect();
    serde_json::to_string(&lista).unwrap_or_else(|_| "[]".to_string())
}

fn extraer_nombre_propio(texto: &str) -> Option<String> {
    let chars: Vec<char> = texto.chars().collect();
    if chars.is_empty() || !chars[0].is_uppercase() { return None; }
    let mut nombre = String::new();
    let mut palabras = 0;
    let mut i = 0;
    while i < chars.len() && palabras < 3 {
        let c = chars[i];
        if c.is_alphabetic() {
            nombre.push(c);
            i += 1;
        } else if c == ' ' && i + 1 < chars.len() && chars[i+1].is_uppercase() {
            nombre.push(' ');
            palabras += 1;
            i += 1;
        } else {
            break;
        }
    }
    if nombre.len() > 2 { Some(nombre.trim().to_string()) } else { None }
}

fn encontrar_cierre(chars: &[char], inicio: usize) -> Option<usize> {
    for i in (inicio + 1)..chars.len().min(inicio + 100) {
        if chars[i] == ')' { return Some(i); }
    }
    None
}

fn parsear_cita_apa(contenido: &str) -> Option<String> {
    if !contenido.contains(',') { return None; }
    let partes: Vec<&str> = contenido.split(',').collect();
    if partes.len() < 2 { return None; }
    let year = partes[1].trim();
    if year.len() < 4 || !year.chars().next()?.is_ascii_digit() { return None; }
    let autor = partes[0].trim();
    if autor.chars().next()?.is_uppercase() && autor.len() > 2 {
        Some(autor.to_string())
    } else {
        None
    }
}

fn contar_citas_apa_interno(texto: &str) -> usize {
    let chars: Vec<char> = texto.chars().collect();
    let mut count = 0;
    let mut i = 0;
    while i < chars.len() {
        if chars[i] == '(' {
            if let Some(cierre) = encontrar_cierre(&chars, i) {
                let contenido: String = chars[i+1..cierre].iter().collect();
                if parsear_cita_apa(&contenido).is_some() { count += 1; }
                i = cierre;
            }
        }
        i += 1;
    }
    count
}

#[wasm_bindgen]
pub fn detectar_idioma(texto: &str) -> String {
    let lower = texto.to_lowercase();
    let palabras: Vec<&str> = lower.split_whitespace().take(100).collect();
    
    let idiomas: [(&str, &[&str]); 6] = [
        ("es", &["el","la","los","las","un","una","y","que","es","en","por","para","con","como","más"]),
        ("en", &["the","and","or","but","of","to","for","with","on","at","from","by","in","that","this"]),
        ("de", &["der","die","das","und","oder","aber","von","für","mit","auf","bei","aus","dass","dies"]),
        ("fr", &["le","la","les","un","une","des","et","ou","mais","de","pour","avec","sur","dans"]),
        ("pt", &["o","a","os","as","um","uma","e","ou","mas","de","para","com","em","por"]),
        ("it", &["il","la","lo","i","gli","le","un","una","e","o","ma","di","per","con"]),
    ];
    
    let mut mejor = ("es", 0.0_f64);
    for (idioma, patrones) in &idiomas {
        let aciertos = palabras.iter().filter(|p| patrones.contains(p)).count();
        let score = aciertos as f64 / patrones.len() as f64;
        if score > mejor.1 { mejor = (idioma, score); }
    }
    mejor.0.to_string()
}

#[wasm_bindgen]
pub fn resumir_texto(texto: &str, num_oraciones: usize) -> String {
    let oraciones: Vec<&str> = texto
        .split(|c| c == '.' || c == '!' || c == '?')
        .map(|s| s.trim())
        .filter(|s| s.len() > 20)
        .collect();
    
    if oraciones.len() <= num_oraciones { return texto.to_string(); }
    
    let mut frecuencias: HashMap<String, usize> = HashMap::new();
    for palabra in texto.split_whitespace() {
        let limpia: String = palabra.chars().filter(|c| c.is_alphabetic()).collect::<String>().to_lowercase();
        if limpia.len() > 3 { *frecuencias.entry(limpia).or_insert(0) += 1; }
    }
    
    let mut scores: Vec<(usize, f64)> = oraciones.iter().enumerate()
        .map(|(i, oracion)| {
            let palabras: Vec<String> = oracion.split_whitespace()
                .map(|p| p.chars().filter(|c| c.is_alphabetic()).collect::<String>().to_lowercase())
                .filter(|p| p.len() > 3)
                .collect();
            let score: f64 = palabras.iter().map(|p| *frecuencias.get(p).unwrap_or(&0) as f64).sum::<f64>() / palabras.len().max(1) as f64;
            (i, score)
        })
        .collect();
    
    scores.sort_by(|a, b| b.1.partial_cmp(&a.1).unwrap_or(std::cmp::Ordering::Equal));
    let mut indices: Vec<usize> = scores.iter().take(num_oraciones).map(|(i, _)| *i).collect();
    indices.sort();
    indices.iter().map(|&i| oraciones[i]).collect::<Vec<&str>>().join(". ")
}

#[wasm_bindgen]
pub fn process_agent_context(query: &str, results: &str) -> String {
    let inicio = now_us();
    let texto_completo = format!("{} {}", query, results);
    
    let autores_json = extract_authors_fast(&texto_completo);
    let autores: Vec<String> = serde_json::from_str(&autores_json).unwrap_or_default();
    
    let keywords_json = extract_keywords(&texto_completo, 8);
    let keywords: Vec<Keyword> = serde_json::from_str(&keywords_json).unwrap_or_default();
    
    let resumen = resumir_texto(results, 3);
    let idioma = detectar_idioma(&texto_completo);
    let total_palabras = contar_palabras(results);
    let total_citas = contar_citas_apa_interno(results);
    let tiempo = now_us() - inicio;
    
    let contexto = ContextoAgente {
        autores, keywords, resumen, idioma, total_palabras, total_citas,
        wasm_rust: true, tiempo_us: tiempo,
    };
    
    serde_json::to_string(&contexto).unwrap_or_else(|_| "{}".to_string())
}

use std::cell::RefCell;
thread_local! {
    static CACHE: RefCell<HashMap<String, (String, f64)>> = RefCell::new(HashMap::new());
}

#[wasm_bindgen]
pub fn cache_response(key: &str, value: &str) {
    let ahora = js_sys::Date::now();
    CACHE.with(|c| { c.borrow_mut().insert(key.to_string(), (value.to_string(), ahora)); });
}

#[wasm_bindgen]
pub fn get_cached_response(key: &str) -> String {
    let ahora = js_sys::Date::now();
    CACHE.with(|c| {
        if let Some((valor, timestamp)) = c.borrow().get(key) {
            if ahora - timestamp < 3_600_000.0 { return valor.clone(); }
        }
        String::new()
    })
}

#[wasm_bindgen]
pub fn get_cache_size() -> usize {
    CACHE.with(|c| c.borrow().len())
}

#[wasm_bindgen]
pub fn clear_cache() {
    CACHE.with(|c| c.borrow_mut().clear());
}

#[wasm_bindgen]
pub fn wasm_version() -> i32 { 403 }

#[wasm_bindgen]
pub fn now_us() -> u128 {
    js_sys::Date::now() as u128 * 1000
}




// ═══════════════════════════════════════════════════════════
// 🆕 FUNCIONES v2.0 — 10 funciones adicionales
// Total después de este bloque: 25 funciones pub fn
// ═══════════════════════════════════════════════════════════

use serde_json::json;

// ─────────────────────────────────────────────────────────────
// 1. DETECTAR MULETILLAS DE IA (Humanize Deep)
// Devuelve: [{"frase":"en conclusión","count":3}, ...]
// ─────────────────────────────────────────────────────────────
#[wasm_bindgen]
pub fn detectar_muletillas_ia(texto: &str) -> String {
    const MULETILLAS: &[&str] = &[
        "en conclusión", "cabe destacar", "es importante mencionar",
        "es importante destacar", "por otro lado", "en primer lugar",
        "en segundo lugar", "en tercer lugar", "cabe resaltar",
        "es fundamental", "es crucial", "en resumen", "en definitiva",
        "sin duda alguna", "en el mundo actual", "en la era digital",
        "no hay duda de que", "es evidente que", "en este sentido",
        "en relación con esto", "es menester", "conviene señalar",
        "cabe mencionar", "como se mencionó", "es digno de mención",
        "resulta interesante", "es relevante", "cabe subrayar",
        "en aras de", "a fin de cuentas", "en última instancia",
        "por consiguiente", "en consecuencia", "en conclusión final",
        "en definitiva,", "sin duda,", "por ende",
    ];

    let lower = texto.to_lowercase();
    let mut encontradas: Vec<(String, usize)> = Vec::new();

    for m in MULETILLAS {
        let count = lower.matches(m).count();
        if count > 0 {
            encontradas.push((m.to_string(), count));
        }
    }

    encontradas.sort_by(|a, b| b.1.cmp(&a.1));

    let total: usize = encontradas.iter().map(|(_, c)| c).sum();

    serde_json::to_string(&json!({
        "muletillas": encontradas.iter().map(|(f, c)| json!({
            "frase": f,
            "count": c,
        })).collect::<Vec<_>>(),
        "total": total,
    })).unwrap_or_else(|_| "{}".to_string())
}

// ─────────────────────────────────────────────────────────────
// 2. SIMILITUD COSENO (Plagio, Comparador, Pro Scout)
// Devuelve 0.0 a 1.0
// ─────────────────────────────────────────────────────────────
#[wasm_bindgen]
pub fn similitud_coseno(texto1: &str, texto2: &str) -> f64 {
    let tokenizar = |t: &str| -> Vec<String> {
        t.to_lowercase()
            .split(|c: char| !c.is_alphanumeric() && c != 'ñ' && c != 'á' && c != 'é'
                && c != 'í' && c != 'ó' && c != 'ú' && c != 'ü')
            .filter(|w| w.len() > 2)
            .map(|w| w.to_string())
            .collect()
    };

    let t1 = tokenizar(texto1);
    let t2 = tokenizar(texto2);
    if t1.is_empty() || t2.is_empty() { return 0.0; }

    let mut f1: HashMap<&String, u32> = HashMap::new();
    let mut f2: HashMap<&String, u32> = HashMap::new();
    for w in &t1 { *f1.entry(w).or_insert(0) += 1; }
    for w in &t2 { *f2.entry(w).or_insert(0) += 1; }

    let mut producto = 0.0;
    for (w, c1) in &f1 {
        if let Some(c2) = f2.get(w) {
            producto += (*c1 as f64) * (*c2 as f64);
        }
    }

    let mag1: f64 = f1.values().map(|c| (*c as f64).powi(2)).sum::<f64>().sqrt();
    let mag2: f64 = f2.values().map(|c| (*c as f64).powi(2)).sum::<f64>().sqrt();
    if mag1 == 0.0 || mag2 == 0.0 { return 0.0; }
    producto / (mag1 * mag2)
}

// ─────────────────────────────────────────────────────────────
// 3. SENTIMIENTO EN ESPAÑOL
// Devuelve: {"sentimiento":"positivo","score":0.5,"positivas":3,"negativas":1}
// ─────────────────────────────────────────────────────────────
#[wasm_bindgen]
pub fn sentimiento_espanol(texto: &str) -> String {
    const POSITIVAS: &[&str] = &[
        "bueno", "buena", "excelente", "increíble", "maravilloso", "maravillosa",
        "fantástico", "fantástica", "positivo", "positiva", "éxito", "logro",
        "avance", "mejora", "ganancia", "feliz", "alegre", "amor", "bonito",
        "bonita", "hermoso", "hermosa", "genial", "perfecto", "perfecta",
        "óptimo", "óptima", "brillante", "espectacular", "grandioso", "grandiosa",
        "triunfo", "victoria", "beneficio", "progreso", "crecimiento",
        "oportunidad", "esperanza", "ilusión", "satisfacción", "alegría",
        "entusiasmo", "admirable", "destacable", "sobresaliente",
        "extraordinario", "extraordinaria", "valioso", "valiosa", "útil",
    ];
    const NEGATIVAS: &[&str] = &[
        "malo", "mala", "terrible", "horrible", "pésimo", "pésima", "desastre",
        "negativo", "negativa", "fracaso", "pérdida", "retroceso", "problema",
        "triste", "enojado", "enojada", "odio", "feo", "fea", "difícil",
        "complicado", "complicada", "error", "falla", "catástrofe", "crisis",
        "peligro", "amenaza", "miedo", "temor", "ansiedad", "preocupación",
        "conflicto", "guerra", "violencia", "muerte", "enfermedad", "dolor",
        "sufrimiento", "corrupción", "injusticia", "pobreza", "desigualdad",
        "fallo", "derrota", "pérdidas", "deuda", "quiebra",
    ];

    let lower = texto.to_lowercase();
    let palabras: Vec<&str> = lower
        .split(|c: char| !c.is_alphanumeric() && c != 'ñ' && c != 'á' && c != 'é'
            && c != 'í' && c != 'ó' && c != 'ú' && c != 'ü')
        .collect();

    let mut pos = 0usize;
    let mut neg = 0usize;
    for p in &palabras {
        if POSITIVAS.contains(p) { pos += 1; }
        if NEGATIVAS.contains(p) { neg += 1; }
    }

    let total = pos + neg;
    if total == 0 {
        return json!({
            "sentimiento": "neutro",
            "score": 0.0,
            "positivas": 0,
            "negativas": 0,
        }).to_string();
    }
    let score = (pos as f64 - neg as f64) / total as f64;
    let sentimiento = if score > 0.2 { "positivo" }
                      else if score < -0.2 { "negativo" }
                      else { "neutro" };

    json!({
        "sentimiento": sentimiento,
        "score": (score * 1000.0).round() / 1000.0,
        "positivas": pos,
        "negativas": neg,
    }).to_string()
}

// ─────────────────────────────────────────────────────────────
// 4. EXTRAER FECHAS
// Devuelve: ["2024-01-15", "AÑO-2023", ...]
// ─────────────────────────────────────────────────────────────
#[wasm_bindgen]
pub fn extraer_fechas(texto: &str) -> String {
    let mut fechas: Vec<String> = Vec::new();
    let palabras: Vec<&str> = texto.split_whitespace().collect();

    // dd/mm/yyyy o dd-mm-yyyy
    for p in &palabras {
        let limpio: String = p.chars()
            .filter(|c| c.is_numeric() || *c == '/' || *c == '-')
            .collect();
        let partes: Vec<&str> = limpio.split(|c| c == '/' || c == '-').collect();
        if partes.len() == 3 {
            if let (Ok(d), Ok(m), Ok(y)) = (
                partes[0].parse::<u32>(),
                partes[1].parse::<u32>(),
                partes[2].parse::<u32>(),
            ) {
                if (1..=31).contains(&d) && (1..=12).contains(&m) && (1900..=2100).contains(&y) {
                    fechas.push(format!("{:04}-{:02}-{:02}", y, m, d));
                }
            }
        }
    }

    // Años sueltos de 4 dígitos
    for p in &palabras {
        let limpio: String = p.chars().filter(|c| c.is_numeric()).collect();
        if limpio.len() == 4 {
            if let Ok(y) = limpio.parse::<u32>() {
                if (1900..=2100).contains(&y) {
                    let año_str = format!("AÑO-{}", y);
                    if !fechas.iter().any(|f| f.contains(&y.to_string())) {
                        fechas.push(año_str);
                    }
                }
            }
        }
    }

    fechas.sort();
    fechas.dedup();

    serde_json::to_string(&json!({
        "fechas": fechas,
        "total": fechas.len(),
    })).unwrap_or_else(|_| "{}".to_string())
}

// ─────────────────────────────────────────────────────────────
// 5. EXTRAER URLs
// ─────────────────────────────────────────────────────────────
#[wasm_bindgen]
pub fn extraer_urls(texto: &str) -> String {
    let mut urls: Vec<String> = Vec::new();
    for palabra in texto.split_whitespace() {
        let p = palabra.trim_matches(|c: char| {
            c == ',' || c == ')' || c == ']' || c == '}' || c == '"' || c == '\''
            || c == ';' || c == '.' && !palabra.ends_with(".com")
        });
        if p.starts_with("http://") || p.starts_with("https://") || p.starts_with("www.") {
            if p.len() > 10 && p.len() < 500 {
                urls.push(p.to_string());
            }
        }
    }
    urls.sort();
    urls.dedup();

    serde_json::to_string(&json!({
        "urls": urls,
        "total": urls.len(),
    })).unwrap_or_else(|_| "{}".to_string())
}

// ─────────────────────────────────────────────────────────────
// 6. EXTRAER EMAILS
// ─────────────────────────────────────────────────────────────
#[wasm_bindgen]
pub fn extraer_emails(texto: &str) -> String {
    let mut emails: Vec<String> = Vec::new();
    for palabra in texto.split_whitespace() {
        let p = palabra.trim_matches(|c: char| {
            !c.is_alphanumeric() && c != '@' && c != '.' && c != '_' && c != '-' && c != '+'
        });
        if p.contains('@') && p.contains('.') {
            let partes: Vec<&str> = p.split('@').collect();
            if partes.len() == 2 && !partes[0].is_empty() && partes[1].contains('.') {
                let dominio: Vec<&str> = partes[1].split('.').collect();
                if let Some(tld) = dominio.last() {
                    if tld.len() >= 2 && tld.len() <= 6 {
                        emails.push(p.to_lowercase());
                    }
                }
            }
        }
    }
    emails.sort();
    emails.dedup();

    serde_json::to_string(&json!({
        "emails": emails,
        "total": emails.len(),
    })).unwrap_or_else(|_| "{}".to_string())
}

// ─────────────────────────────────────────────────────────────
// 7. NORMALIZAR TEXTO (todas las apps)
// ─────────────────────────────────────────────────────────────
#[wasm_bindgen]
pub fn normalizar_texto(texto: &str) -> String {
    let mut resultado = String::new();
    let mut espacio_previo = false;

    for c in texto.chars() {
        if c.is_whitespace() {
            if !espacio_previo && !resultado.is_empty() {
                resultado.push(' ');
                espacio_previo = true;
            }
        } else {
            resultado.push(c);
            espacio_previo = false;
        }
    }

    // Corregir espacios antes de puntuación
    let mut final_texto = String::new();
    let chars: Vec<char> = resultado.chars().collect();
    for (i, c) in chars.iter().enumerate() {
        if matches!(c, ',' | '.' | ';' | ':' | '!' | '?') {
            while final_texto.ends_with(' ') { final_texto.pop(); }
        }
        final_texto.push(*c);
        if matches!(c, ',' | '.' | ';' | ':' | '!' | '?') {
            if i + 1 < chars.len() && !chars[i + 1].is_whitespace() {
                final_texto.push(' ');
            }
        }
    }

    final_texto.trim().to_string()
}

// ─────────────────────────────────────────────────────────────
// 8. HASH SIMHASH (OSINT, duplicados)
// Devuelve: "a3f5c8d9e1b2f4a6" (16 chars hex)
// ─────────────────────────────────────────────────────────────
#[wasm_bindgen]
pub fn calcular_hash_simhash(texto: &str) -> String {
    let mut hash: u64 = 0xcbf29ce484222325;
    for byte in texto.as_bytes() {
        hash ^= *byte as u64;
        hash = hash.wrapping_mul(0x100000001b3);
    }
    let lower = texto.to_lowercase();
    let palabras: Vec<&str> = lower.split_whitespace().collect();
    hash = hash.wrapping_add(palabras.len() as u64);
    for p in &palabras {
        for b in p.as_bytes() {
            hash = hash.wrapping_mul(31).wrapping_add(*b as u64);
        }
    }
    format!("{:016x}", hash)
}

// ─────────────────────────────────────────────────────────────
// 9. CONTAR SÍLABAS EN ESPAÑOL
// ─────────────────────────────────────────────────────────────
#[wasm_bindgen]
pub fn contar_silabas_espanol(texto: &str) -> usize {
    let vocales = ['a', 'e', 'i', 'o', 'u', 'á', 'é', 'í', 'ó', 'ú', 'ü'];
    let lower = texto.to_lowercase();
    let mut silabas = 0usize;
    let mut vocal_previa = false;

    for c in lower.chars() {
        if !c.is_alphabetic() {
            vocal_previa = false;
            continue;
        }
        let es_vocal = vocales.contains(&c);
        if es_vocal && !vocal_previa {
            silabas += 1;
        }
        vocal_previa = es_vocal;
    }
    silabas.max(1)
}

// ─────────────────────────────────────────────────────────────
// 10. RESUMIR CON TF-IDF SIMPLE
// Devuelve: {"resumen":"frase 1. frase 2.","frases":2}
// ─────────────────────────────────────────────────────────────
#[wasm_bindgen]
pub fn resumir_tfidf(texto: &str, num_frases: usize) -> String {
    let frases: Vec<String> = texto
        .split(|c| c == '.' || c == '!' || c == '?')
        .map(|f| f.trim().to_string())
        .filter(|f| f.len() > 20)
        .collect();

    if frases.is_empty() || frases.len() <= num_frases {
        let r = frases.join(". ");
        return serde_json::to_string(&json!({
            "resumen": r,
            "frases": frases.len(),
        })).unwrap_or_else(|_| "{}".to_string());
    }

    // Frecuencia global de palabras
    let mut freq_global: HashMap<String, u32> = HashMap::new();
    let palabras_todas: Vec<String> = texto
        .to_lowercase()
        .split(|c: char| !c.is_alphanumeric())
        .filter(|w| w.len() > 3)
        .map(|w| w.to_string())
        .collect();
    for p in &palabras_todas {
        *freq_global.entry(p.clone()).or_insert(0) += 1;
    }

    // Puntuar frases
    let mut puntuadas: Vec<(usize, f64)> = frases
        .iter()
        .enumerate()
        .map(|(i, f)| {
            let palabras: Vec<String> = f
                .to_lowercase()
                .split(|c: char| !c.is_alphanumeric())
                .filter(|w| w.len() > 3)
                .map(|w| w.to_string())
                .collect();
            if palabras.is_empty() { return (i, 0.0); }
            let score: f64 = palabras.iter()
                .map(|p| *freq_global.get(p).unwrap_or(&0) as f64)
                .sum::<f64>() / palabras.len() as f64;
            (i, score)
        })
        .collect();

    puntuadas.sort_by(|a, b| b.1.partial_cmp(&a.1).unwrap_or(std::cmp::Ordering::Equal));
    let mut indices: Vec<usize> = puntuadas.iter().take(num_frases).map(|(i, _)| *i).collect();
    indices.sort();

    let resumen = indices.iter()
        .map(|&i| frases[i].clone())
        .collect::<Vec<_>>()
        .join(". ");

    serde_json::to_string(&json!({
        "resumen": resumen,
        "frases": indices.len(),
    })).unwrap_or_else(|_| "{}".to_string())
}




// ═══════════════════════════════════════════════════════════
// 🆕 FUNCIONES v3.0 — 15 funciones adicionales
// Total después de este bloque: 40 funciones pub fn
// ═══════════════════════════════════════════════════════════


// ─────────────────────────────────────────────────────────────
// 1. DETECTAR IDIOMA AVANZADO (20 idiomas con scores)
// Devuelve: {"idioma":"es","score":0.85,"candidatos":[...]}
// ─────────────────────────────────────────────────────────────
#[wasm_bindgen]
pub fn detectar_idioma_avanzado(texto: &str) -> String {
    let lower = texto.to_lowercase();
    let palabras: Vec<&str> = lower
        .split(|c: char| !c.is_alphabetic())
        .filter(|w| !w.is_empty())
        .collect();

    if palabras.is_empty() {
        return json!({"idioma": "es", "score": 0.0, "candidatos": []}).to_string();
    }

    let idiomas: Vec<(&str, Vec<&str>)> = vec![
        ("es", vec!["el","la","los","las","de","que","y","en","un","una","es","por","con","para","no","se","su","al","lo","como","más","pero","sus","le","ya","o","este","sí","porque","esta","entre","cuando","muy","sin","sobre","también","me","hasta","hay","donde","quien","desde","todo","nos","durante","todos","uno","les","ni","contra","otros","ese","eso","ante","ellos","e","esto","mí","antes","algunos","qué","unos","yo","otro","otras","otra","él","tanto","esa","estos","mucho","quienes","nada","muchos","cual","poco","ella","estar","estas","algunas","algo","nosotros","mi","mis","tú","te","ti","tu","tus","ellas","nosotras","vosotros","vosotras","os","mío","mía","míos","mías","tuyo","tuya","tuyos","tuyas","suyo","suya","suyos","suyas","nuestro","nuestra","nuestros","nuestras","vuestro","vuestra","vuestros","vuestras","esos","esas","estoy","estás","está","estamos","estáis","están","ser","soy","eres","somos","sois","son","fue","fueron","era","eran","tiene","tienen","tengo","tenemos"]),
        ("en", vec!["the","of","and","to","in","is","you","that","it","he","was","for","on","are","as","with","his","they","i","at","be","this","have","from","or","one","had","by","word","but","not","what","all","were","we","when","your","can","said","there","use","an","each","which","she","do","how","their","if","will","up","other","about","out","many","then","them","these","so","some","her","would","make","like","him","into","time","has","look","two","more","write","go","see","number","no","way","could","people","my","than","first","water","been","call","who","its","now","find","long","down","day","did","get","come","made","may","part"]),
        ("fr", vec!["le","la","les","de","des","du","un","une","et","est","en","que","qui","pour","dans","sur","pas","plus","par","au","aux","ce","cette","ces","son","sa","ses","avec","ne","se","au","il","elle","nous","vous","ils","elles","être","avoir","faire","dire","pouvoir","aller","voir","savoir","vouloir","venir","devoir","prendre","trouver","donner","falloir","parler","mettre","passer","regarder","aimer","croire","demander","rester","répondre","entendre","penser","arriver","rendre","partir","revenir","sembler","tenir","connaître","devenir","sentir","attendre","sortir","vivre","chercher","comprendre","porter","montrer"]),
        ("de", vec!["der","die","das","und","in","den","von","zu","mit","sich","des","auf","für","ist","im","dem","nicht","ein","eine","als","auch","es","an","werden","aus","er","hat","dass","sie","nach","wird","bei","einer","um","am","sind","noch","wie","einem","über","einen","so","zum","war","haben","nur","oder","aber","vor","zur","bis","mehr","durch","man","sein","wurde","sei","daß","ihre","ihrer","kann","gegen","vom","können","schon","wenn","habe","seine","ihre","wurden"]),
        ("it", vec!["il","lo","la","i","gli","le","un","uno","una","di","del","della","dei","degli","delle","a","al","allo","alla","ai","agli","alle","da","dal","dallo","dalla","dai","dagli","dalle","in","nel","nello","nella","nei","negli","nelle","con","su","sul","sullo","sulla","sui","sugli","sulle","per","tra","fra","e","è","sono","sei","siamo","siete","era","erano","essere","avere","ho","hai","ha","abbiamo","avete","hanno"]),
        ("pt", vec!["o","a","os","as","um","uma","uns","umas","de","do","da","dos","das","em","no","na","nos","nas","por","para","com","que","não","se","é","são","foi","foram","era","eram","tem","têm","tinha","tinham","ser","estar","ter","haver","fazer","dizer","poder","querer","saber","ver","vir","dar","falar","encontrar","chamar","ficar","deixar","levar","trazer","passar","acontecer","seguir"]),
        ("ru", vec!["и","в","не","на","что","он","с","как","а","то","все","она","так","его","но","да","ты","к","у","же","вы","за","бы","по","только","ее","мне","было","вот","от","меня","еще","нет","о","из","ему","теперь","когда","даже","ну","вдруг","ли","если","уже","или","ни","быть","был","него","до","вас","нибудь","опять","уж","вам","ведь","там","потом","себя","ничего","ей","может","они","тут","где","есть","надо","ней","для","мы","тебя","их","чем","была","сам","чтоб","без","будто","чего","раз","тоже","себе","под","будет","ж","тогда","кто","этот","того","потому","этого","какой","совсем","ним","здесь","этом","один","почти","мой","тем","чтобы","нее","сейчас","были","куда","зачем","всех","никогда","можно","при","наконец","два","об","другой","хоть","после","над","больше","тот","через","эти","нас","про","всего","них","какая","много","разве","три","эту","моя","впрочем","хорошо","свою","этой","перед","иногда","лучше","чуть","том","нельзя","такой","им","более","всегда","конечно","всю","между"]),
        ("ja", vec!["の","に","は","を","た","が","で","て","と","し","れ","さ","ある","いる","も","する","から","な","こと","として","い","や","れる","など","なっ","ない","この","ため","その","あっ","よう","また","もの","という","あり","まで","られ","なる","へ","か","だ","これ","によって","により","おり","より","による","ず","なり","られる","において","ば","なか","ほど","ます","的","と共に","に対する","たり"]),
        ("zh", vec!["的","一","是","在","不","了","有","和","人","这","中","大","为","上","个","国","我","以","要","他","时","来","用","们","生","到","作","地","于","出","就","分","对","成","会","可","主","发","年","动","同","工","也","能","下","过","子","说","产","种","面","而","方","后","多","定","行","学","法","所","民","得","经","十","三","之","进","着","等","部","度","家","电","力","里","如","水","化","高","自","二","理","起","小","物","现","实","加","量","都","两","体","制","机","当","使","点","从","业","本","去","把","性","好","应","开","它","合","还","因","由","其","些","然","前","外","天","政","四","日","那","社","义","事","平","形","相","全","表","间","样","与","关","各","重","新","线","内","数","正","心","反","你","明","看","原","又","么","利","比","或","但"]),
        ("ar", vec!["في","من","على","إلى","عن","مع","هذا","هذه","التي","الذي","كان","كانت","قد","لا","ما","هو","هي","أو","أن","إن","كل","بعض","بين","حتى","عند","بعد","قبل","أمام","خلف","فوق","تحت","داخل","خارج","حول","ضد","نحو","منذ","خلال","حيث","كيف","متى","أين","لماذا","ماذا"]),
        ("nl", vec!["de","van","het","een","in","is","dat","op","te","zijn","met","voor","niet","aan","er","om","ook","als","dan","maar","bij","of","uit","door","over","tot","naar","kan","meer","worden","werd","wordt","die","deze","dit","wij","jullie","zij","ik","jij","hij","het"]),
        ("pl", vec!["w","i","na","z","do","to","się","jest","że","nie","jak","po","tak","ale","czy","już","bardzo","tylko","może","być","mieć","robić","mówić","wiedzieć","chcieć","widzieć","iść","dać","wziąć","powiedzieć","myśleć","przyjść","zostać","wyjść","wrócić","znaleźć","powinien","musieć","móc","by","aby","gdy","kiedy","gdzie","dlaczego"]),
        ("tr", vec!["bir","ve","bu","da","de","ne","ki","o","ben","sen","biz","siz","onlar","var","yok","için","ile","ama","ancak","veya","ya","çok","az","daha","en","gibi","kadar","sonra","önce","şimdi","burada","orada","nasıl","neden","niçin","kim","ne","hangi"]),
        ("ko", vec!["이","그","저","것","수","등","들","및","에서","에게","으로","로","와","과","은","는","가","을","를","의","에","도","만","부터","까지","보다","처럼","같이","대해","통해","위해","때문","경우","동안","후","전","사이"]),
    ];

    let mut scores: Vec<(&str, f64)> = Vec::new();
    for (codigo, marcas) in &idiomas {
        let coincidencias = palabras.iter().filter(|p| marcas.contains(p)).count();
        let score = coincidencias as f64 / palabras.len() as f64;
        scores.push((codigo, score));
    }

    scores.sort_by(|a, b| b.1.partial_cmp(&a.1).unwrap_or(std::cmp::Ordering::Equal));

    let mejor = scores.first().copied().unwrap_or(("es", 0.0));
    let candidatos: Vec<_> = scores.iter().take(5)
        .map(|(c, s)| json!({"idioma": c, "score": (s * 1000.0).round() / 1000.0}))
        .collect();

    json!({
        "idioma": mejor.0,
        "score": (mejor.1 * 1000.0).round() / 1000.0,
        "candidatos": candidatos,
    }).to_string()
}

// ─────────────────────────────────────────────────────────────
// 2. DETECTAR CLICKBAIT
// ─────────────────────────────────────────────────────────────
#[wasm_bindgen]
pub fn detectar_clickbait(texto: &str) -> String {
    let lower = texto.to_lowercase();
    let mut razones: Vec<String> = Vec::new();
    let mut score = 0.0;

    let sensacionalistas = ["increíble","impresionante","escandaloso","impactante","brutal","alucinante","flipante","increíblemente","no vas a creer","no te imaginas","te sorprenderá","te va a encantar","esto cambiará tu vida","nadie esperaba","lo que pasó después","mira lo que","el secreto","truco","trucos","hack","revelado","descubierto","filtrado","polémico"];
    let encontradas_sens: Vec<&str> = sensacionalistas.iter().filter(|s| lower.contains(*s)).copied().collect();
    if !encontradas_sens.is_empty() {
        score += 0.35 * (encontradas_sens.len() as f64).min(3.0) / 3.0;
        razones.push(format!("Palabras sensacionalistas: {}", encontradas_sens.join(", ")));
    }

    let letras: Vec<char> = texto.chars().filter(|c| c.is_alphabetic()).collect();
    let mayusculas = letras.iter().filter(|c| c.is_uppercase()).count();
    if !letras.is_empty() {
        let ratio_mayus = mayusculas as f64 / letras.len() as f64;
        if ratio_mayus > 0.4 {
            score += 0.25;
            razones.push(format!("Exceso de mayúsculas ({:.0}%)", ratio_mayus * 100.0));
        }
    }

    let exclamaciones = texto.matches('!').count() + texto.matches('¡').count();
    let interrogaciones = texto.matches('?').count() + texto.matches('¿').count();
    if exclamaciones >= 3 {
        score += 0.15;
        razones.push(format!("{} signos de exclamación", exclamaciones));
    }
    if interrogaciones >= 2 && exclamaciones >= 2 {
        score += 0.1;
        razones.push("Mezcla de interrogación y exclamación".to_string());
    }

    let emojis = texto.chars().filter(|c| (*c as u32) > 0x1F000).count();
    if emojis >= 2 {
        score += 0.1;
        razones.push(format!("{} emojis", emojis));
    }

    if texto.len() < 120 {
        let digitos = texto.chars().filter(|c| c.is_ascii_digit()).count();
        if digitos >= 2 {
            score += 0.05;
            razones.push("Título con números específicos".to_string());
        }
    }

    score = score.min(1.0);
    json!({
        "es_clickbait": score > 0.4,
        "score": (score * 100.0).round() / 100.0,
        "razones": razones,
    }).to_string()
}

// ─────────────────────────────────────────────────────────────
// 3. DETECTAR DISCURSO DE ODIO (heurística básica)
// ─────────────────────────────────────────────────────────────
#[wasm_bindgen]
pub fn detectar_discurso_odio(texto: &str) -> String {
    let lower = texto.to_lowercase();
    let palabras: Vec<&str> = lower.split_whitespace().collect();

    let terminos_agresivos = ["idiota","imbécil","estúpido","tarado","subnormal","maldito","maldita","basura","asqueroso","asquerosa","repugnante","despreciable","cerdo","cerda","rata","cobarde","hipócrita","mentiroso","traidor","traidora","escoria","inútil","patético","patética","ridículo","ridícula","gilipollas","cabrón","cabrona","puta","puto","joder","coño","mierda","hostia","pendejo","pendeja"];
    let terminos_odio = ["matar","asesinar","exterminar","eliminar","aniquilar","masacrar","colgar","linchar","quemar","ahorcar","decapitar","violar","apuñalar","disparar","golpear","destruir"];

    let mut encontrados_agresivos: Vec<&str> = Vec::new();
    let mut encontrados_odio: Vec<&str> = Vec::new();

    for t in &terminos_agresivos {
        if lower.contains(t) { encontrados_agresivos.push(t); }
    }
    for t in &terminos_odio {
        if lower.contains(t) { encontrados_odio.push(t); }
    }

    let total_agresivos = encontrados_agresivos.len();
    let total_odio = encontrados_odio.len();
    let ratio = (total_agresivos + total_odio * 3) as f64 / palabras.len().max(1) as f64;
    let score = (ratio * 10.0).min(1.0);

    let nivel = if total_odio > 0 { "alto" }
                else if total_agresivos >= 3 { "medio" }
                else if total_agresivos >= 1 { "bajo" }
                else { "ninguno" };

    let mut terminos: Vec<String> = Vec::new();
    terminos.extend(encontrados_agresivos.iter().map(|s| s.to_string()));
    terminos.extend(encontrados_odio.iter().map(|s| s.to_string()));

    json!({
        "nivel": nivel,
        "score": (score * 100.0).round() / 100.0,
        "terminos": terminos,
    }).to_string()
}

// ─────────────────────────────────────────────────────────────
// 4. DETECTAR PII (emails, teléfonos, tarjetas, DNI)
// ─────────────────────────────────────────────────────────────
#[wasm_bindgen]
pub fn detectar_pii(texto: &str) -> String {
    let emails_json = extraer_emails(texto);
    let emails: serde_json::Value = serde_json::from_str(&emails_json).unwrap_or(json!({"emails": []}));

    let mut telefonos: Vec<String> = Vec::new();
    for palabra in texto.split_whitespace() {
        let limpio: String = palabra.chars()
            .filter(|c| c.is_ascii_digit() || *c == '+' || *c == '-' || *c == ' ' || *c == '(' || *c == ')')
            .collect();
        let digitos: String = limpio.chars().filter(|c| c.is_ascii_digit()).collect();
        if digitos.len() >= 9 && digitos.len() <= 15 && (limpio.contains('+') || limpio.contains('-') || limpio.contains('(') || digitos.len() == 9 || digitos.len() == 10) {
            telefonos.push(limpio.trim().to_string());
        }
    }
    telefonos.sort();
    telefonos.dedup();

    let mut tarjetas: Vec<String> = Vec::new();
    for palabra in texto.split_whitespace() {
        let digitos: String = palabra.chars().filter(|c| c.is_ascii_digit()).collect();
        if digitos.len() == 16 {
            tarjetas.push(format!("****-****-****-{}", &digitos[12..]));
        }
    }

    let mut dni: Vec<String> = Vec::new();
    for palabra in texto.split_whitespace() {
        let p = palabra.trim_matches(|c: char| !c.is_alphanumeric());
        if p.len() == 9 {
            let (num, letra) = p.split_at(8);
            if num.chars().all(|c| c.is_ascii_digit()) && letra.chars().all(|c| c.is_alphabetic()) {
                dni.push(format!("******{}", letra));
            }
        }
    }

    json!({
        "emails": emails.get("emails").cloned().unwrap_or(json!([])),
        "telefonos": telefonos,
        "tarjetas": tarjetas,
        "dni": dni,
        "total": emails.get("total").and_then(|v| v.as_u64()).unwrap_or(0) as usize + telefonos.len() + tarjetas.len() + dni.len(),
    }).to_string()
}

// ─────────────────────────────────────────────────────────────
// 5. DETECTAR TÓPICOS / FRASES VACÍAS
// ─────────────────────────────────────────────────────────────
#[wasm_bindgen]
pub fn detectar_topicos(texto: &str) -> String {
    let lower = texto.to_lowercase();
    let topicos = [
        "en el mundo actual", "en la sociedad actual", "en la era digital",
        "desde tiempos inmemoriales", "a lo largo de la historia",
        "es bien sabido que", "todo el mundo sabe que", "es de conocimiento general",
        "cabe destacar que", "es importante mencionar", "es necesario destacar",
        "no es casualidad que", "sin lugar a dudas", "sin duda alguna",
        "en definitiva", "a fin de cuentas", "al final del día",
        "hay que tener en cuenta", "es fundamental comprender",
        "juega un papel crucial", "juega un papel fundamental",
        "marca un antes y un después", "sienta las bases",
        "abre un abanico de posibilidades", "en este sentido",
        "por otro lado", "por su parte", "en cuanto a",
        "en relación con", "a nivel de", "a nivel global",
        "de alguna manera", "de cierto modo", "en cierta medida",
    ];

    let mut encontrados: Vec<(String, usize)> = Vec::new();
    for t in &topicos {
        let count = lower.matches(t).count();
        if count > 0 {
            encontrados.push((t.to_string(), count));
        }
    }

    encontrados.sort_by(|a, b| b.1.cmp(&a.1));
    let total: usize = encontrados.iter().map(|(_, c)| c).sum();

    json!({
        "topicos": encontrados.iter().map(|(t, c)| json!({"frase": t, "count": c})).collect::<Vec<_>>(),
        "total": total,
    }).to_string()
}

// ─────────────────────────────────────────────────────────────
// 6. ÍNDICE FLESCH ADAPTADO AL ESPAÑOL (Fernández-Huerta)
// ─────────────────────────────────────────────────────────────
#[wasm_bindgen]
pub fn indice_flesch_espanol(texto: &str) -> String {
    let palabras = contar_palabras(texto);
    let oraciones = contar_oraciones(texto).max(1);
    let silabas = contar_silabas_espanol(texto);

    if palabras == 0 { return json!({"score": 0, "nivel": "vacío"}).to_string(); }

    let p = (silabas as f64 / palabras as f64) * 100.0;
    let f = palabras as f64 / oraciones as f64;
    let score = 206.84 - 0.60 * p - 1.02 * f;
    let score = score.max(0.0).min(100.0);

    let (nivel, interpretacion) = if score >= 90.0 {
        ("muy fácil", "Comprensible para un niño de 10 años")
    } else if score >= 80.0 {
        ("fácil", "Comprensible para un niño de 12 años")
    } else if score >= 70.0 {
        ("bastante fácil", "Comprensible para un adolescente")
    } else if score >= 60.0 {
        ("normal", "Comprensible para un adulto promedio")
    } else if score >= 50.0 {
        ("algo difícil", "Requiere educación secundaria")
    } else if score >= 30.0 {
        ("difícil", "Requiere educación universitaria")
    } else {
        ("muy difícil", "Nivel académico o profesional avanzado")
    };

    json!({
        "score": (score * 10.0).round() / 10.0,
        "nivel": nivel,
        "interpretacion": interpretacion,
        "silabas": silabas,
        "palabras": palabras,
        "oraciones": oraciones,
    }).to_string()
}

// ─────────────────────────────────────────────────────────────
// 7. ÍNDICE GUNNING FOG (años de escolaridad)
// ─────────────────────────────────────────────────────────────
#[wasm_bindgen]
pub fn indice_gunning_fog(texto: &str) -> String {
    let palabras = contar_palabras(texto);
    let oraciones = contar_oraciones(texto).max(1);

    if palabras == 0 { return json!({"score": 0, "nivel": "vacío"}).to_string(); }

    let palabras_complejas = texto
        .split_whitespace()
        .filter(|p| p.chars().filter(|c| c.is_alphabetic()).count() >= 8)
        .count();

    let asl = palabras as f64 / oraciones as f64;
    let phw = (palabras_complejas as f64 / palabras as f64) * 100.0;
    let score = 0.4 * (asl + phw);
    let años_escolaridad = score.round() as i32;

    let nivel = if años_escolaridad <= 6 { "Primaria" }
                else if años_escolaridad <= 9 { "ESO" }
                else if años_escolaridad <= 12 { "Bachillerato" }
                else if años_escolaridad <= 16 { "Universidad" }
                else { "Posgrado" };

    json!({
        "score": (score * 10.0).round() / 10.0,
        "años_escolaridad": años_escolaridad,
        "nivel": nivel,
        "palabras_complejas": palabras_complejas,
    }).to_string()
}

// ─────────────────────────────────────────────────────────────
// 8. DENSIDAD LÉXICA (riqueza de vocabulario)
// ─────────────────────────────────────────────────────────────
#[wasm_bindgen]
pub fn densidad_lexica(texto: &str) -> f64 {
    let palabras: Vec<String> = texto
        .to_lowercase()
        .split(|c: char| !c.is_alphabetic())
        .filter(|w| w.len() > 2)
        .map(|w| w.to_string())
        .collect();

    if palabras.is_empty() { return 0.0; }

    let unicas: std::collections::HashSet<&String> = palabras.iter().collect();
    unicas.len() as f64 / palabras.len() as f64
}

// ─────────────────────────────────────────────────────────────
// 9. LONGITUD MEDIA DE ORACIONES
// ─────────────────────────────────────────────────────────────
#[wasm_bindgen]
pub fn longitud_media_oraciones(texto: &str) -> f64 {
    let palabras = contar_palabras(texto);
    let oraciones = contar_oraciones(texto).max(1);
    palabras as f64 / oraciones as f64
}

// ─────────────────────────────────────────────────────────────
// 10. EXTRAER ENTIDADES NOMBRADAS (heurística)
// ─────────────────────────────────────────────────────────────
#[wasm_bindgen]
pub fn extraer_entidades_nombradas(texto: &str) -> String {
    let palabras: Vec<&str> = texto.split_whitespace().collect();
    let mut entidades: std::collections::HashSet<String> = std::collections::HashSet::new();

    let mut i = 0;
    while i < palabras.len() {
        let p = palabras[i].trim_matches(|c: char| !c.is_alphanumeric());
        if p.len() >= 2 && p.chars().next().map_or(false, |c| c.is_uppercase()) {
            let mut secuencia = vec![p];
            let mut j = i + 1;
            while j < palabras.len() && secuencia.len() < 4 {
                let next = palabras[j].trim_matches(|c: char| !c.is_alphanumeric());
                if next.len() >= 2 && next.chars().next().map_or(false, |c| c.is_uppercase()) {
                    secuencia.push(next);
                    j += 1;
                } else {
                    break;
                }
            }
            if secuencia.len() >= 2 {
                entidades.insert(secuencia.join(" "));
            }
            i = j;
        } else {
            i += 1;
        }
    }

    let lista: Vec<String> = entidades.into_iter().collect();
    json!({
        "entidades": lista,
        "total": lista.len(),
    }).to_string()
}

// ─────────────────────────────────────────────────────────────
// 11. EXTRAER NÚMEROS (cifras, cantidades, porcentajes)
// ─────────────────────────────────────────────────────────────
#[wasm_bindgen]
pub fn extraer_numeros(texto: &str) -> String {
    let mut numeros: Vec<String> = Vec::new();
    let mut porcentajes: Vec<String> = Vec::new();
    let mut dinero: Vec<String> = Vec::new();

    for palabra in texto.split_whitespace() {
        if palabra.contains('%') {
            let limpio: String = palabra.chars().filter(|c| c.is_ascii_digit() || *c == '%' || *c == '.' || *c == ',').collect();
            if limpio.contains('%') && limpio.chars().any(|c| c.is_ascii_digit()) {
                porcentajes.push(limpio);
            }
            continue;
        }
        if palabra.contains('€') || palabra.contains('$') || palabra.to_uppercase().contains("USD") || palabra.to_uppercase().contains("EUR") {
            let limpio: String = palabra.chars()
                .filter(|c| c.is_ascii_digit() || *c == '.' || *c == ',' || *c == '€' || *c == '$')
                .collect();
            if !limpio.is_empty() { dinero.push(limpio); }
            continue;
        }
        let limpio: String = palabra.chars()
            .filter(|c| c.is_ascii_digit() || *c == '.' || *c == ',')
            .collect();
        if !limpio.is_empty() && limpio.chars().any(|c| c.is_ascii_digit()) {
            let solo_digitos = limpio.replace(&['.', ','][..], "");
            if solo_digitos.len() >= 2 {
                numeros.push(limpio);
            }
        }
    }

    numeros.sort();
    numeros.dedup();
    porcentajes.sort();
    porcentajes.dedup();
    dinero.sort();
    dinero.dedup();

    json!({
        "numeros": numeros,
        "porcentajes": porcentajes,
        "dinero": dinero,
        "total": numeros.len() + porcentajes.len() + dinero.len(),
    }).to_string()
}

// ─────────────────────────────────────────────────────────────
// 12. EXTRAER HASHTAGS Y MENCIONES
// ─────────────────────────────────────────────────────────────
#[wasm_bindgen]
pub fn extraer_hashtags_menciones(texto: &str) -> String {
    let mut hashtags: Vec<String> = Vec::new();
    let mut menciones: Vec<String> = Vec::new();

    for palabra in texto.split_whitespace() {
        let p = palabra.trim_matches(|c: char| !c.is_alphanumeric() && c != '#' && c != '@' && c != '_');
        if p.starts_with('#') && p.len() > 1 {
            hashtags.push(p.to_lowercase());
        }
        if p.starts_with('@') && p.len() > 1 {
            menciones.push(p.to_lowercase());
        }
    }

    hashtags.sort();
    hashtags.dedup();
    menciones.sort();
    menciones.dedup();

    json!({
        "hashtags": hashtags,
        "menciones": menciones,
        "total": hashtags.len() + menciones.len(),
    }).to_string()
}

// ─────────────────────────────────────────────────────────────
// 13. EXTRAER N-GRAMAS (frases más repetidas)
// ─────────────────────────────────────────────────────────────
#[wasm_bindgen]
pub fn extract_ngramas(texto: &str, top: usize) -> String {
    let palabras: Vec<String> = texto
        .to_lowercase()
        .split(|c: char| !c.is_alphabetic())
        .filter(|w| w.len() > 2)
        .map(|w| w.to_string())
        .collect();

    if palabras.len() < 2 {
        return json!({"unigramas": [], "bigramas": [], "trigramas": []}).to_string();
    }

    let mut uni: HashMap<String, usize> = HashMap::new();
    let mut bi: HashMap<String, usize> = HashMap::new();
    let mut tri: HashMap<String, usize> = HashMap::new();

    for p in &palabras {
        *uni.entry(p.clone()).or_insert(0) += 1;
    }
    for w in palabras.windows(2) {
        *bi.entry(format!("{} {}", w[0], w[1])).or_insert(0) += 1;
    }
    for w in palabras.windows(3) {
        *tri.entry(format!("{} {} {}", w[0], w[1], w[2])).or_insert(0) += 1;
    }

    let top_n = |m: &HashMap<String, usize>| -> Vec<(String, usize)> {
        let mut v: Vec<(String, usize)> = m.iter().map(|(k, v)| (k.clone(), *v)).collect();
        v.sort_by(|a, b| b.1.cmp(&a.1));
        v.truncate(top);
        v
    };

    json!({
        "unigramas": top_n(&uni).iter().map(|(k, v)| json!({"texto": k, "count": v})).collect::<Vec<_>>(),
        "bigramas": top_n(&bi).iter().map(|(k, v)| json!({"texto": k, "count": v})).collect::<Vec<_>>(),
        "trigramas": top_n(&tri).iter().map(|(k, v)| json!({"texto": k, "count": v})).collect::<Vec<_>>(),
    }).to_string()
}

// ─────────────────────────────────────────────────────────────
// 14. SANITIZAR TEXTO (quita HTML, scripts, caracteres raros)
// ─────────────────────────────────────────────────────────────
#[wasm_bindgen]
pub fn sanitizar_texto(texto: &str) -> String {
    let mut resultado = String::new();
    let mut dentro_tag = false;
    let mut dentro_script = false;
    let mut buffer = String::new();

    for c in texto.chars() {
        if c == '<' {
            buffer.clear();
            buffer.push(c);
            dentro_tag = true;
            continue;
        }
        if dentro_tag {
            buffer.push(c);
            if c == '>' {
                let tag_lower = buffer.to_lowercase();
                if tag_lower.starts_with("<script") || tag_lower.starts_with("<style") {
                    dentro_script = true;
                }
                if tag_lower.starts_with("</script") || tag_lower.starts_with("</style") {
                    dentro_script = false;
                }
                dentro_tag = false;
                buffer.clear();
            }
            continue;
        }
        if dentro_script {
            continue;
        }
        if c.is_control() && c != '\n' && c != '\t' {
            continue;
        }
        resultado.push(c);
    }

    normalizar_texto(&resultado)
}

// ─────────────────────────────────────────────────────────────
// 15. CALCULAR HASH SHA-256 (puro Rust, sin dependencias)
// ─────────────────────────────────────────────────────────────
#[wasm_bindgen]
pub fn calcular_hash_sha256(texto: &str) -> String {
    const K: [u32; 64] = [
        0x428a2f98, 0x71374491, 0xb5c0fbcf, 0xe9b5dba5, 0x3956c25b, 0x59f111f1, 0x923f82a4, 0xab1c5ed5,
        0xd807aa98, 0x12835b01, 0x243185be, 0x550c7dc3, 0x72be5d74, 0x80deb1fe, 0x9bdc06a7, 0xc19bf174,
        0xe49b69c1, 0xefbe4786, 0x0fc19dc6, 0x240ca1cc, 0x2de92c6f, 0x4a7484aa, 0x5cb0a9dc, 0x76f988da,
        0x983e5152, 0xa831c66d, 0xb00327c8, 0xbf597fc7, 0xc6e00bf3, 0xd5a79147, 0x06ca6351, 0x14292967,
        0x27b70a85, 0x2e1b2138, 0x4d2c6dfc, 0x53380d13, 0x650a7354, 0x766a0abb, 0x81c2c92e, 0x92722c85,
        0xa2bfe8a1, 0xa81a664b, 0xc24b8b70, 0xc76c51a3, 0xd192e819, 0xd6990624, 0xf40e3585, 0x106aa070,
        0x19a4c116, 0x1e376c08, 0x2748774c, 0x34b0bcb5, 0x391c0cb3, 0x4ed8aa4a, 0x5b9cca4f, 0x682e6ff3,
        0x748f82ee, 0x78a5636f, 0x84c87814, 0x8cc70208, 0x90befffa, 0xa4506ceb, 0xbef9a3f7, 0xc67178f2,
    ];

    let mut h: [u32; 8] = [
        0x6a09e667, 0xbb67ae85, 0x3c6ef372, 0xa54ff53a,
        0x510e527f, 0x9b05688c, 0x1f83d9ab, 0x5be0cd19,
    ];

    let mut msg = texto.as_bytes().to_vec();
    let bit_len = (msg.len() as u64) * 8;
    msg.push(0x80);
    while (msg.len() + 8) % 64 != 0 {
        msg.push(0);
    }
    msg.extend_from_slice(&bit_len.to_be_bytes());

    for chunk in msg.chunks(64) {
        let mut w = [0u32; 64];
        for i in 0..16 {
            w[i] = u32::from_be_bytes([chunk[i * 4], chunk[i * 4 + 1], chunk[i * 4 + 2], chunk[i * 4 + 3]]);
        }
        for i in 16..64 {
            let s0 = w[i - 15].rotate_right(7) ^ w[i - 15].rotate_right(18) ^ (w[i - 15] >> 3);
            let s1 = w[i - 2].rotate_right(17) ^ w[i - 2].rotate_right(19) ^ (w[i - 2] >> 10);
            w[i] = w[i - 16].wrapping_add(s0).wrapping_add(w[i - 7]).wrapping_add(s1);
        }

        let (mut a, mut b, mut c, mut d, mut e, mut f, mut g, mut hh) =
            (h[0], h[1], h[2], h[3], h[4], h[5], h[6], h[7]);

        for i in 0..64 {
            let s1 = e.rotate_right(6) ^ e.rotate_right(11) ^ e.rotate_right(25);
            let ch = (e & f) ^ ((!e) & g);
            let temp1 = hh.wrapping_add(s1).wrapping_add(ch).wrapping_add(K[i]).wrapping_add(w[i]);
            let s0 = a.rotate_right(2) ^ a.rotate_right(13) ^ a.rotate_right(22);
            let maj = (a & b) ^ (a & c) ^ (b & c);
            let temp2 = s0.wrapping_add(maj);
            hh = g; g = f; f = e; e = d.wrapping_add(temp1);
            d = c; c = b; b = a; a = temp1.wrapping_add(temp2);
        }

        h[0] = h[0].wrapping_add(a);
        h[1] = h[1].wrapping_add(b);
        h[2] = h[2].wrapping_add(c);
        h[3] = h[3].wrapping_add(d);
        h[4] = h[4].wrapping_add(e);
        h[5] = h[5].wrapping_add(f);
        h[6] = h[6].wrapping_add(g);
        h[7] = h[7].wrapping_add(hh);
    }

    h.iter().map(|x| format!("{:08x}", x)).collect::<String>()
}





