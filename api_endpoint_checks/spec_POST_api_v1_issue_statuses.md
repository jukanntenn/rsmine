# API Endpoint Check

- Method: `POST`
- Endpoint (Spec): `/api/v1/issue_statuses`
- Endpoint (Executed): `/api/v1/issue_statuses`

## 有权限

### curl

```bash
curl -sS -X POST 'http://127.0.0.1:3001/api/v1/issue_statuses' \
  -H 'Authorization: Bearer <ADMIN_JWT>' \
  -H 'Content-Type: application/json' \
  -d '{"issue_status": {"name": "QA Status New 317037", "is_closed": false}}'
```

### Response

```json
{
  "http_status": 201,
  "response": {
    "issue_status": {
      "id": 23,
      "name": "QA Status New 317037",
      "is_closed": false,
      "is_default": false,
      "default_done_ratio": null,
      "description": null
    }
  }
}
```

## 无权限

### curl

```bash
curl -sS -X POST 'http://127.0.0.1:3001/api/v1/issue_statuses' \
  -H 'Authorization: Bearer <REGULAR_JWT>' \
  -H 'Content-Type: application/json' \
  -d '{"issue_status": {"name": "Denied Status 317037", "is_closed": false}}'
```

### Response

```json
{
  "http_status": 403,
  "response": {
    "errors": [
      "Only administrators can create issue statuses"
    ]
  }
}
```

