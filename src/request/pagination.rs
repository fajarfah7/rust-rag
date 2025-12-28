use serde::Deserialize;

#[derive(Deserialize, Debug)]
pub struct PaginationRequest {
    pub page: Option<u32>,
    pub per_page: Option<u32>,
    pub offset: Option<u32>,
    pub search: Option<String>,
    pub sort: Option<String>,
}

impl PaginationRequest {
    pub fn format_sort(&self) -> Option<String> {
        let raw_sorts = match &self.sort {
            Some(v) if !v.trim().is_empty() => v,
            _ => return None,
        };

        let mut receiver = Vec::new();

        for s in raw_sorts.split(",") {
            let s = s.trim();
            if s.is_empty() {
                continue;
            }

            if let Some(field) = s.strip_prefix("-") {
                receiver.push(format!("{field} DESC"));
            } else {
                receiver.push(format!("{s} ASC"));
            }
        }
        Some(receiver.join(","))
    }

}
