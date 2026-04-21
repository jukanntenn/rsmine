# API Endpoint Check

- Method: `GET`
- Endpoint (Spec): `/api/v1/trackers`
- Endpoint (Executed): `/api/v1/trackers`

## 有权限

### curl

```bash
curl -sS -X GET 'http://127.0.0.1:3001/api/v1/trackers' \
  -H 'Authorization: Bearer <ADMIN_JWT>'
```

### Response

```json
{
  "http_status": 200,
  "response": {
    "trackers": [
      {
        "id": 1,
        "name": "Bug",
        "position": 1,
        "is_in_roadmap": true,
        "default_status": {
          "id": 1,
          "name": "New"
        },
        "description": null,
        "enabled_standard_fields": [
          "assigned_to_id",
          "category_id",
          "fixed_version_id",
          "parent_issue_id",
          "start_date",
          "due_date",
          "estimated_hours",
          "done_ratio",
          "description",
          "priority_id"
        ]
      },
      {
        "id": 2,
        "name": "Feature",
        "position": 2,
        "is_in_roadmap": true,
        "default_status": {
          "id": 1,
          "name": "New"
        },
        "description": null,
        "enabled_standard_fields": [
          "assigned_to_id",
          "category_id",
          "fixed_version_id",
          "parent_issue_id",
          "start_date",
          "due_date",
          "estimated_hours",
          "done_ratio",
          "description",
          "priority_id"
        ]
      },
      {
        "id": 5,
        "name": "QA Tracker",
        "position": 2,
        "is_in_roadmap": true,
        "default_status": {
          "id": 1,
          "name": "New"
        },
        "description": null,
        "enabled_standard_fields": [
          "assigned_to_id",
          "category_id",
          "fixed_version_id",
          "parent_issue_id",
          "start_date",
          "due_date",
          "estimated_hours",
          "done_ratio",
          "description",
          "priority_id"
        ]
      },
      {
        "id": 6,
        "name": "QA Tracker 314654",
        "position": 2,
        "is_in_roadmap": true,
        "default_status": {
          "id": 1,
          "name": "New"
        },
        "description": null,
        "enabled_standard_fields": [
          "assigned_to_id",
          "category_id",
          "fixed_version_id",
          "parent_issue_id",
          "start_date",
          "due_date",
          "estimated_hours",
          "done_ratio",
          "description",
          "priority_id"
        ]
      },
      {
        "id": 7,
        "name": "QA Tracker Del 314654",
        "position": 2,
        "is_in_roadmap": true,
        "default_status": {
          "id": 1,
          "name": "New"
        },
        "description": null,
        "enabled_standard_fields": [
          "assigned_to_id",
          "category_id",
          "fixed_version_id",
          "parent_issue_id",
          "start_date",
          "due_date",
          "estimated_hours",
          "done_ratio",
          "description",
          "priority_id"
        ]
      },
      {
        "id": 8,
        "name": "QA Tracker 314657",
        "position": 2,
        "is_in_roadmap": true,
        "default_status": {
          "id": 1,
          "name": "New"
        },
        "description": null,
        "enabled_standard_fields": [
          "assigned_to_id",
          "category_id",
          "fixed_version_id",
          "parent_issue_id",
          "start_date",
          "due_date",
          "estimated_hours",
          "done_ratio",
          "description",
          "priority_id"
        ]
      },
      {
        "id": 9,
        "name": "QA Tracker Del 314657",
        "position": 2,
        "is_in_roadmap": true,
        "default_status": {
          "id": 1,
          "name": "New"
        },
        "description": null,
        "enabled_standard_fields": [
          "assigned_to_id",
          "category_id",
          "fixed_version_id",
          "parent_issue_id",
          "start_date",
          "due_date",
          "estimated_hours",
          "done_ratio",
          "description",
          "priority_id"
        ]
      },
      {
        "id": 10,
        "name": "QA Tracker 314766",
        "position": 2,
        "is_in_roadmap": true,
        "default_status": {
          "id": 1,
          "name": "New"
        },
        "description": null,
        "enabled_standard_fields": [
          "assigned_to_id",
          "category_id",
          "fixed_version_id",
          "parent_issue_id",
          "start_date",
          "due_date",
          "estimated_hours",
          "done_ratio",
          "description",
          "priority_id"
        ]
      },
      {
        "id": 11,
        "name": "QA Tracker Del 314766",
        "position": 2,
        "is_in_roadmap": true,
        "default_status": {
          "id": 1,
          "name": "New"
        },
        "description": null,
        "enabled_standard_fields": [
          "assigned_to_id",
          "category_id",
          "fixed_version_id",
          "parent_issue_id",
          "start_date",
          "due_date",
          "estimated_hours",
          "done_ratio",
          "description",
          "priority_id"
        ]
      },
      {
        "id": 12,
        "name": "QA Track Upd 314822",
        "position": 2,
        "is_in_roadmap": true,
        "default_status": {
          "id": 1,
          "name": "New"
        },
        "description": null,
        "enabled_standard_fields": [
          "assigned_to_id",
          "category_id",
          "fixed_version_id",
          "parent_issue_id",
          "start_date",
          "due_date",
          "estimated_hours",
          "done_ratio",
          "description",
          "priority_id"
        ]
      },
      {
        "id": 14,
        "name": "QA Track New 314822",
        "position": 2,
        "is_in_roadmap": true,
        "default_status": {
          "id": 1,
          "name": "New"
        },
        "description": null,
        "enabled_standard_fields": [
          "assigned_to_id",
          "category_id",
          "fixed_version_id",
          "parent_issue_id",
          "start_date",
          "due_date",
          "estimated_hours",
          "done_ratio",
          "description",
          "priority_id"
        ]
      },
      {
        "id": 15,
        "name": "QA Tracker 317037",
        "position": 2,
        "is_in_roadmap": true,
        "default_status": {
          "id": 1,
          "name": "New"
        },
        "description": null,
        "enabled_standard_fields": [
          "assigned_to_id",
          "category_id",
          "fixed_version_id",
          "parent_issue_id",
          "start_date",
          "due_date",
          "estimated_hours",
          "done_ratio",
          "description",
          "priority_id"
        ]
      },
      {
        "id": 16,
        "name": "QA Tracker Del 317037",
        "position": 2,
        "is_in_roadmap": true,
        "default_status": {
          "id": 1,
          "name": "New"
        },
        "description": null,
        "enabled_standard_fields": [
          "assigned_to_id",
          "category_id",
          "fixed_version_id",
          "parent_issue_id",
          "start_date",
          "due_date",
          "estimated_hours",
          "done_ratio",
          "description",
          "priority_id"
        ]
      },
      {
        "id": 3,
        "name": "Support",
        "position": 3,
        "is_in_roadmap": false,
        "default_status": {
          "id": 1,
          "name": "New"
        },
        "description": null,
        "enabled_standard_fields": [
          "assigned_to_id",
          "category_id",
          "fixed_version_id",
          "parent_issue_id",
          "start_date",
          "due_date",
          "estimated_hours",
          "done_ratio",
          "description",
          "priority_id"
        ]
      }
    ]
  }
}
```

## 无权限

### curl

```bash
curl -sS -X GET 'http://127.0.0.1:3001/api/v1/trackers'
```

### Response

```json
{
  "http_status": 401,
  "response": {}
}
```

