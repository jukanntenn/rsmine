# API Endpoint Check

- Method: `GET`
- Endpoint (Spec): `/api/v1/issue_statuses/:id`
- Endpoint (Executed): `/api/v1/issue_statuses/21`

## 有权限

### curl

```bash
curl -sS -X GET 'http://127.0.0.1:3001/api/v1/issue_statuses/21' \
  -H 'Authorization: Bearer <ADMIN_JWT>'
```

### Response

```json
{
  "http_status": 200,
  "response": {
    "issue_status": {
      "id": 21,
      "name": "QA Status 317037",
      "is_closed": false,
      "is_default": false,
      "default_done_ratio": null,
      "description": null
    }
  }
}
```

## 无权限

### curl

```bash
curl -sS -X GET 'http://127.0.0.1:3001/api/v1/issue_statuses/21'
```

### Response

```json
{
  "http_status": 401,
  "response": {}
}
```

