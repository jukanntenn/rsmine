# API Endpoint Check

- Method: `PUT`
- Endpoint (Spec): `/api/v1/issue_statuses/:id`
- Endpoint (Executed): `/api/v1/issue_statuses/21`

## 有权限

### curl

```bash
curl -sS -X PUT 'http://127.0.0.1:3001/api/v1/issue_statuses/21' \
  -H 'Authorization: Bearer <ADMIN_JWT>' \
  -H 'Content-Type: application/json' \
  -d '{"issue_status": {"name": "QA Status Upd 317037"}}'
```

### Response

```json
{
  "http_status": 200,
  "response": {
    "issue_status": {
      "id": 21,
      "name": "QA Status Upd 317037",
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
curl -sS -X PUT 'http://127.0.0.1:3001/api/v1/issue_statuses/21' \
  -H 'Authorization: Bearer <REGULAR_JWT>' \
  -H 'Content-Type: application/json' \
  -d '{"issue_status": {"name": "denied"}}'
```

### Response

```json
{
  "http_status": 403,
  "response": {
    "errors": [
      "Only administrators can update issue statuses"
    ]
  }
}
```

