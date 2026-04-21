# API Endpoint Check

- Method: `PUT`
- Endpoint (Spec): `/api/v1/projects/:id`
- Endpoint (Executed): `/api/v1/projects/1`

## 有权限

### curl

```bash
curl -sS -X PUT 'http://127.0.0.1:3001/api/v1/projects/1' \
  -H 'Authorization: Bearer <ADMIN_JWT>' \
  -H 'Content-Type: application/json' \
  -d '{"project": {"description": "updated"}}'
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
      "description": "updated",
      "homepage": "https://bytehome.io",
      "status": 1,
      "is_public": false,
      "inherit_members": false,
      "default_assignee": null,
      "created_on": "2026-03-22T15:10:05+00:00",
      "updated_on": "2026-03-24T01:50:39.639407864+00:00"
    }
  }
}
```

## 无权限

### curl

```bash
curl -sS -X PUT 'http://127.0.0.1:3001/api/v1/projects/1' \
  -H 'Authorization: Bearer <REGULAR_JWT>' \
  -H 'Content-Type: application/json' \
  -d '{"project": {"description": "deny"}}'
```

### Response

```json
{
  "http_status": 403,
  "response": {
    "errors": [
      "You don't have permission to edit this project"
    ]
  }
}
```

