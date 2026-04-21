# API Endpoint Check

- Method: `GET`
- Endpoint (Spec): `/api/v1/projects/:id/trackers`
- Endpoint (Executed): `/api/v1/projects/1/trackers`

## 有权限

### curl

```bash
curl -sS -X GET 'http://127.0.0.1:3001/api/v1/projects/1/trackers' \
  -H 'Authorization: Bearer <ADMIN_JWT>'
```

### Response

```json
{
  "http_status": 200,
  "response": {
    "trackers": []
  }
}
```

## 无权限

### curl

```bash
curl -sS -X GET 'http://127.0.0.1:3001/api/v1/projects/1/trackers'
```

### Response

```json
{
  "http_status": 401,
  "response": {}
}
```

