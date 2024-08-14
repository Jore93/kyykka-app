use lambda_http::{run, service_fn, tracing, Body, Error, Request, RequestExt, Response};

fn create(event: Request) -> Result<Response<Body>, Error> {
    Ok(Response::builder()
        .status(201)
        .header("content-type", "text/html")
        .body("Hello, World!".into())
        .map_err(Box::new)?)
}

fn read(event: Request) -> Result<Response<Body>, Error> {
    Ok(Response::builder()
        .status(200)
        .header("content-type", "text/html")
        .body("Hello, World!".into())
        .map_err(Box::new)?)
}

fn update(event: Request) -> Result<Response<Body>, Error> {
    Ok(Response::builder()
        .status(200)
        .header("content-type", "text/html")
        .body("Hello, World!".into())
        .map_err(Box::new)?)
}

fn delete(event: Request) -> Result<Response<Body>, Error> {
    Ok(Response::builder()
        .status(200)
        .header("content-type", "text/html")
        .body("Hello, World!".into())
        .map_err(Box::new)?)
}

/// This is the main body for the function.
/// Write your code inside it.
/// There are some code example in the following URLs:
/// - https://github.com/awslabs/aws-lambda-rust-runtime/tree/main/examples
async fn user_function_handler(event: Request) -> Result<Response<Body>, Error> {
    // Extract some useful information from the request
    let who = event
        .query_string_parameters_ref()
        .and_then(|params| params.first("name"))
        .unwrap_or("world");

    if event.method() == http::Method::POST {
        return create(event);
    } else if event.method() == http::Method::GET {
        return read(event);
    } else if event.method() == http::Method::PUT {
        return update(event);
    } else if event.method() == http::Method::DELETE {
        return delete(event);
    }
}

#[tokio::main]
async fn main() -> Result<(), Error> {
    tracing::init_default_subscriber();

    run(service_fn(user_function_handler)).await
}
