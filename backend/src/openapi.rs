
use crate::models::{CompletedTask, CreateTask, Task, UpdateTask};
use crate::handlers;

use axum::{routing::get, Json, Router};
use utoipa::OpenApi;

const OPENAPI_URL: &str = "/api-docs/openapi.json";

#[derive(OpenApi)]
#[openapi(
    info(
        description = "Task Management App made by Adam Rachid",
        license(name = "All Rights Reserved"),        
    ),    
    paths(
        handlers::get_all,
        handlers::get_one,
        handlers::create,
        handlers::update,
        handlers::deletes,
        handlers::set_completed,
        handlers::multi_create,
        handlers::search_task,
        handlers::import_csv,
        handlers::export_csv
    ),
    components(
        schemas(Task, CreateTask, UpdateTask, CompletedTask)
    ),
    tags((name = "task", description = "Task management API"))
)]
pub struct ApiDoc;


async fn openapi_spec() -> Json<utoipa::openapi::OpenApi> {
    Json(ApiDoc::openapi())
}

fn get_scalar_html() -> String {
    format!(r##"<!doctype html>
<html>
<head>
  <title>API Docs</title>
  <meta charset="utf-8" />
  <meta name="viewport" content="width=device-width, initial-scale=1" />
</head>
<body>
  <div id="app"></div>
  <script src="https://cdn.jsdelivr.net/npm/@scalar/api-reference"></script>
  <script>
    Scalar.createApiReference("#app", {{
        url: "{url}",
        agent: {{ disabled: true }},
        hideClientButton: true,
        customCss: `            
            .scalar-mcp-layer,
            [class*="powered-by"],
            [class*="PoweredBy"],
            a[href*="scalar.com"] {{ display: none !important; }}
        `,
    }});
  </script>
</body>
</html>"##, url = OPENAPI_URL)
}


pub fn create_router() -> Router {

    Router::new()        
        .route(OPENAPI_URL, get(openapi_spec))
        .route("/scalar", get(|| async { axum::response::Html(get_scalar_html()) }))
        // .merge(SwaggerUi::new("/swagger/swagger-ui").url("/swagger/api-docs/openapi.json", ApiDoc::openapi()))                
        // .merge(Redoc::with_url("/redoc", ApiDoc::openapi()))
        // .merge(RapiDoc::with_openapi("/rapidoc-api-docs/openapi.json", ApiDoc::openapi()).path("/rapidoc"))
}     