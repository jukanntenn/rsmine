# API Endpoint Check

- Method: `DELETE`
- Endpoint (Spec): `/api/v1/memberships/:id`
- Endpoint (Executed): `/api/v1/memberships/191`

## 有权限

### curl

```bash
curl -sS -X DELETE 'http://127.0.0.1:3001/api/v1/memberships/191' \
  -H 'Authorization: Bearer <ADMIN_JWT>'
```

### Response

```json
{
  "http_status": 204,
  "response": {}
}
```

## 无权限

### curl

```bash
curl -sS -X DELETE 'http://127.0.0.1:3001/api/v1/memberships/191' \
  -H 'Authorization: Bearer <REGULAR_JWT>'
```

### Response

```json
{
  "http_status": 404,
  "response": {
    "errors": [
      "Membership not found"
    ]
  }
}
```

