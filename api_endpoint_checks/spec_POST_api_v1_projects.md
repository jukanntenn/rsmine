# API Endpoint Check

- Method: `POST`
- Endpoint (Spec): `/api/v1/projects`
- Endpoint (Executed): `/api/v1/projects`

## 有权限

### curl

```bash
curl -sS -X POST 'http://127.0.0.1:3001/api/v1/projects' \
  -H 'Authorization: Bearer <ADMIN_JWT>' \
  -H 'Content-Type: application/json' \
  -d '{"project": {"name": "QA P 317037", "identifier": "qa-p-317037", "is_public": true}}'
```

### Response

```json
{
  "http_status": 201,
  "response": {
    "project": {
      "id": 92,
      "name": "QA P 317037",
      "identifier": "qa-p-317037",
      "description": "",
      "homepage": "",
      "status": 1,
      "is_public": true,
      "inherit_members": false,
      "default_assignee": null,
      "created_on": "2026-03-24T01:50:39.580551650+00:00",
      "updated_on": "2026-03-24T01:50:39.580551650+00:00"
    }
  }
}
```

## 无权限

### curl

```bash
curl -sS -X POST 'http://127.0.0.1:3001/api/v1/projects' \
  -H 'Authorization: Bearer <REGULAR_JWT>' \
  -H 'Content-Type: application/json' \
  -d '{"project": {"name": "QA NP 317037", "identifier": "qa-np-317037", "is_public": true}}'
```

### Response

```json
{
  "http_status": 403,
  "response": {
    "errors": [
      "You don't have permission to create projects"
    ]
  }
}
```

