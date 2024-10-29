use axum::http::{Request, Response};
use tower_http::{
    classify::{ServerErrorsAsFailures, SharedClassifier},
    trace::{DefaultMakeSpan, OnRequest, OnResponse, TraceLayer},
};
use tracing::Span;

#[derive(Default, Clone)]
pub struct LogRequest {}

// TODO: Replace with whatever logging format you want to support
impl<B> OnRequest<B> for LogRequest {
    fn on_request(&mut self, request: &Request<B>, _span: &Span) {
        tracing::info!("start: {} {}", request.method(), request.uri());
    }
}

impl<B> OnResponse<B> for LogRequest {
    fn on_response(self, response: &Response<B>, latency: std::time::Duration, _span: &Span) {
        tracing::info!("finished: {} {}", response.status(), latency.as_micros());
    }
}

impl LogRequest {
    pub fn layer(
    ) -> TraceLayer<SharedClassifier<ServerErrorsAsFailures>, DefaultMakeSpan, LogRequest, LogRequest>
    {
        TraceLayer::new_for_http()
            .on_request(LogRequest::default())
            .on_response(LogRequest::default())
    }
}
