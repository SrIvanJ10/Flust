use axum::{
    http::StatusCode,
    response::{IntoResponse, Response},
    routing::{get, post},
    Json, Router,
};
use flust_codegen::generator;
use flust_core::ir::Flow;
use serde::{Deserialize, Serialize};
use std::net::SocketAddr;
use tower_http::cors::{Any, CorsLayer};
use env_logger;
use std::fs;
use std::process::Command;
use tempfile::TempDir;

#[tokio::main]
async fn main() {
    env_logger::init();

    let app = Router::new()
        .route("/api/health", get(health_check))
        .route("/api/compile", post(compile_flow))
        .route("/api/execute", post(execute_code))
        .layer(
            CorsLayer::new()
                .allow_origin(Any)
                .allow_methods(Any)
                .allow_headers(Any),
        );

    let addr = SocketAddr::from(([127, 0, 0, 1], 3000));
    println!("🚀 Flust Server listening on {}", addr);
    println!("📡 API endpoints:");
    println!("   - GET  /api/health");
    println!("   - POST /api/compile");
    println!("   - POST /api/execute");
    
    let listener = tokio::net::TcpListener::bind(addr).await.unwrap();
    axum::serve(listener, app).await.unwrap();
}

async fn health_check() -> &'static str {
    "OK"
}

#[derive(Deserialize)]
struct ExecuteRequest {
    code: String,
    filename: String,
}

#[derive(Serialize)]
struct ExecuteResponse {
    success: bool,
    compile_output: String,
    execution_output: String,
    error: Option<String>,
}

async fn execute_code(
    Json(request): Json<ExecuteRequest>,
) -> Result<Json<ExecuteResponse>, ApiError> {
    println!("📥 Received execution request for: {}", request.filename);

    // Use output_project directory in the workspace root
    let project_path = std::path::Path::new("./output_project");
    
    if !project_path.exists() {
         return Err(ApiError::InternalError("output_project directory not found. Make sure you are running from the workspace root.".to_string()));
    }

    let src_path = project_path.join("src");
    let main_rs = src_path.join("main.rs");

    // Write code to src/main.rs
    fs::write(&main_rs, &request.code).map_err(|e| {
        ApiError::InternalError(format!("Failed to write file: {}", e))
    })?;

    println!("📝 Wrote code to: {}", main_rs.display());

    // Execute with cargo run
    println!("▶️  Executing with cargo run...");
    let exec_result = Command::new("cargo")
        .arg("run")
        .arg("--quiet")
        .current_dir(project_path)
        .output()
        .map_err(|e| {
            ApiError::InternalError(format!("Failed to execute cargo run: {}", e))
        })?;

    let stdout = String::from_utf8_lossy(&exec_result.stdout).to_string();
    let stderr = String::from_utf8_lossy(&exec_result.stderr).to_string();

    if !exec_result.status.success() {
        println!("❌ Execution failed");
        return Ok(Json(ExecuteResponse {
            success: false,
            compile_output: stderr,
            execution_output: stdout,
            error: Some("Execution failed".to_string()),
        }));
    }

    println!("✅ Execution successful");

    Ok(Json(ExecuteResponse {
        success: true,
        compile_output: stderr, // Warnings might be here
        execution_output: stdout,
        error: None,
    }))
}



#[derive(Serialize)]
struct CompileResponse {
    code: String,
}

#[derive(Serialize)]
struct ErrorResponse {
    error: String,
}

/// Custom error type for API responses
enum ApiError {
    CompilationError(anyhow::Error),
    InternalError(String),
}

impl IntoResponse for ApiError {
    fn into_response(self) -> Response {
        match self {
            ApiError::CompilationError(err) => {
                let error_msg = err.to_string();
                eprintln!("❌ Compilation error: {}", error_msg);
                
                (
                    StatusCode::BAD_REQUEST,
                    Json(ErrorResponse { error: error_msg }),
                ).into_response()
            }
            ApiError::InternalError(msg) => {
                eprintln!("❌ Internal error: {}", msg);
                
                (
                    StatusCode::INTERNAL_SERVER_ERROR,
                    Json(ErrorResponse { error: msg }),
                ).into_response()
            }
        }
    }
}

impl<E> From<E> for ApiError
where
    E: Into<anyhow::Error>,
{
    fn from(err: E) -> Self {
        Self::CompilationError(err.into())
    }
}

async fn compile_flow(Json(flow): Json<Flow>) -> Result<Json<CompileResponse>, ApiError> {
    println!("📥 Received compilation request:");
    println!("   - Nodes: {}", flow.nodes.len());
    println!("   - Connections: {}", flow.connections.len());
    
    for node in &flow.nodes {
        println!("   - Node {}: type={}", node.id, node.plugin_type);
    }
    
    let code = generator::generate_rust(&flow)?;
    
    println!("✅ Compilation successful");
    println!("📝 Generated {} lines of code", code.lines().count());
    
    Ok(Json(CompileResponse { code }))
}
