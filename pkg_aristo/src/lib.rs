use wasm_bindgen::prelude::*;
use serde::Serialize;
use js_sys;

#[derive(Serialize)]
struct Filosofo {
    nombre: String,
    epoca: String,
    escuela: String,
    idea_central: String,
}

#[derive(Serialize)]
struct TemaFilosofico {
    tema: String,
    puntuacion: f64,
}

#[wasm_bindgen]
pub fn aristo_version() -> String {
    "0.1.0".to_string()
}

#[wasm_bindgen]
pub fn listar_filosofos() -> String {
    let filosofos = vec![
        ("Sócrates", "Antigua", "Socrática", "Conócete a ti mismo"),
        ("Platón", "Antigua", "Platónica", "Mundo de las ideas"),
        ("Aristóteles", "Antigua", "Peripatética", "El justo medio"),
        ("Epicteto", "Helenística", "Estoica", "Lo que depende de ti"),
        ("Marco Aurelio", "Helenística", "Estoica", "Vivir conforme a la naturaleza"),
        ("Séneca", "Helenística", "Estoica", "La brevedad de la vida"),
        ("Descartes", "Moderna", "Racionalista", "Pienso, luego existo"),
        ("Spinoza", "Moderna", "Racionalista", "Dios o la naturaleza"),
        ("Leibniz", "Moderna", "Racionalista", "Mónadas"),
        ("Locke", "Moderna", "Empirista", "Tabula rasa"),
        ("Hume", "Moderna", "Empirista", "Causalidad como hábito"),
        ("Berkeley", "Moderna", "Empirista", "Ser es ser percibido"),
        ("Kant", "Moderna", "Idealismo trascendental", "Imperativo categórico"),
        ("Hegel", "Moderna", "Idealismo alemán", "Dialéctica del espíritu"),
        ("Schopenhauer", "Moderna", "Pesimismo", "La voluntad como esencia"),
        ("Nietzsche", "Contemporánea", "Vitalismo", "Voluntad de poder"),
        ("Kierkegaard", "Contemporánea", "Existencialismo", "El salto de fe"),
        ("Sartre", "Contemporánea", "Existencialismo", "La existencia precede a la esencia"),
        ("Camus", "Contemporánea", "Absurdismo", "El mito de Sísifo"),
        ("Heidegger", "Contemporánea", "Fenomenología", "El ser-en-el-mundo"),
        ("Wittgenstein", "Contemporánea", "Analítica", "Los juegos del lenguaje"),
        ("Russell", "Contemporánea", "Analítica", "Descripciones definidas"),
        ("Hannah Arendt", "Contemporánea", "Política", "La banalidad del mal"),
        ("Simone de Beauvoir", "Contemporánea", "Existencialismo", "No se nace mujer"),
        ("Foucault", "Contemporánea", "Postestructuralismo", "El poder disciplinario"),
        ("Derrida", "Contemporánea", "Deconstrucción", "La différance"),
        ("Habermas", "Contemporánea", "Teoría crítica", "Acción comunicativa"),
        ("Rawls", "Contemporánea", "Liberalismo", "El velo de la ignorancia"),
        ("Nozick", "Contemporánea", "Libertarismo", "El Estado mínimo"),
        ("Singer", "Contemporánea", "Utilitarismo", "Liberación animal"),
    ];

    let lista: Vec<Filosofo> = filosofos
        .into_iter()
        .map(|(n, e, esc, idea)| Filosofo {
            nombre: n.to_string(),
            epoca: e.to_string(),
            escuela: esc.to_string(),
            idea_central: idea.to_string(),
        })
        .collect();

    serde_json::to_string(&lista).unwrap_or_else(|_| "[]".to_string())
}

#[wasm_bindgen]
pub fn detectar_temas_filosoficos(texto: &str) -> String {
    let t = texto.to_lowercase();
    let temas: Vec<(&str, Vec<&str>)> = vec![
        ("Ética", vec!["ética", "moral", "virtud", "bien", "mal", "deber", "justo"]),
        ("Epistemología", vec!["conocimiento", "verdad", "creencia", "saber", "razón"]),
        ("Metafísica", vec!["ser", "realidad", "existencia", "esencia", "sustancia"]),
        ("Política", vec!["política", "estado", "poder", "justicia", "libertad", "sociedad"]),
        ("Estética", vec!["belleza", "arte", "estética", "sublime"]),
        ("Lógica", vec!["lógica", "silogismo", "argumento", "premisa", "conclusión"]),
        ("Existencialismo", vec!["existencia", "absurdo", "libertad", "angustia", "autenticidad"]),
        ("Fenomenología", vec!["fenómeno", "conciencia", "intencionalidad", "experiencia"]),
        ("Estoicismo", vec!["estoico", "virtud", "naturaleza", "razón", "apatía"]),
        ("Nihilismo", vec!["nihilismo", "nada", "vacío", "sin sentido"]),
    ];

    let mut resultados: Vec<TemaFilosofico> = Vec::new();
    for (tema, palabras) in temas {
        let coincidencias = palabras.iter().filter(|p| t.contains(*p)).count();
        if coincidencias > 0 {
            resultados.push(TemaFilosofico {
                tema: tema.to_string(),
                puntuacion: coincidencias as f64,
            });
        }
    }
    resultados.sort_by(|a, b| b.puntuacion.partial_cmp(&a.puntuacion).unwrap());
    serde_json::to_string(&resultados).unwrap_or_else(|_| "[]".to_string())
}

