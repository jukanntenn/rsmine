# API Endpoint Check

- Method: `POST`
- Endpoint (Spec): `/api/v1/roles`
- Endpoint (Executed): `/api/v1/roles`

## 有权限

### curl

```bash
curl -sS -X POST 'http://127.0.0.1:3001/api/v1/roles' \
  -H 'Authorization: Bearer <ADMIN_JWT>' \
  -H 'Content-Type: application/json' \
  -d '{"role": {"name": "QA Role New 317037", "permissions": ["view_project"], "issues_visibility": "default", "assignable": true}}'
```

### Response

```json
{
  "http_status": 201,
  "response": {
    "role": {
      "id": 21,
      "name": "QA Role New 317037",
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
curl -sS -X POST 'http://127.0.0.1:3001/api/v1/roles' \
  -H 'Authorization: Bearer <REGULAR_JWT>' \
  -H 'Content-Type: application/json' \
  -d '{"role": {"name": "Denied Role 317037", "permissions": ["view_project"], "issues_visibility": "default", "assignable": true}}'
```

### Response

```json
{
  "http_status": 403,
  "response": {
    "errors": [
      "Only administrators can create roles"
    ]
  }
}
```

