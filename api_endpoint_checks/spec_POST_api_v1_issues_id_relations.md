# API Endpoint Check

- Method: `POST`
- Endpoint (Spec): `/api/v1/issues/:id/relations`
- Endpoint (Executed): `/api/v1/issues/None/relations`

## 有权限

### curl

```bash
curl -sS -X POST 'http://127.0.0.1:3001/api/v1/issues/None/relations' \
  -H 'Authorization: Bearer <ADMIN_JWT>' \
  -H 'Content-Type: application/json' \
  -d '{"relation": {"issue_to_id": null, "relation_type": "blocks"}}'
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
curl -sS -X POST 'http://127.0.0.1:3001/api/v1/issues/None/relations' \
  -H 'Authorization: Bearer <REGULAR_JWT>' \
  -H 'Content-Type: application/json' \
  -d '{"relation": {"issue_to_id": null, "relation_type": "blocks"}}'
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

