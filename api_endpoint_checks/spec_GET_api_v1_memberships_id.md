# API Endpoint Check

- Method: `GET`
- Endpoint (Spec): `/api/v1/memberships/:id`
- Endpoint (Executed): `/api/v1/memberships/191`

## 有权限

### curl

```bash
curl -sS -X GET 'http://127.0.0.1:3001/api/v1/memberships/191' \
  -H 'Authorization: Bearer <ADMIN_JWT>'
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
curl -sS -X GET 'http://127.0.0.1:3001/api/v1/memberships/191'
```

### Response

```json
{
  "http_status": 401,
  "response": {}
}
```

