# API Endpoint Check

- Method: `PUT`
- Endpoint (Spec): `/api/v1/issues/:id`
- Endpoint (Executed): `/api/v1/issues/None`

## 有权限

### curl

```bash
curl -sS -X PUT 'http://127.0.0.1:3001/api/v1/issues/None' \
  -H 'Authorization: Bearer <ADMIN_JWT>' \
  -H 'Content-Type: application/json' \
  -d '{"issue": {"done_ratio": 30, "notes": "upd"}}'
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
curl -sS -X PUT 'http://127.0.0.1:3001/api/v1/issues/None' \
  -H 'Authorization: Bearer <REGULAR_JWT>' \
  -H 'Content-Type: application/json' \
  -d '{"issue": {"done_ratio": 40}}'
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

