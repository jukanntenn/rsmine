# API Endpoint Check

- Method: `PUT`
- Endpoint (Spec): `/api/v1/issue_categories/:id`
- Endpoint (Executed): `/api/v1/issue_categories/28`

## 有权限

### curl

```bash
curl -sS -X PUT 'http://127.0.0.1:3001/api/v1/issue_categories/28' \
  -H 'Authorization: Bearer <ADMIN_JWT>' \
  -H 'Content-Type: application/json' \
  -d '{"issue_category": {"name": "updated cat"}}'
```

### Response

```json
{
  "http_status": 200,
  "response": {
    "issue_category": {
      "id": 28,
      "name": "updated cat",
      "project": {
        "id": 1,
        "name": "Updated ByteHome Project"
      }
    }
  }
}
```

## 无权限

### curl

```bash
curl -sS -X PUT 'http://127.0.0.1:3001/api/v1/issue_categories/28' \
  -H 'Authorization: Bearer <REGULAR_JWT>' \
  -H 'Content-Type: application/json' \
  -d '{"issue_category": {"name": "deny cat"}}'
```

### Response

```json
{
  "http_status": 403,
  "response": {
    "errors": [
      "You don't have permission to manage categories in this project"
    ]
  }
}
```

