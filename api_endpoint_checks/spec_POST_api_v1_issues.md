# API Endpoint Check

- Method: `POST`
- Endpoint (Spec): `/api/v1/issues`
- Endpoint (Executed): `/api/v1/issues`

## 有权限

### curl

```bash
curl -sS -X POST 'http://127.0.0.1:3001/api/v1/issues' \
  -H 'Authorization: Bearer <ADMIN_JWT>' \
  -H 'Content-Type: application/json' \
  -d '{"issue": {"project_id": 1, "tracker_id": 2, "subject": "qa-create-317037"}}'
```

### Response

```json
{
  "http_status": 400,
  "response": {
    "errors": [
      "Tracker is not available in this project"
    ]
  }
}
```

## 无权限

### curl

```bash
curl -sS -X POST 'http://127.0.0.1:3001/api/v1/issues' \
  -H 'Authorization: Bearer <REGULAR_JWT>' \
  -H 'Content-Type: application/json' \
  -d '{"issue": {"project_id": 1, "tracker_id": 2, "subject": "qa-deny-317037"}}'
```

### Response

```json
{
  "http_status": 403,
  "response": {
    "errors": [
      "You don't have permission to create issues in this project"
    ]
  }
}
```

