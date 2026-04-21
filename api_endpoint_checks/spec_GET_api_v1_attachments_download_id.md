# API Endpoint Check

- Method: `GET`
- Endpoint (Spec): `/api/v1/attachments/download/:id`
- Endpoint (Executed): `/api/v1/attachments/download/None`

## 有权限

### curl

```bash
curl -sS -X GET 'http://127.0.0.1:3001/api/v1/attachments/download/None' \
  -H 'Authorization: Bearer <ADMIN_JWT>'
```

### Response

```json
{
  "http_status": 400,
  "response": {
    "raw_body": "Invalid URL: Cannot parse `None` to a `i32`"
  }
}
```

## 无权限

### curl

```bash
curl -sS -X GET 'http://127.0.0.1:3001/api/v1/attachments/download/None'
```

### Response

```json
{
  "http_status": 401,
  "response": {}
}
```

