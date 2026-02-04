use axum::{
    extract::State,
    routing::{get, post},
    Router,
    Json,
};
use std::net::SocketAddr;
use std::sync::Arc;
use std::collections::HashMap;
use serde_json::{Value, json};
use csln_core::Style;
use csln_processor::{Processor, Reference, Bibliography, Citation, CitationItem};
use serde::{Deserialize, Serialize};
use intent_engine::{StyleIntent, DecisionPackage};

struct AppState {
    references: HashMap<String, Reference>,
}

#[tokio::main]
async fn main() {
    tracing_subscriber::fmt::init();

    // Load references from YAML
    // Check multiple potential locations for the file
    let possible_paths = [
        "server/resources/comprehensive.yaml",
        "resources/comprehensive.yaml",
        "../resources/comprehensive.yaml",
    ];

    let mut ref_path = "server/resources/comprehensive.yaml";
    for path in possible_paths {
        if std::path::Path::new(path).exists() {
            ref_path = path;
            break;
        }
    }
    
    println!("Loading references from: {}", ref_path);

    let references: HashMap<String, Reference> = match std::fs::File::open(ref_path) {
        Ok(f) => match serde_yaml::from_reader::<_, HashMap<String, Reference>>(f) {
            Ok(mut refs) => {
                // Ensure each reference has its ID set from the map key
                for (id, reference) in refs.iter_mut() {
                    reference.set_id(id.clone());
                }
                refs
            },
            Err(e) => {
                println!("Failed to parse comprehensive.yaml: {}", e);
                HashMap::new()
            }
        },
        Err(e) => {
            println!("Failed to open comprehensive.yaml: {}", e);
            HashMap::new()
        }
    };

    let state = Arc::new(AppState {
        references: references.clone()
    });
    
    println!("Loaded {} references.", references.len());

    let app = Router::new()
        .route("/", get(health_check))
        .route("/version", get(version))
        .route("/api/references", get(get_references))
        .route("/preview/citation", post(preview_citation))
        .route("/preview/bibliography", post(preview_bibliography))
        .route("/api/v1/decide", post(decide_handler))
        .route("/api/v1/preview", post(preview_set_handler))
        .route("/api/v1/generate", post(generate_handler))
        .with_state(state)
        .layer(tower_http::cors::CorsLayer::permissive());

    let addr = SocketAddr::from(([0, 0, 0, 0], 3000));
    println!("listening on {}", addr);
    
    let listener = tokio::net::TcpListener::bind(addr).await.unwrap();
    axum::serve(listener, app).await.unwrap();
}

async fn health_check() -> &'static str {
    "OK"
}

async fn version() -> Json<Value> {
    Json(json!({
        "service": "style-editor-server",
        "csln_core_version": "git-latest"
    }))
}

async fn get_references(State(state): State<Arc<AppState>>) -> Json<HashMap<String, Reference>> {
    Json(state.references.clone())
}

#[derive(Deserialize)]
struct PreviewRequest {
    style: Style,
    references: Vec<Reference>,
}

#[derive(Serialize)]
struct PreviewResponse {
    result: String,
}

async fn preview_citation(Json(payload): Json<PreviewRequest>) -> Json<PreviewResponse> {
    println!("Handling preview_citation request");
    // 1. Convert Vec<Reference> to Bibliography (IndexMap)
    let bib: Bibliography = payload.references
        .into_iter()
        .map(|r| (r.id().clone().unwrap_or_default(), r))
        .collect();

    // 2. Identify IDs to cite (for now just cite them all)
    let cite_ids: Vec<String> = bib.keys().cloned().collect();

    // 3. Initialize Processor
    let processor = Processor::new(payload.style, bib);

    // 4. Create Citation object
    let citation = Citation {
        id: Some("preview-1".to_string()),
        items: cite_ids.into_iter().map(|id| CitationItem { id, ..Default::default() }).collect(),
        ..Default::default()
    };

    // 5. Render
    let result = match processor.process_citation(&citation) {
        Ok(res) => res,
        Err(e) => {
            println!("preview_citation error: {}", e);
            format!("Error: {}", e)
        },
    };

    Json(PreviewResponse { result })
}

async fn preview_bibliography(Json(payload): Json<PreviewRequest>) -> Json<PreviewResponse> {
    println!("Handling preview_bibliography request");
    let bib: Bibliography = payload.references
        .into_iter()
        .map(|r| (r.id().clone().unwrap_or_default(), r))
        .collect();

    let processor = Processor::new(payload.style, bib);
    let output = processor.process_references();
    
    // Simple join of entries for now, typically this would be a list
    let result = match output.bibliography.is_empty() {
        true => String::new(),
        false => output.bibliography.iter()
            .map(|entry| csln_processor::citation_to_string(entry, None, None, None, None))
            .collect::<Vec<String>>()
            .join("\n")
    };

    Json(PreviewResponse { result })
}

#[derive(Default, Serialize, Deserialize)]
struct PreviewSet {
    in_text: Option<String>,
    note: Option<String>,
    bibliography: Option<String>,
}

