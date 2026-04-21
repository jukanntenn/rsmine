# API Endpoint Check

- Method: `POST`
- Endpoint (Spec): `/api/v1/trackers`
- Endpoint (Executed): `/api/v1/trackers`

## 有权限

### curl

```bash
curl -sS -X POST 'http://127.0.0.1:3001/api/v1/trackers' \
  -H 'Authorization: Bearer <ADMIN_JWT>' \
  -H 'Content-Type: application/json' \
  -d '{"tracker": {"name": "QA Track New 317037", "default_status_id": 1}}'
```

### Response

```json
{
  "http_status": 201,
  "response": {
    "tracker": {
      "id": 17,
      "name": "QA Track New 317037",
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
    }
  }
}
```

## 无权限

### curl

```bash
curl -sS -X POST 'http://127.0.0.1:3001/api/v1/trackers' \
  -H 'Authorization: Bearer <REGULAR_JWT>' \
  -H 'Content-Type: application/json' \
  -d '{"tracker": {"name": "Denied Track 317037", "default_status_id": 1}}'
```

### Response

```json
{
  "http_status": 403,
  "response": {
    "errors": [
      "Only administrators can create trackers"
    ]
  }
}
```

