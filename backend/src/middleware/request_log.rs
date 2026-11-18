use std::time::{Duration, Instant};

use actix_web::body::BoxBody;
use actix_web::dev::{ServiceRequest, ServiceResponse};
use actix_web::http::{Method, StatusCode};
use actix_web::middleware::Next;
use actix_web::Error;
use chrono::Local;

const RESET: &str = "\x1b[0m";
const DIM: &str = "\x1b[2m";
const WHITE: &str = "\x1b[37m";
const CYAN: &str = "\x1b[36m";
const GREEN: &str = "\x1b[32m";
const YELLOW: &str = "\x1b[33m";
const RED: &str = "\x1b[31m";
const MAGENTA: &str = "\x1b[35m";
const BOLD_RED: &str = "\x1b[1;31m";
const METHOD_WIDTH: usize = 7;
const STATUS_WIDTH: usize = 16;
const IP_WIDTH: usize = 15;
const PATH_WIDTH: usize = 36;
const SLOW_REQUEST_THRESHOLD: Duration = Duration::from_millis(500);

pub async fn request_log(
    request: ServiceRequest,
    next: Next<BoxBody>,
) -> Result<ServiceResponse<BoxBody>, Error> {
    let method = request.method().clone();
    let raw_path = request.path().to_owned();
    let matched_pattern = request.match_pattern();
    let client_ip = request_client_ip(&request);
    let started_at = Instant::now();

    match next.call(request).await {
        Ok(response) => {
            let path = response
                .request()
                .match_pattern()
                .or(matched_pattern)
                .unwrap_or(raw_path);
            print_request_line(
                &method,
                &path,
                &client_ip,
                response.status(),
                started_at.elapsed(),
            );
            Ok(response)
        }
        Err(error) => {
            let elapsed = started_at.elapsed();
            let status = error.as_response_error().status_code();
            print_request_line(
                &method,
                &matched_pattern.unwrap_or(raw_path),
                &client_ip,
                status,
                elapsed,
            );
            Err(error)
        }
    }
}

fn request_client_ip(request: &ServiceRequest) -> String {
    request
        .peer_addr()
        .map(|address| address.ip().to_string())
        .unwrap_or_else(|| "127.0.0.1".to_owned())
}

fn print_request_line(
    method: &Method,
    path: &str,
    client_ip: &str,
    status: StatusCode,
    elapsed: Duration,
) {
    if !should_print(path, status) {
        return;
    }

    println!(
        "{}",
        format_request_line(
            &Local::now().format("%H:%M:%S").to_string(),
            method,
            path,
            client_ip,
            status,
            elapsed,
        )
    );
}

fn should_print(path: &str, status: StatusCode) -> bool {
    status.as_u16() >= 400 || path != "/health"
}

fn format_request_line(
    time: &str,
    method: &Method,
    path: &str,
    client_ip: &str,
    status: StatusCode,
    elapsed: Duration,
) -> String {
    let time = colorize(DIM, fit_column(time, 8));
    let method = colorize(
        method_color(method),
        fit_column(method.as_str(), METHOD_WIDTH),
    );
    let status_text = format!("{} {}", status.as_u16(), display_reason(status));
    let status = colorize(status_color(status), fit_column(&status_text, STATUS_WIDTH));
    let client_ip = colorize(MAGENTA, format!("{client_ip:<IP_WIDTH$}"));
    let path = fit_column(path, PATH_WIDTH);
    let duration = format!("{:>5} ms", elapsed.as_millis());
    let slow = if elapsed >= SLOW_REQUEST_THRESHOLD {
        format!("  {BOLD_RED}SLOW{RESET}")
    } else {
        String::new()
    };

    format!("  {time}  {method}  {status}  {client_ip}  {path}  {duration}{slow}")
}

fn display_reason(status: StatusCode) -> &'static str {
    if status == StatusCode::INTERNAL_SERVER_ERROR {
        "Server Error"
    } else {
        status.canonical_reason().unwrap_or("Unknown")
    }
}

fn method_color(method: &Method) -> &'static str {
    match *method {
        Method::GET => CYAN,
        Method::POST => GREEN,
        Method::PUT | Method::PATCH => YELLOW,
        Method::DELETE => RED,
        _ => WHITE,
    }
}

fn status_color(status: StatusCode) -> &'static str {
    match status.as_u16() {
        200..=299 => GREEN,
        300..=399 => CYAN,
        400..=499 => YELLOW,
        500..=599 => BOLD_RED,
        _ => WHITE,
    }
}

fn fit_column(value: &str, width: usize) -> String {
    if value.len() > width {
        format!("{}...", &value[..width.saturating_sub(3)])
    } else {
        format!("{value:<width$}")
    }
}

fn colorize(color: &str, value: String) -> String {
    format!("{color}{value}{RESET}")
}

#[cfg(test)]
mod tests {
    use super::*;

    #[test]
    fn formats_method_status_ip_path_and_duration() {
        let line = format_request_line(
            "12:34:56",
            &Method::POST,
            "/api/v1/families",
            "127.0.0.1",
            StatusCode::CREATED,
            Duration::from_millis(42),
        );

        assert!(line.contains("POST"));
        assert!(line.contains("201 Created"));
        assert!(line.contains("127.0.0.1"));
        assert!(line.contains("/api/v1/families"));
        assert!(line.contains("42 ms"));
    }

    #[test]
    fn marks_slow_requests() {
        let line = format_request_line(
            "12:34:56",
            &Method::GET,
            "/api/v1/families",
            "127.0.0.1",
            StatusCode::OK,
            Duration::from_millis(501),
        );

        assert!(line.contains("SLOW"));
    }

    #[test]
    fn quiets_successful_health_checks() {
        assert!(!should_print("/health", StatusCode::OK));
        assert!(should_print("/health", StatusCode::INTERNAL_SERVER_ERROR));
    }
}
