use crate::application::use_cases::{
    CreateTrackerResponse, ProjectTrackersResponse, TrackerDefaultStatus, TrackerItem,
    TrackerListResponse, UpdateTrackerResponse,
};
use serde::{Deserialize, Serialize};

/// Default status JSON response
#[derive(Debug, Serialize, Deserialize)]
pub struct DefaultStatusJson {
    pub id: i32,
    pub name: String,
}

impl From<TrackerDefaultStatus> for DefaultStatusJson {
    fn from(status: TrackerDefaultStatus) -> Self {
        Self {
            id: status.id,
            name: status.name,
        }
    }
}

/// Tracker JSON response for single tracker
#[derive(Debug, Serialize, Deserialize)]
pub struct TrackerJson {
    pub id: i32,
    pub name: String,
    pub position: Option<i32>,
    pub is_in_roadmap: bool,
    pub default_status: Option<DefaultStatusJson>,
    pub description: Option<String>,
    pub enabled_standard_fields: Vec<String>,
}

impl From<TrackerItem> for TrackerJson {
    fn from(item: TrackerItem) -> Self {
        Self {
            id: item.id,
            name: item.name,
            position: item.position,
            is_in_roadmap: item.is_in_roadmap,
            default_status: item.default_status.map(DefaultStatusJson::from),
            description: item.description,
            enabled_standard_fields: item.enabled_standard_fields,
        }
    }
}

/// Response for GET /api/v1/trackers.json
#[derive(Debug, Serialize, Deserialize)]
pub struct TrackerListJsonResponse {
    pub trackers: Vec<TrackerJson>,
}

impl From<TrackerListResponse> for TrackerListJsonResponse {
    fn from(response: TrackerListResponse) -> Self {
        Self {
            trackers: response
                .trackers
                .into_iter()
                .map(TrackerJson::from)
                .collect(),
        }
    }
}

impl From<ProjectTrackersResponse> for TrackerListJsonResponse {
    fn from(response: ProjectTrackersResponse) -> Self {
        Self {
            trackers: response
                .trackers
                .into_iter()
                .map(TrackerJson::from)
                .collect(),
        }
    }
}

/// Response for GET /api/v1/trackers/:id.json
#[derive(Debug, Serialize, Deserialize)]
pub struct GetTrackerJsonResponse {
    pub tracker: TrackerJson,
}

impl From<TrackerItem> for GetTrackerJsonResponse {
    fn from(tracker: TrackerItem) -> Self {
        Self {
            tracker: TrackerJson::from(tracker),
        }
    }
}

/// Response for POST /api/v1/trackers.json
#[derive(Debug, Serialize, Deserialize)]
pub struct CreateTrackerJsonResponse {
    pub tracker: TrackerJson,
}

impl From<CreateTrackerResponse> for CreateTrackerJsonResponse {
    fn from(response: CreateTrackerResponse) -> Self {
        Self {
            tracker: TrackerJson::from(response.tracker),
        }
    }
}

/// Response for PUT /api/v1/trackers/:id.json
#[derive(Debug, Serialize, Deserialize)]
pub struct UpdateTrackerJsonResponse {
    pub tracker: TrackerJson,
}

impl From<UpdateTrackerResponse> for UpdateTrackerJsonResponse {
    fn from(response: UpdateTrackerResponse) -> Self {
        Self {
            tracker: TrackerJson::from(response.tracker),
        }
    }
}
