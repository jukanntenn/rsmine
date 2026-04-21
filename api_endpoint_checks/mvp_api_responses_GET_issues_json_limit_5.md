# API Endpoint Check

- Method: `GET`
- Endpoint (Spec): `/issues.json?limit=5`
- Endpoint (Executed): `/issues.json?limit=5`

## 有权限

### curl

```bash
curl -sS -X GET 'http://127.0.0.1:3001/issues.json?limit=5' \
  -H 'Authorization: Bearer <ADMIN_JWT>'
```

### Response

```json
{
  "http_status": 404,
  "response": {}
}
```

## 无权限

### curl

```bash
curl -sS -X GET 'http://127.0.0.1:3001/issues.json?limit=5'
```

### Response

```json
{
  "http_status": 404,
  "response": {}
}
```

## 与 MVP_API_RESPONSES.md 差异

- 文档路径为 Redmine 原生风格（无 /api/v1 前缀），当前服务以 /api/v1 为主。
- 文档鉴权为 API Key，当前服务鉴权为 JWT Bearer。

