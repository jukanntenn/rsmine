# API Endpoint Check

- Method: `GET`
- Endpoint (Spec): `/api/v1/roles/:id`
- Endpoint (Executed): `/api/v1/roles/19`

## 有权限

### curl

```bash
curl -sS -X GET 'http://127.0.0.1:3001/api/v1/roles/19' \
  -H 'Authorization: Bearer <ADMIN_JWT>'
```

### Response

```json
{
  "http_status": 200,
  "response": {
    "role": {
      "id": 19,
      "name": "QA Role 317037",
      "permissions": [
        "view_project"
      ],
      "issues_visibility": "default"
    }
  }
}
```

## 无权限

### curl

```bash
curl -sS -X GET 'http://127.0.0.1:3001/api/v1/roles/19'
```

### Response

```json
{
  "http_status": 401,
  "response": {}
}
```