#[wasm_bindgen]
pub fn glosario_filosofico(termino: &str) -> String {
    let t = termino.to_lowercase();
    let glosario: Vec<(&str, &str)> = vec![
        ("ética", "Rama de la filosofía que estudia la moral y el comportamiento humano."),
        ("metafísica", "Estudio de la naturaleza de la realidad y del ser."),
        ("epistemología", "Teoría del conocimiento; cómo sabemos lo que sabemos."),
        ("lógica", "Estudio del razonamiento válido y las inferencias correctas."),
        ("estética", "Estudio de la belleza, el arte y la experiencia estética."),
        ("ontología", "Parte de la metafísica que estudia el ser en cuanto ser."),
        ("teleología", "Estudio de los fines o propósitos de las cosas."),
        ("dialéctica", "Método de razonamiento por contradicción y superación."),
        ("empirismo", "Corriente que sostiene que el conocimiento viene de la experiencia."),
        ("racionalismo", "Corriente que sostiene que la razón es fuente del conocimiento."),
        ("existencialismo", "Corriente que pone la existencia individual en el centro."),
        ("estoicismo", "Escuela que busca la virtud y la aceptación del destino."),
        ("nihilismo", "Negación de todo sentido, valor o verdad objetiva."),
        ("utilitarismo", "Ética que busca la mayor felicidad para el mayor número."),
        ("imperativo categórico", "Principio moral kantiano de actuar según normas universales."),
    ];

    for (term, def) in glosario {
        if t.contains(term) {
            return serde_json::json!({
                "termino": term,
                "definicion": def
            }).to_string();
        }
    }
    serde_json::json!({
        "termino": termino,
        "definicion": "Término no encontrado en el glosario básico."
    }).to_string()
}

#[wasm_bindgen]
pub fn profundidad_filosofica(texto: &str) -> f64 {
    let t = texto.to_lowercase();
    let palabras_clave = [
        "ser", "existencia", "verdad", "razón", "ética", "moral", "libertad",
        "conocimiento", "realidad", "conciencia", "virtud", "justicia", "bien",
        "mal", "sentido", "absurdo", "dialéctica", "trascendencia",
    ];
    let coincidencias = palabras_clave.iter().filter(|p| t.contains(*p)).count();
    let total_palabras = texto.split_whitespace().count().max(1);
    ((coincidencias as f64 / 10.0) + (total_palabras as f64 / 500.0)).min(1.0)
}

#[wasm_bindgen]
pub fn detectar_escuela_predominante(texto: &str) -> String {
    let t = texto.to_lowercase();
    let escuelas: Vec<(&str, Vec<&str>)> = vec![
        ("Estoicismo", vec!["estoico", "epicteto", "marco aurelio", "séneca", "virtud"]),
        ("Existencialismo", vec!["sartre", "camus", "kierkegaard", "existencia", "absurdo"]),
        ("Racionalismo", vec!["descartes", "spinoza", "leibniz", "razón", "innato"]),
        ("Empirismo", vec!["locke", "hume", "berkeley", "experiencia", "sensación"]),
        ("Idealismo", vec!["kant", "hegel", "ideal", "trascendental", "espíritu"]),
        ("Analítica", vec!["wittgenstein", "russell", "lenguaje", "lógica", "análisis"]),
        ("Fenomenología", vec!["heidegger", "husserl", "fenómeno", "conciencia"]),
        ("Postestructuralismo", vec!["foucault", "derrida", "deconstrucción", "poder"]),
    ];
    let mut mejor = ("Desconocida", 0);
    for (escuela, palabras) in &escuelas {
        let count = palabras.iter().filter(|p| t.contains(*p)).count();
        if count > mejor.1 {
            mejor = (escuela, count);
        }
    }
    mejor.0.to_string()
}

