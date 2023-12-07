use axum::response::Response;

pub async fn response_logger(res: Response) -> Response {
    let status = res.status().to_string();
    println!("call end :: {status:?}");
    println!();
    return res;
}
