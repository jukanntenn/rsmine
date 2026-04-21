# API Endpoint Check

- Method: `GET`
- Endpoint (Spec): `/api/v1/projects/:id/memberships`
- Endpoint (Executed): `/api/v1/projects/1/memberships`

## 有权限

### curl

```bash
curl -sS -X GET 'http://127.0.0.1:3001/api/v1/projects/1/memberships' \
  -H 'Authorization: Bearer <ADMIN_JWT>'
```

### Response

```json
{
  "http_status": 200,
  "response": {
    "memberships": [
      {
        "id": 101,
        "project": {
          "id": 1,
          "name": "Updated ByteHome Project"
        },
        "user": {
          "id": 4,
          "name": "John Smith"
        },
        "roles": [
          {
            "id": 3,
            "name": "Reporter"
          },
          {
            "id": 4,
            "name": "Non-Member"
          }
        ]
      },
      {
        "id": 175,
        "project": {
          "id": 1,
          "name": "Updated ByteHome Project"
        },
        "user": {
          "id": 7,
          "name": "Qa Mem"
        },
        "roles": [
          {
            "id": 2,
            "name": "Developer"
          }
        ]
      },
      {
        "id": 177,
        "project": {
          "id": 1,
          "name": "Updated ByteHome Project"
        },
        "user": {
          "id": 9,
          "name": "Qa Mem"
        },
        "roles": [
          {
            "id": 2,
            "name": "Developer"
          }
        ]
      },
      {
        "id": 187,
        "project": {
          "id": 1,
          "name": "Updated ByteHome Project"
        },
        "user": {
          "id": 16,
          "name": "Qa Mem"
        },
        "roles": [
          {
            "id": 2,
            "name": "Developer"
          }
        ]
      },
      {
        "id": 189,
        "project": {
          "id": 1,
          "name": "Updated ByteHome Project"
        },
        "user": {
          "id": 18,
          "name": "Qa Mem"
        },
        "roles": [
          {
            "id": 2,
            "name": "Developer"
          }
        ]
      },
      {
        "id": 191,
        "project": {
          "id": 1,
          "name": "Updated ByteHome Project"
        },
        "user": {
          "id": 20,
          "name": "Qa Mem"
        },
        "roles": [
          {
            "id": 2,
            "name": "Developer"
          }
        ]
      }
    ],
    "total_count": 6,
    "offset": 0,
    "limit": 25
  }
}
```

## 无权限

### curl

```bash
curl -sS -X GET 'http://127.0.0.1:3001/api/v1/projects/1/memberships'
```

### Response

```json
{
  "http_status": 401,
  "response": {}
}
```

