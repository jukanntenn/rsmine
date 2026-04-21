# API Endpoint Check

- Method: `PUT`
- Endpoint (Spec): `/api/v1/memberships/:id`
- Endpoint (Executed): `/api/v1/memberships/191`

## 有权限

### curl

```bash
curl -sS -X PUT 'http://127.0.0.1:3001/api/v1/memberships/191' \
  -H 'Authorization: Bearer <ADMIN_JWT>' \
  -H 'Content-Type: application/json' \
  -d '{"membership": {"role_ids": [1]}}'
```

### Response

```json
{
  "http_status": 200,
  "response": {
    "membership": {
      "id": 191,
      "project": {
        "id": 1,
        "name": "Updated ByteHome Project"
      },
      "user": {
        "id": 20,
        "name": "Qa Mem"
      },
      "roles": [
        {
          "id": 1,
          "name": "Manager"
        }
      ]
    }
  }
}
```

## 无权限

### curl

```bash
curl -sS -X PUT 'http://127.0.0.1:3001/api/v1/memberships/191' \
  -H 'Authorization: Bearer <REGULAR_JWT>' \
  -H 'Content-Type: application/json' \
  -d '{"membership": {"role_ids": [2]}}'
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

