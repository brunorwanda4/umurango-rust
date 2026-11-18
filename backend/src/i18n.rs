//! Internationalization (i18n) and localized error translation for Umuryango Backend.

use actix_web::HttpRequest;
use std::collections::HashMap;
use std::sync::OnceLock;

/// Languages supported by the Umuryango platform.
#[derive(Clone, Copy, Debug, Default, PartialEq, Eq)]
pub enum SupportedLanguage {
    #[default]
    English,
    Kinyarwanda,
}

impl SupportedLanguage {
    /// Parses an ISO 639 language code or string into `SupportedLanguage`.
    pub fn from_code(code: &str) -> Self {
        let clean = code.trim().to_lowercase();
        if clean.starts_with("rw") || clean.contains("kinyarwanda") {
            SupportedLanguage::Kinyarwanda
        } else {
            SupportedLanguage::English
        }
    }

    /// Returns standard 2-letter lowercase language code.
    pub fn code(self) -> &'static str {
        match self {
            SupportedLanguage::English => "en",
            SupportedLanguage::Kinyarwanda => "rw",
        }
    }
}

fn dictionary(language: SupportedLanguage) -> &'static HashMap<String, String> {
    static ENGLISH: OnceLock<HashMap<String, String>> = OnceLock::new();
    static KINYARWANDA: OnceLock<HashMap<String, String>> = OnceLock::new();

    match language {
        SupportedLanguage::English => ENGLISH.get_or_init(|| {
            serde_json::from_str(include_str!("../../assets/locales/en.json"))
                .expect("English locale dictionary must be valid JSON")
        }),
        SupportedLanguage::Kinyarwanda => KINYARWANDA.get_or_init(|| {
            serde_json::from_str(include_str!("../../assets/locales/rw.json"))
                .expect("Kinyarwanda locale dictionary must be valid JSON")
        }),
    }
}

/// Extracts the user's preferred language from the incoming HTTP request.
/// Checks `x-lang` header, `?lang=` query parameter, and standard `Accept-Language` header.
pub fn extract_language(req: &HttpRequest) -> SupportedLanguage {
    // 1. Check custom "x-lang" header
    if let Some(val) = req.headers().get("x-lang").and_then(|v| v.to_str().ok()) {
        return SupportedLanguage::from_code(val);
    }

    // 2. Check query parameter "?lang=..."
    if let Ok(query) =
        actix_web::web::Query::<HashMap<String, String>>::from_query(req.query_string())
    {
        if let Some(lang) = query.get("lang") {
            return SupportedLanguage::from_code(lang);
        }
    }

    // 3. Check standard "Accept-Language" header (e.g., "rw-RW,rw;q=0.9,en;q=0.8")
    if let Some(accept_lang) = req
        .headers()
        .get("Accept-Language")
        .and_then(|v| v.to_str().ok())
    {
        for part in accept_lang.split(',') {
            let tag = part.split(';').next().unwrap_or("").trim().to_lowercase();
            if tag.starts_with("rw") {
                return SupportedLanguage::Kinyarwanda;
            }
        }
    }

    SupportedLanguage::English
}

/// Translates an error key or known error sentence into the specified language.
pub fn translate_error(raw: &str, lang: SupportedLanguage) -> String {
    let dict = dictionary(lang);

    // 1. Direct key match (e.g. "error.invalid_credentials")
    if let Some(msg) = dict.get(raw) {
        return msg.clone();
    }

    // 2. Map standard English error sentences to localization keys
    let mapped_key = match raw {
        "Full name is required" => "error.name_required",
        "Phone number is required" => "error.phone_required",
        "Password must be at least 4 characters" => "error.password_min",
        "An account with this phone number or email already exists" => "error.account_exists",
        "Phone number or email is required" => "error.identifier_required",
        "Password is required" => "error.password_required",
        "Invalid phone number or password" => "error.invalid_credentials",
        "Account is suspended or deactivated" => "error.account_disabled",
        "User not found" => "error.user_not_found",
        "Family name cannot be empty" => "error.family_name_empty",
        "Family join code is required" => "error.join_code_required",
        "Invalid family code. Please check the code and try again." => "error.invalid_join_code",
        "Invalid family code" => "error.invalid_join_code",
        "Invitation not found or expired" => "error.invite_not_found",
        "Missing or invalid Authorization header" => "error.unauthorized_missing_token",
        "Invalid or expired token" => "error.unauthorized_invalid_token",
        "Unauthorized access" => "error.unauthorized",
        _ => return raw.to_string(),
    };

    dict.get(mapped_key)
        .cloned()
        .unwrap_or_else(|| raw.to_string())
}

#[cfg(test)]
mod tests {
    use super::*;

    #[test]
    fn test_translate_error_english() {
        let msg = translate_error(
            "Invalid phone number or password",
            SupportedLanguage::English,
        );
        assert_eq!(msg, "Invalid phone number or password");

        let msg_key = translate_error("error.invalid_credentials", SupportedLanguage::English);
        assert_eq!(msg_key, "Invalid phone number or password");
    }

    #[test]
    fn test_translate_error_kinyarwanda() {
        let msg = translate_error(
            "Invalid phone number or password",
            SupportedLanguage::Kinyarwanda,
        );
        assert_eq!(msg, "Nimero ya telefone cyangwa ijambo ry'ibanga sibyo");

        let msg_key = translate_error("error.invalid_credentials", SupportedLanguage::Kinyarwanda);
        assert_eq!(msg_key, "Nimero ya telefone cyangwa ijambo ry'ibanga sibyo");

        let join_msg = translate_error(
            "Family join code is required",
            SupportedLanguage::Kinyarwanda,
        );
        assert_eq!(join_msg, "Kode yo kwinjira mu muryango irakenewe");
    }

    #[test]
    fn test_supported_language_from_code() {
        assert_eq!(
            SupportedLanguage::from_code("rw"),
            SupportedLanguage::Kinyarwanda
        );
        assert_eq!(
            SupportedLanguage::from_code("rw-RW"),
            SupportedLanguage::Kinyarwanda
        );
        assert_eq!(
            SupportedLanguage::from_code("kinyarwanda"),
            SupportedLanguage::Kinyarwanda
        );
        assert_eq!(
            SupportedLanguage::from_code("en"),
            SupportedLanguage::English
        );
        assert_eq!(
            SupportedLanguage::from_code("en-US"),
            SupportedLanguage::English
        );
        assert_eq!(
            SupportedLanguage::from_code("fr"),
            SupportedLanguage::English
        );
    }

    #[test]
    fn test_extract_language_from_headers_and_query() {
        let req_rw_header = actix_web::test::TestRequest::default()
            .insert_header(("Accept-Language", "rw-RW,rw;q=0.9,en;q=0.8"))
            .to_http_request();
        assert_eq!(
            extract_language(&req_rw_header),
            SupportedLanguage::Kinyarwanda
        );

        let req_x_lang = actix_web::test::TestRequest::default()
            .insert_header(("x-lang", "rw"))
            .to_http_request();
        assert_eq!(
            extract_language(&req_x_lang),
            SupportedLanguage::Kinyarwanda
        );

        let req_query = actix_web::test::TestRequest::default()
            .uri("/api/v1/auth/login?lang=rw")
            .to_http_request();
        assert_eq!(extract_language(&req_query), SupportedLanguage::Kinyarwanda);

        let req_default = actix_web::test::TestRequest::default().to_http_request();
        assert_eq!(extract_language(&req_default), SupportedLanguage::English);
    }
}
