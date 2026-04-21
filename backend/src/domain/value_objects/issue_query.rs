/// Query parameters for listing issues
#[derive(Debug, Clone, Default)]
pub struct IssueQueryParams {
    /// Filter by project ID
    pub project_id: Option<i32>,

    /// Filter by status ID or "open"/"closed"
    pub status_id: Option<String>,

    /// Filter by tracker ID
    pub tracker_id: Option<i32>,

    /// Filter by priority ID
    pub priority_id: Option<i32>,

    /// Filter by category ID
    pub category_id: Option<i32>,

    /// Filter by assignee ID or "me"
    pub assigned_to_id: Option<String>,

    /// Filter by author ID
    pub author_id: Option<i32>,

    /// Filter by subject (fuzzy search)
    pub subject: Option<String>,

    /// Filter by parent issue ID
    pub parent_id: Option<i32>,

    /// Filter by created_on (>=YYYY-MM-DD)
    pub created_on: Option<String>,

    /// Filter by updated_on (>=YYYY-MM-DD)
    pub updated_on: Option<String>,

    /// Pagination offset (default: 0)
    pub offset: u32,

    /// Items per page (default: 25, max: 100)
    pub limit: u32,

    /// Sort field and direction (e.g., "created_on:desc")
    pub sort: Option<String>,
}

impl IssueQueryParams {
    #[allow(clippy::too_many_arguments)]
    pub fn new(
        project_id: Option<i32>,
        status_id: Option<String>,
        tracker_id: Option<i32>,
        priority_id: Option<i32>,
        category_id: Option<i32>,
        assigned_to_id: Option<String>,
        author_id: Option<i32>,
        subject: Option<String>,
        parent_id: Option<i32>,
        created_on: Option<String>,
        updated_on: Option<String>,
        offset: u32,
        limit: u32,
        sort: Option<String>,
    ) -> Self {
        Self {
            project_id,
            status_id,
            tracker_id,
            priority_id,
            category_id,
            assigned_to_id,
            author_id,
            subject,
            parent_id,
            created_on,
            updated_on,
            offset,
            limit: limit.min(100), // Max 100 items per page
            sort,
        }
    }

    /// Parse status_id to get either a specific status ID or "open"/"closed" filter
    pub fn parsed_status_id(&self) -> Option<Result<i32, String>> {
        self.status_id.as_ref().map(|s| match s.as_str() {
            "open" => Err("open".to_string()),
            "closed" => Err("closed".to_string()),
            id => id
                .parse::<i32>()
                .map_err(|_| format!("Invalid status_id: {}", s)),
        })
    }

    /// Parse assigned_to_id to get either a specific user ID or the current user's ID
    pub fn parsed_assigned_to_id(&self, current_user_id: i32) -> Option<i32> {
        self.assigned_to_id.as_ref().and_then(|s| match s.as_str() {
            "me" => Some(current_user_id),
            id => id.parse::<i32>().ok(),
        })
    }
}
