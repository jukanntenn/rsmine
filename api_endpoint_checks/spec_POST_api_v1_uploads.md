# API Endpoint Check

- Method: `POST`
- Endpoint (Spec): `/api/v1/uploads`
- Endpoint (Executed): `/api/v1/uploads`

## 有权限

### curl

```bash
curl -sS -X POST 'http://127.0.0.1:3001/api/v1/uploads' \
  -H 'Authorization: Bearer <ADMIN_JWT>' \
  -F 'file=@/tmp/rsmine_api_test.txt' \
  -F 'filename=rsmine_api_test.txt' \
  -F 'content_type=text/plain'
```

### Response

```json
{
  "http_status": 201,
  "response": {
    "upload": {
      "token": "510e795fd0b44c848aa2946c73f651ae"
    }
  }
}
```

## 无权限

### curl

```bash
curl -sS -X POST 'http://127.0.0.1:3001/api/v1/uploads' \
  -F 'file=@/tmp/rsmine_api_test.txt' \
  -F 'filename=rsmine_api_test.txt' \
  -F 'content_type=text/plain'
```

### Response

```json
{
  "http_status": 401,
  "response": {}
}
```

