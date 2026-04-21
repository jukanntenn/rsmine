# API Endpoint Check

- Method: `PUT`
- Endpoint (Spec): `/api/v1/trackers/:id`
- Endpoint (Executed): `/api/v1/trackers/15`

## 有权限

### curl

```bash
curl -sS -X PUT 'http://127.0.0.1:3001/api/v1/trackers/15' \
  -H 'Authorization: Bearer <ADMIN_JWT>' \
  -H 'Content-Type: application/json' \
  -d '{"tracker": {"name": "QA Track Upd 317037"}}'
```

### Response

```json
{
  "http_status": 200,
  "response": {
    "tracker": {
      "id": 15,
      "name": "QA Track Upd 317037",
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
curl -sS -X PUT 'http://127.0.0.1:3001/api/v1/trackers/15' \
  -H 'Authorization: Bearer <REGULAR_JWT>' \
  -H 'Content-Type: application/json' \
  -d '{"tracker": {"name": "denied"}}'
```

### Response

```json
{
  "http_status": 403,
  "response": {
    "errors": [
      "Only administrators can update trackers"
    ]
  }
}
```

