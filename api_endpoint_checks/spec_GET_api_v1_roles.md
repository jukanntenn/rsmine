# API Endpoint Check

- Method: `GET`
- Endpoint (Spec): `/api/v1/roles`
- Endpoint (Executed): `/api/v1/roles`

## 有权限

### curl

```bash
curl -sS -X GET 'http://127.0.0.1:3001/api/v1/roles' \
  -H 'Authorization: Bearer <ADMIN_JWT>'
```

### Response

```json
{
  "http_status": 200,
  "response": {
    "roles": [
      {
        "id": 1,
        "name": "Manager"
      },
      {
        "id": 9,
        "name": "QA Role"
      },
      {
        "id": 10,
        "name": "QA Role 314654"
      },
      {
        "id": 11,
        "name": "QA Role Del 314654"
      },
      {
        "id": 12,
        "name": "QA Role 314657"
      },
      {
        "id": 13,
        "name": "QA Role Del 314657"
      },
      {
        "id": 14,
        "name": "QA Role 314766"
      },
      {
        "id": 15,
        "name": "QA Role Del 314766"
      },
      {
        "id": 16,
        "name": "QA Role Upd 314822"
      },
      {
        "id": 18,
        "name": "QA Role New 314822"
      },
      {
        "id": 19,
        "name": "QA Role 317037"
      },
      {
        "id": 20,
        "name": "QA Role Del 317037"
      },
      {
        "id": 2,
        "name": "Developer"
      },
      {
        "id": 3,
        "name": "Reporter"
      }
    ]
  }
}
```

## 无权限

### curl

```bash
curl -sS -X GET 'http://127.0.0.1:3001/api/v1/roles'
```

### Response

```json
{
  "http_status": 401,
  "response": {}
}
```

