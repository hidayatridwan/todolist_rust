use serde::{Deserialize, Serialize};

#[derive(Deserialize)]
pub struct FilterPaginationQuery {
    pub page: Option<i64>,
    pub limit: Option<i64>,
    pub search_field: Option<String>,
    pub search_value: Option<String>,
    pub completed: Option<String>,
    pub sort_field: Option<String>,
    pub sort_order: Option<String>,
}

#[derive(Serialize)]
pub struct PaginationMeta {
    pub page: i64,
    pub limit: i64,
    pub total_records: i64,
    pub total_pages: i64,
}

impl FilterPaginationQuery {
    pub fn normalize(&self) -> (i64, i64) {
        let page = self.page.unwrap_or(1).max(1);
        let limit = self.limit.unwrap_or(10).clamp(1, 100);
        (page, limit)
    }

    pub fn parse_completed(&self) -> Option<Vec<bool>> {
        self.completed.as_ref().map(|v| {
            v.split(",")
                .filter_map(|s| s.parse::<bool>().ok())
                .collect::<Vec<bool>>()
        })
    }

    pub fn sort_field(&self) -> &'static str {
        match self.sort_field.as_deref() {
            Some("title") => "title",
            Some("description") => "description",
            Some("completed") => "completed",
            Some("created_at") => "created_at",
            _ => "created_at",
        }
    }

    pub fn sort_order(&self) -> &'static str {
        match self.sort_order.as_deref() {
            Some("asc") => "ASC",
            Some("desc") => "DESC",
            _ => "DESC",
        }
    }
}
