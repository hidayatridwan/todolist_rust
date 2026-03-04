use serde::{Deserialize, Serialize};

#[derive(Deserialize)]
pub struct FilterPaginationQuery {
    pub page: Option<i64>,
    pub limit: Option<i64>,
    pub search_field: Option<String>,
    pub search_value: Option<String>,
    pub completed: Option<String>,
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
}
