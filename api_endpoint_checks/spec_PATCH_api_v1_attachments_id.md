# API Endpoint Check

- Method: `PATCH`
- Endpoint (Spec): `/api/v1/attachments/:id`
- Endpoint (Executed): `/api/v1/attachments/None`

## 有权限

### curl

```bash
curl -sS -X PATCH 'http://127.0.0.1:3001/api/v1/attachments/None' \
  -H 'Authorization: Bearer <ADMIN_JWT>' \
  -H 'Content-Type: application/json' \
  -d '{"attachment": {"description": "updated desc"}}'
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
curl -sS -X PATCH 'http://127.0.0.1:3001/api/v1/attachments/None' \
  -H 'Authorization: Bearer <REGULAR_JWT>' \
  -H 'Content-Type: application/json' \
  -d '{"attachment": {"description": "denied"}}'
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

