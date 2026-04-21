# API Endpoint Check

- Method: `PUT`
- Endpoint (Spec): `/api/v1/roles/:id`
- Endpoint (Executed): `/api/v1/roles/19`

## 有权限

### curl

```bash
curl -sS -X PUT 'http://127.0.0.1:3001/api/v1/roles/19' \
  -H 'Authorization: Bearer <ADMIN_JWT>' \
  -H 'Content-Type: application/json' \
  -d '{"role": {"name": "QA Role Upd 317037"}}'
```

### Response

```json
{
  "http_status": 200,
  "response": {
    "role": {
      "id": 19,
      "name": "QA Role Upd 317037",
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
curl -sS -X PUT 'http://127.0.0.1:3001/api/v1/roles/19' \
  -H 'Authorization: Bearer <REGULAR_JWT>' \
  -H 'Content-Type: application/json' \
  -d '{"role": {"name": "denied"}}'
```

### Response

```json
{
  "http_status": 403,
  "response": {
    "errors": [
      "Only administrators can update roles"
    ]
  }
}
```