async fn preview_set_handler(
    State(state): State<Arc<AppState>>,
    Json(intent): Json<StyleIntent>
) -> Json<PreviewSet> {
    Json(generate_preview_set(&intent, &state.references))
}

/// Helper to generate preview HTML for a given intent and references
fn generate_preview_set(intent: &StyleIntent, references: &HashMap<String, Reference>) -> PreviewSet {
    let mut set = PreviewSet::default();
    // Generate real preview using the processor
    let style = intent.to_style();
    println!("Generated style: {:?}", style);
    
    // Pick a few diverse references to cite for the preview
    // Prioritize specific references that show off style features
    let candidates = ["vaswani_attention", "foucault_discipline", "brown_v_board"];
    let mut cite_ids = Vec::new();
    for id in candidates {
        if references.contains_key(id) {
            cite_ids.push(id.to_string());
        }
    }
    
    // Fallback to random ones if none found
    if cite_ids.is_empty() {
        cite_ids = references.keys().take(3).cloned().collect();
    }

    if cite_ids.is_empty() {
        return set;
    }

    if let Some(class) = &intent.class {
        // Create a small bibliography containing only the cited references
        let bib: Bibliography = references.iter()
            .filter(|(k, _)| cite_ids.contains(k))
            .map(|(k, v)| (k.clone(), v.clone()))
            .collect();

        let processor = Processor::new(style, bib);
        let citation = Citation {
            id: Some("preview-1".to_string()),
            items: cite_ids.into_iter().map(|id| CitationItem { id, ..Default::default() }).collect(),
            ..Default::default()
        };

        if let Ok(res) = processor.process_citation(&citation) {
            if !res.trim().is_empty() {
                match class {
                    intent_engine::CitationClass::AuthorDate => set.in_text = Some(res),
                    intent_engine::CitationClass::Footnote | intent_engine::CitationClass::Endnote => set.note = Some(res),
                    intent_engine::CitationClass::Numeric => set.in_text = Some(res), 
                }
            }
        }

        if intent.has_bibliography.unwrap_or(false) {
            let bib_output = processor.process_references();
            if !bib_output.bibliography.is_empty() {
                let mut bib_html = String::new();
                for entry in bib_output.bibliography {
                    let bib_str = csln_processor::citation_to_string(&entry, None, None, None, None);
                    bib_html.push_str(&format!("<div class='bib-entry'>{}</div>", bib_str));
                }
                set.bibliography = Some(bib_html);
            }
        }
    }
    set
}

/// Handler for the `/api/v1/decide` endpoint.
/// 
/// Receives the current `StyleIntent` from the frontend and determines:
/// 1. What is missing?
/// 2. What is the next logical question to ask?
/// 3. What are the preview options for that question?
async fn decide_handler(
    State(state): State<Arc<AppState>>,
    Json(intent): Json<StyleIntent>
) -> Json<DecisionPackage> {
    println!("Handling decide request: {:?}", intent);
    // Call the engine to determine the next decision based on current intent
    let mut package = intent.decide();

    // 1. Generate live preview for the CURRENT intent
    let current_previews = generate_preview_set(&intent, &state.references);
    package.in_text_preview = current_previews.in_text;
    package.note_preview = current_previews.note;
    package.bibliography_preview = current_previews.bibliography;

    // 2. Generate live previews for EACH choice
    for preview in &mut package.previews {
        match serde_json::to_value(&intent) {
            Ok(mut intent_val) => {
                if let Some(obj) = intent_val.as_object_mut() {
                    if let Some(choice_obj) = preview.choice_value.as_object() {
                        for (k, v) in choice_obj {
                            obj.insert(k.clone(), v.clone());
                        }
                    }
                }

                if let Ok(temp_intent) = serde_json::from_value::<StyleIntent>(intent_val) {
                    let p_set = generate_preview_set(&temp_intent, &state.references);
                    // For choices, we still want a single HTML string for the small card preview
                    let mut html = String::new();
                    if let Some(it) = p_set.in_text { html.push_str(&format!("<div class='cit'>{}</div>", it)); }
                    if let Some(nt) = p_set.note { html.push_str(&format!("<div class='cit'>{}</div>", nt)); }
                    if let Some(bb) = p_set.bibliography { html.push_str(&format!("<div class='bib'>{}</div>", bb)); }
                    preview.html = html;
                }
            },
            Err(e) => println!("Error serializing intent: {}", e),
        }
    }

    Json(package)
}

/// Handler for the `/api/v1/generate` endpoint.
/// 
/// Receives the final `StyleIntent` and returns the complete CSLN YAML.
async fn generate_handler(Json(intent): Json<StyleIntent>) -> (axum::http::HeaderMap, String) {
    let csln = intent.generate_csln();
    
    let mut headers = axum::http::HeaderMap::new();
    headers.insert(
        axum::http::header::CONTENT_TYPE,
        axum::http::HeaderValue::from_static("application/x-yaml"),
    );
    headers.insert(
        axum::http::header::CONTENT_DISPOSITION,
        axum::http::HeaderValue::from_static("attachment; filename=\"custom-style.yaml\""),
    );

    (headers, csln)
}