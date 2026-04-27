
use crate::models::{CompletedTodo, CreateTodo, Todo, UpdateTodo};
use crate::handlers;

use axum::{routing::get, Json, Router};
use utoipa::{
    //openapi::security::{HttpAuthScheme, HttpBuilder, SecurityScheme},
    //Modify, 
    OpenApi,
};

const OPENAPI_URL: &str = "/api-docs/openapi.json";

#[derive(OpenApi)]
#[openapi(
    info(
        description = "Todo app made by Adam Rachid and his dad Abdessamad Rachid",
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
        handlers::export_csv
    ),
    components(
        schemas(Todo, CreateTodo, UpdateTodo, CompletedTodo)
    ),
    tags((name = "todo", description = "Todo management API"))
)]
pub struct ApiDoc;


async fn openapi_spec() -> Json<utoipa::openapi::OpenApi> {
    Json(ApiDoc::openapi())
}


//custom HTML template to remove scalar upsell like  The AI Agent chat button and The "Open in Client" button (upsell to Scalar desktop app)
// We use a function or a lazy static to build this
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