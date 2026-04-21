# API Endpoint Check

- Method: `GET`
- Endpoint (Spec): `/api/v1/enumerations/issue_priorities`
- Endpoint (Executed): `/api/v1/enumerations/issue_priorities`

## 有权限

### curl

```bash
curl -sS -X GET 'http://127.0.0.1:3001/api/v1/enumerations/issue_priorities' \
  -H 'Authorization: Bearer <ADMIN_JWT>'
```

### Response

```json
{
  "http_status": 200,
  "response": {
    "issue_priorities": [
      {
        "id": 1,
        "name": "Low",
        "is_default": false,
        "active": true
      },
      {
        "id": 2,
        "name": "Normal",
        "is_default": true,
        "active": true
      },
      {
        "id": 3,
        "name": "High",
        "is_default": false,
        "active": true
      },
      {
        "id": 4,
        "name": "Urgent",
        "is_default": false,
        "active": true
      },
      {
        "id": 5,
        "name": "Immediate",
        "is_default": false,
        "active": true
      }
    ]
  }
}
```

## 无权限

### curl

```bash
curl -sS -X GET 'http://127.0.0.1:3001/api/v1/enumerations/issue_priorities'
```

### Response

```json
{
  "http_status": 401,
  "response": {}
}
```

