# API Endpoint Check

- Method: `DELETE`
- Endpoint (Spec): `/api/v1/relations/:id`
- Endpoint (Executed): `/api/v1/relations/None`

## 有权限

### curl

```bash
curl -sS -X DELETE 'http://127.0.0.1:3001/api/v1/relations/None' \
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
curl -sS -X DELETE 'http://127.0.0.1:3001/api/v1/relations/None' \
  -H 'Authorization: Bearer <REGULAR_JWT>'
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