#[wasm_bindgen]
pub fn cita_filosofica() -> String {
    let citas = vec![
        ("Sócrates", "Solo sé que no sé nada."),
        ("Platón", "El conocimiento es recuerdo."),
        ("Aristóteles", "Somos lo que hacemos repetidamente."),
        ("Epicteto", "No son las cosas las que nos perturban, sino nuestras opiniones sobre ellas."),
        ("Marco Aurelio", "Tienes poder sobre tu mente, no sobre los acontecimientos."),
        ("Descartes", "Pienso, luego existo."),
        ("Kant", "Actúa de tal modo que tu máxima pueda ser ley universal."),
        ("Nietzsche", "Quien tiene un porqué para vivir puede soportar casi cualquier cómo."),
        ("Sartre", "El hombre está condenado a ser libre."),
        ("Camus", "Hay que imaginar a Sísifo feliz."),
        ("Hannah Arendt", "Nadie tiene derecho a obedecer."),
        ("Simone de Beauvoir", "No se nace mujer, se llega a serlo."),
    ];
    let idx = (js_sys::Math::random() * citas.len() as f64) as usize;
    let (autor, cita) = citas[idx.min(citas.len() - 1)];
    serde_json::json!({
        "autor": autor,
        "cita": cita
    }).to_string()
}

#[wasm_bindgen]
pub fn comparar_filosofos(a: &str, b: &str) -> String {
    serde_json::json!({
        "filosofo_a": a,
        "filosofo_b": b,
        "comparacion": format!("Comparación entre {} y {}. Ambos abordan la naturaleza de la existencia desde perspectivas complementarias.", a, b)
    }).to_string()
}

#[wasm_bindgen]
pub fn analizar_argumento(texto: &str) -> String {
    let t = texto.to_lowercase();
    let tiene_premisa = t.contains("porque") || t.contains("ya que") || t.contains("puesto que");
    let tiene_conclusion = t.contains("por lo tanto") || t.contains("así que") || t.contains("en conclusión");
    serde_json::json!({
        "tiene_premisa": tiene_premisa,
        "tiene_conclusion": tiene_conclusion,
        "estructura_valida": tiene_premisa && tiene_conclusion,
        "longitud": texto.len()
    }).to_string()
}

#[wasm_bindgen]
pub fn pregunta_socratica(tema: &str) -> String {
    format!("¿Qué es realmente {}? ¿Cómo sabes que lo que crees sobre {} es verdadero?", tema, tema)
}

#[wasm_bindgen]
pub fn detectar_falacias(texto: &str) -> String {
    let t = texto.to_lowercase();
    let mut falacias: Vec<&str> = Vec::new();
    if t.contains("todos saben") || t.contains("todo el mundo") {
        falacias.push("Ad populum");
    }
    if t.contains("o estás conmigo o contra mí") {
        falacias.push("Falso dilema");
    }
    if t.contains("porque lo dijo") && !t.contains("evidencia") {
        falacias.push("Ad hominem");
    }
    if t.contains("después de esto") && t.contains("por lo tanto") {
        falacias.push("Post hoc");
    }
    serde_json::to_string(&falacias).unwrap_or_else(|_| "[]".to_string())
}

#[wasm_bindgen]
pub fn resumen_filosofico(texto: &str) -> String {
    let temas = detectar_temas_filosoficos(texto);
    let escuela = detectar_escuela_predominante(texto);
    let profundidad = profundidad_filosofica(texto);
    serde_json::json!({
        "temas": serde_json::from_str::<serde_json::Value>(&temas).unwrap_or(serde_json::json!([])),
        "escuela_predominante": escuela,
        "profundidad": profundidad
    }).to_string()
}

#[wasm_bindgen]
pub fn listar_escuelas() -> String {
    let escuelas = vec![
        "Estoicismo", "Existencialismo", "Racionalismo", "Empirismo",
        "Idealismo", "Analítica", "Fenomenología", "Postestructuralismo",
        "Nihilismo", "Utilitarismo", "Epicureísmo", "Cinismo",
    ];
    serde_json::to_string(&escuelas).unwrap_or_else(|_| "[]".to_string())
}

#[wasm_bindgen]
pub fn validar_coherencia(texto: &str) -> f64 {
    let oraciones = texto.matches('.').count().max(1);
    let conectores = ["por lo tanto", "sin embargo", "además", "en cambio", "porque"]
        .iter()
        .filter(|c| texto.to_lowercase().contains(*c))
        .count();
    (conectores as f64 / oraciones as f64).min(1.0)
}

#[wasm_bindgen]
pub fn aristo_stats() -> String {
    serde_json::json!({
        "version": "0.1.0",
        "funciones": 15,
        "filosofos": 30,
        "escuelas": 12,
        "temas": 10
    }).to_string()
}
