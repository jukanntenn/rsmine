# API Endpoint Check

- Method: `POST`
- Endpoint (Spec): `/api/v1/users`
- Endpoint (Executed): `/api/v1/users`

## 有权限

### curl

```bash
curl -sS -X POST 'http://127.0.0.1:3001/api/v1/users' \
  -H 'Authorization: Bearer <ADMIN_JWT>' \
  -H 'Content-Type: application/json' \
  -d '{"user": {"login": "qa_new_317037", "firstname": "Qa", "lastname": "New", "mail": "qa_new_317037@example.com", "password": "admin123", "password_confirmation": "admin123"}}'
```

### Response

```json
{
  "http_status": 201,
  "response": {
    "user": {
      "id": 21,
      "login": "qa_new_317037",
      "admin": false,
      "firstname": "Qa",
      "lastname": "New",
      "mail": "qa_new_317037@example.com",
      "created_on": "2026-03-24T01:50:39.449034998+00:00",
      "updated_on": "2026-03-24T01:50:39.449034998+00:00",
      "last_login_on": null,
      "passwd_changed_on": "2026-03-24T01:50:39.449034998+00:00",
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
curl -sS -X POST 'http://127.0.0.1:3001/api/v1/users' \
  -H 'Authorization: Bearer <REGULAR_JWT>' \
  -H 'Content-Type: application/json' \
  -d '{"user": {"login": "qa_den_317037", "firstname": "Qa", "lastname": "Denied", "mail": "qa_den_317037@example.com", "password": "admin123", "password_confirmation": "admin123"}}'
```

### Response

```json
{
  "http_status": 403,
  "response": {
    "errors": [
      "Only administrators can create users"
    ]
  }
}
```

