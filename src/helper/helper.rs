pub fn is_option_has_string_value(data: &Option<String>) -> bool {
    data.as_deref().map(|s| !s.trim().is_empty()).unwrap_or(false)
}