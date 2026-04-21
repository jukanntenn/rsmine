# API Endpoint Check

- Method: `POST`
- Endpoint (Spec): `/api/v1/projects/:id/issue_categories`
- Endpoint (Executed): `/api/v1/projects/1/issue_categories`

## 有权限

### curl

```bash
curl -sS -X POST 'http://127.0.0.1:3001/api/v1/projects/1/issue_categories' \
  -H 'Authorization: Bearer <ADMIN_JWT>' \
  -H 'Content-Type: application/json' \
  -d '{"issue_category": {"name": "qa-new-317037"}}'
```

### Response

```json
{
  "http_status": 201,
  "response": {
    "issue_category": {
      "id": 30,
      "name": "qa-new-317037",
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
curl -sS -X POST 'http://127.0.0.1:3001/api/v1/projects/1/issue_categories' \
  -H 'Authorization: Bearer <REGULAR_JWT>' \
  -H 'Content-Type: application/json' \
  -d '{"issue_category": {"name": "qa-den-317037"}}'
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

