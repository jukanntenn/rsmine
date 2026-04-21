# API Endpoint Check

- Method: `POST`
- Endpoint (Spec): `/api/v1/projects/:id/memberships`
- Endpoint (Executed): `/api/v1/projects/1/memberships`

## 有权限

### curl

```bash
curl -sS -X POST 'http://127.0.0.1:3001/api/v1/projects/1/memberships' \
  -H 'Authorization: Bearer <ADMIN_JWT>' \
  -H 'Content-Type: application/json' \
  -d '{"membership": {"user_id": 1, "role_ids": [2]}}'
```

### Response

```json
{
  "http_status": 200,
  "response": {
    "membership": {
      "id": 193,
      "project": {
        "id": 1,
        "name": "Updated ByteHome Project"
      },
      "user": {
        "id": 1,
        "name": "Admin User"
      },
      "roles": [
        {
          "id": 2,
          "name": "Developer"
        }
      ]
    }
  }
}
```

## 无权限

### curl

```bash
curl -sS -X POST 'http://127.0.0.1:3001/api/v1/projects/1/memberships' \
  -H 'Authorization: Bearer <REGULAR_JWT>' \
  -H 'Content-Type: application/json' \
  -d '{"membership": {"user_id": 1, "role_ids": [2]}}'
```

### Response

```json
{
  "http_status": 403,
  "response": {
    "errors": [
      "You don't have permission to manage members in this project"
    ]
  }
}
```

