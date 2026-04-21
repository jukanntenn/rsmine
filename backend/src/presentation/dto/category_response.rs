use crate::application::use_cases::category::{
    CreateCategoryDetail, GetCategoryDetail, NamedId, UpdateCategoryDetail,
};
use crate::application::use_cases::{
    CategoryItem, CategoryListResponse, CreateCategoryResponse, GetCategoryResponse,
    UpdateCategoryResponse,
};
use serde::{Deserialize, Serialize};

/// Named ID JSON response
#[derive(Debug, Serialize, Deserialize)]
pub struct NamedIdJson {
    pub id: i32,
    pub name: String,
}

impl From<NamedId> for NamedIdJson {
    fn from(named_id: NamedId) -> Self {
        Self {
            id: named_id.id,
            name: named_id.name,
        }
    }
}

/// Category JSON response
#[derive(Debug, Serialize, Deserialize)]
pub struct CategoryJson {
    pub id: i32,
    pub name: String,
    pub project: NamedIdJson,
    #[serde(skip_serializing_if = "Option::is_none")]
    pub assigned_to: Option<NamedIdJson>,
}

impl From<CategoryItem> for CategoryJson {
    fn from(item: CategoryItem) -> Self {
        Self {
            id: item.id,
            name: item.name,
            project: NamedIdJson::from(item.project),
            assigned_to: item.assigned_to.map(NamedIdJson::from),
        }
    }
}

/// Response for GET /api/v1/projects/:id/issue_categories.json
#[derive(Debug, Serialize, Deserialize)]
pub struct CategoryListJsonResponse {
    pub issue_categories: Vec<CategoryJson>,
    pub total_count: u32,
}

impl From<CategoryListResponse> for CategoryListJsonResponse {
    fn from(response: CategoryListResponse) -> Self {
        Self {
            issue_categories: response
                .issue_categories
                .into_iter()
                .map(CategoryJson::from)
                .collect(),
            total_count: response.total_count,
        }
    }
}

/// Category detail JSON response
#[derive(Debug, Serialize, Deserialize)]
pub struct CategoryDetailJson {
    pub id: i32,
    pub name: String,
    pub project: NamedIdJson,
    #[serde(skip_serializing_if = "Option::is_none")]
    pub assigned_to: Option<NamedIdJson>,
}

impl From<GetCategoryDetail> for CategoryDetailJson {
    fn from(detail: GetCategoryDetail) -> Self {
        Self {
            id: detail.id,
            name: detail.name,
            project: NamedIdJson::from(detail.project),
            assigned_to: detail.assigned_to.map(NamedIdJson::from),
        }
    }
}

impl From<CreateCategoryDetail> for CategoryDetailJson {
    fn from(detail: CreateCategoryDetail) -> Self {
        Self {
            id: detail.id,
            name: detail.name,
            project: NamedIdJson::from(detail.project),
            assigned_to: detail.assigned_to.map(NamedIdJson::from),
        }
    }
}

impl From<UpdateCategoryDetail> for CategoryDetailJson {
    fn from(detail: UpdateCategoryDetail) -> Self {
        Self {
            id: detail.id,
            name: detail.name,
            project: NamedIdJson::from(detail.project),
            assigned_to: detail.assigned_to.map(NamedIdJson::from),
        }
    }
}

/// Response for GET /api/v1/issue_categories/:id.json
#[derive(Debug, Serialize, Deserialize)]
pub struct GetCategoryJsonResponse {
    pub issue_category: CategoryDetailJson,
}

impl From<GetCategoryResponse> for GetCategoryJsonResponse {
    fn from(response: GetCategoryResponse) -> Self {
        Self {
            issue_category: CategoryDetailJson::from(response.issue_category),
        }
    }
}

/// Response for POST /api/v1/projects/:id/issue_categories.json
#[derive(Debug, Serialize, Deserialize)]
pub struct CreateCategoryJsonResponse {
    pub issue_category: CategoryDetailJson,
}

impl From<CreateCategoryResponse> for CreateCategoryJsonResponse {
    fn from(response: CreateCategoryResponse) -> Self {
        Self {
            issue_category: CategoryDetailJson::from(response.issue_category),
        }
    }
}

/// Response for PUT /api/v1/issue_categories/:id.json
#[derive(Debug, Serialize, Deserialize)]
pub struct UpdateCategoryJsonResponse {
    pub issue_category: CategoryDetailJson,
}

impl From<UpdateCategoryResponse> for UpdateCategoryJsonResponse {
    fn from(response: UpdateCategoryResponse) -> Self {
        Self {
            issue_category: CategoryDetailJson::from(response.issue_category),
        }
    }
}
