# API Endpoint Check

- Method: `GET`
- Endpoint (Spec): `/api/v1/projects/:id`
- Endpoint (Executed): `/api/v1/projects/1`

## 有权限

### curl

```bash
curl -sS -X GET 'http://127.0.0.1:3001/api/v1/projects/1' \
  -H 'Authorization: Bearer <ADMIN_JWT>'
```

### Response

```json
{
  "http_status": 200,
  "response": {
    "project": {
      "id": 1,
      "name": "Updated ByteHome Project",
      "identifier": "updated-bytehome",
      "description": "Home automation project",
      "homepage": "https://bytehome.io",
      "status": 1,
      "is_public": false,
      "inherit_members": false,
      "default_assignee": null,
      "created_on": "2026-03-22T15:10:05+00:00",
      "updated_on": "2026-03-22T16:09:40.721475538+00:00"
    }
  }
}
```

## 无权限

### curl

```bash
curl -sS -X GET 'http://127.0.0.1:3001/api/v1/projects/1'
```

### Response

```json
{
  "http_status": 401,
  "response": {}
}
```

