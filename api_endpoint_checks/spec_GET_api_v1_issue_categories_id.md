# API Endpoint Check

- Method: `GET`
- Endpoint (Spec): `/api/v1/issue_categories/:id`
- Endpoint (Executed): `/api/v1/issue_categories/28`

## 有权限

### curl

```bash
curl -sS -X GET 'http://127.0.0.1:3001/api/v1/issue_categories/28' \
  -H 'Authorization: Bearer <ADMIN_JWT>'
```

### Response

```json
{
  "http_status": 200,
  "response": {
    "issue_category": {
      "id": 28,
      "name": "QA Cat 317037",
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
curl -sS -X GET 'http://127.0.0.1:3001/api/v1/issue_categories/28'
```

### Response

```json
{
  "http_status": 401,
  "response": {}
}
```

