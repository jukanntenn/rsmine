use serde::{Deserialize, Serialize};

#[derive(Debug, Serialize, Deserialize)]
pub struct PagedResponse<T> {
    pub items: Vec<T>,
    pub total_count: u64,
    pub page: u32,
    pub per_page: u32,
    pub total_pages: u32,
}

impl<T> PagedResponse<T> {
    pub fn new(items: Vec<T>, total_count: u64, page: u32, per_page: u32) -> Self {
        let total_pages = if per_page > 0 {
            ((total_count as f64) / (per_page as f64)).ceil() as u32
        } else {
            0
        };

        Self {
            items,
            total_count,
            page,
            per_page,
            total_pages,
        }
    }
}
