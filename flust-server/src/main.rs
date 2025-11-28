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

    // Create temporary directory
    let temp_dir = TempDir::new().map_err(|e| {
        ApiError::InternalError(format!("Failed to create temp dir: {}", e))
    })?;
    
    let temp_path = temp_dir.path();
    let rs_file = temp_path.join(format!("{}.rs", request.filename));
    let binary_file = temp_path.join(&request.filename);

    // Write code to file
    fs::write(&rs_file, &request.code).map_err(|e| {
        ApiError::InternalError(format!("Failed to write file: {}", e))
    })?;

    println!("📝 Wrote code to: {}", rs_file.display());

    // Compile with rustc
    println!("🔨 Compiling with rustc...");
    let compile_result = Command::new("rustc")
        .arg(&rs_file)
        .arg("-o")
        .arg(&binary_file)
        .current_dir(temp_path)
        .output()
        .map_err(|e| {
            ApiError::InternalError(format!("Failed to run rustc: {}", e))
        })?;

    let compile_output = format!(
        "{}{}",
        String::from_utf8_lossy(&compile_result.stdout),
        String::from_utf8_lossy(&compile_result.stderr)
    );

    if !compile_result.status.success() {
        println!("❌ Compilation failed");
        return Ok(Json(ExecuteResponse {
            success: false,
            compile_output,
            execution_output: String::new(),
            error: Some("Compilation failed".to_string()),
        }));
    }

    println!("✅ Compilation successful");

    // Execute the binary
    println!("▶️  Executing binary...");
    let exec_result = Command::new(&binary_file)
        .current_dir(temp_path)
        .output()
        .map_err(|e| {
            ApiError::InternalError(format!("Failed to execute binary: {}", e))
        })?;

    let execution_output = format!(
        "{}{}",
        String::from_utf8_lossy(&exec_result.stdout),
        String::from_utf8_lossy(&exec_result.stderr)
    );

    println!("✅ Execution completed");
    println!("📤 Output: {} bytes", execution_output.len());

    Ok(Json(ExecuteResponse {
        success: true,
        compile_output: if compile_output.is_empty() {
            "Compilation successful (no warnings)".to_string()
        } else {
            compile_output
        },
        execution_output,
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
