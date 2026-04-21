# API Endpoint Check

- Method: `PUT`
- Endpoint (Spec): `/api/v1/users/:id`
- Endpoint (Executed): `/api/v1/users/1`

## 有权限

### curl

```bash
curl -sS -X PUT 'http://127.0.0.1:3001/api/v1/users/1' \
  -H 'Authorization: Bearer <ADMIN_JWT>' \
  -H 'Content-Type: application/json' \
  -d '{"user": {"firstname": "Admin"}}'
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
      "updated_on": "2026-03-24T01:50:39.507526621+00:00",
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
curl -sS -X PUT 'http://127.0.0.1:3001/api/v1/users/1' \
  -H 'Authorization: Bearer <REGULAR_JWT>' \
  -H 'Content-Type: application/json' \
  -d '{"user": {"firstname": "NoPerm"}}'
```

### Response

```json
{
  "http_status": 403,
  "response": {
    "errors": [
      "You don't have permission to update this user"
    ]
  }
}
```

