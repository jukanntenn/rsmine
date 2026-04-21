# API Endpoint Check

- Method: `GET`
- Endpoint (Spec): `/api/v1/users/:id`
- Endpoint (Executed): `/api/v1/users/1`

## 有权限

### curl

```bash
curl -sS -X GET 'http://127.0.0.1:3001/api/v1/users/1' \
  -H 'Authorization: Bearer <ADMIN_JWT>'
```

### Response

```json
{
  "http_status": 200,
  "response": {
    "user": {
      "id": 1,
      "login": "admin",
      "admin": true,
      "firstname": "Admin",
      "lastname": "User",
      "mail": "",
      "created_on": null,
      "updated_on": null,
      "last_login_on": "2026-03-24T01:50:38.947803848+00:00",
      "passwd_changed_on": null,
      "twofa_scheme": null,
      "api_key": null,
      "status": 1
    }
  }
}
```

## 无权限

### curl

```bash
curl -sS -X GET 'http://127.0.0.1:3001/api/v1/users/1'
```

### Response

```json
{
  "http_status": 401,
  "response": {}
}
```

