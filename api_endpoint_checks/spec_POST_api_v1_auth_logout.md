# API Endpoint Check

- Method: `POST`
- Endpoint (Spec): `/api/v1/auth/logout`
- Endpoint (Executed): `/api/v1/auth/logout`

## 有权限

### curl

```bash
curl -sS -X POST 'http://127.0.0.1:3001/api/v1/auth/logout' \
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
curl -sS -X POST 'http://127.0.0.1:3001/api/v1/auth/logout'
```

### Response

```json
{
  "http_status": 401,
  "response": {}
}
```

