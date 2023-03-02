use lambda_http::request::RequestContext;
use lambda_http::{service_fn, Error};
use lambda_http::{IntoResponse, Request, RequestExt};

async fn function_handler(request: Request) -> Result<impl IntoResponse, std::convert::Infallible> {
    let _context = request.lambda_context();
    let apigw_context = match request.request_context() {
        RequestContext::ApiGatewayV2(x) => x,
        _ => panic!("Unexpected Request Context Type"),
    };

    Ok(format!(
        "hello {} - {:?}",
        request
            .query_string_parameters()
            .first("name")
            .unwrap_or_else(|| "stranger"),
        apigw_context
    ))
}

#[tokio::main]
async fn main() -> Result<(), Error> {
    tracing_subscriber::fmt()
        .with_max_level(tracing::Level::INFO)
        // disable printing the name of the module in every log line.
        .with_target(false)
        // disabling time is handy because CloudWatch will add the ingestion time.
        .without_time()
        .init();

    // initialize dependencies once here for the lifetime of your
    // lambda task
    lambda_http::run(service_fn(function_handler)).await?;
    Ok(())
}
