// use axum::http::HeaderName;
// use tower::ServiceBuilder;
// use tower::Layer;
// use tower_http::{
//     request_id::{MakeRequestUuid, PropagateRequestIdLayer, SetRequestIdLayer},
//     trace::TraceLayer,
//     cors::{CorsLayer, Any},
// };

// /// ✅ Middleware: attach a unique request ID
// pub fn request_id_layer() -> impl Layer<axum::Router> {
//     ServiceBuilder::new()
//         .layer(SetRequestIdLayer::new(
//             HeaderName::from_static("x-request-id"),
//             MakeRequestUuid,
//         ))
//         .layer(PropagateRequestIdLayer::new(
//             HeaderName::from_static("x-request-id"),
//         ))
// }

// /// ✅ Middleware: simple real IP / tracing layer
// pub fn real_ip_layer() -> TraceLayer {
//     TraceLayer::new_for_http()
// }

// /// ✅ Middleware: recover / trace layer (kept simple)
// pub fn recover_layer() -> TraceLayer {
//     TraceLayer::new_for_http()
// }

// /// ✅ CORS layer helper
// pub fn cors_layer() -> CorsLayer {
//     CorsLayer::new()
//         .allow_origin(Any)
//         .allow_methods(Any)
//         .allow_headers(Any)
// }

// pub fn stack() -> impl Layer<axum::Router> {
//     ServiceBuilder::new()
//         .layer(request_id_layer())
//         .layer(real_ip_layer())
//         .layer(recover_layer())
//         .layer(cors_layer())
// }