mod create_category;
mod delete_category;
mod get_category;
mod list_categories;
mod update_category;

// Re-export types from list_categories
pub use list_categories::{CategoryItem, CategoryListResponse, ListCategoriesUseCase, NamedId};

// Re-export types from get_category
pub use get_category::{
    CategoryDetail as GetCategoryDetail, GetCategoryResponse, GetCategoryUseCase,
};

// Re-export types from create_category
pub use create_category::{
    CategoryDetail as CreateCategoryDetail, CreateCategoryResponse, CreateCategoryUseCase,
};

// Re-export types from update_category
pub use update_category::{
    CategoryDetail as UpdateCategoryDetail, UpdateCategoryResponse, UpdateCategoryUseCase,
};

// Re-export delete category
pub use delete_category::DeleteCategoryUseCase;
