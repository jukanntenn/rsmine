# API Endpoint Check

- Method: `POST`
- Endpoint (Spec): `/api/v1/auth/login`
- Endpoint (Executed): `/api/v1/auth/login`

## 有权限

### curl

```bash
curl -sS -X POST 'http://127.0.0.1:3001/api/v1/auth/login' \
  -H 'Content-Type: application/json' \
  -d '{"username": "admin", "password": "admin123"}'
```

### Response

```json
{
  "http_status": 200,
  "response": {
    "token": "eyJ0eXAiOiJKV1QiLCJhbGciOiJIUzI1NiJ9.eyJzdWIiOjEsImV4cCI6MTc3NDQwMzQzOCwiaWF0IjoxNzc0MzE3MDM4fQ.Esc7WHW6rzQTdnsLipVnkZgAl2fxBv5YV0mD1eUf1Vs",
    "user": {
      "id": 1,
      "login": "admin",
      "firstname": "Admin",
      "lastname": "User",
      "admin": true
    }
  }
}
```

## 无权限

### curl

```bash
curl -sS -X POST 'http://127.0.0.1:3001/api/v1/auth/login' \
  -H 'Content-Type: application/json' \
  -d '{"username": "admin", "password": "bad"}'
```

### Response

```json
{
  "http_status": 401,
  "response": {
    "errors": [
      "Invalid username or password"
    ]
  }
}
```

