# API Endpoint Check

- Method: `GET`
- Endpoint (Spec): `/api/v1/projects/:id/issue_categories`
- Endpoint (Executed): `/api/v1/projects/1/issue_categories`

## 有权限

### curl

```bash
curl -sS -X GET 'http://127.0.0.1:3001/api/v1/projects/1/issue_categories' \
  -H 'Authorization: Bearer <ADMIN_JWT>'
```

### Response

```json
{
  "http_status": 200,
  "response": {
    "issue_categories": [
      {
        "id": 13,
        "name": "Category B Target",
        "project": {
          "id": 1,
          "name": "Updated ByteHome Project"
        }
      },
      {
        "id": 16,
        "name": "Different Project Category",
        "project": {
          "id": 1,
          "name": "Updated ByteHome Project"
        }
      },
      {
        "id": 19,
        "name": "QA Cat 314499",
        "project": {
          "id": 1,
          "name": "Updated ByteHome Project"
        }
      },
      {
        "id": 20,
        "name": "QA Cat 314654",
        "project": {
          "id": 1,
          "name": "Updated ByteHome Project"
        }
      },
      {
        "id": 21,
        "name": "QA Cat 314657",
        "project": {
          "id": 1,
          "name": "Updated ByteHome Project"
        }
      },
      {
        "id": 26,
        "name": "QA Cat 316607",
        "project": {
          "id": 1,
          "name": "Updated ByteHome Project"
        }
      },
      {
        "id": 27,
        "name": "QA Cat 316960",
        "project": {
          "id": 1,
          "name": "Updated ByteHome Project"
        }
      },
      {
        "id": 28,
        "name": "QA Cat 317037",
        "project": {
          "id": 1,
          "name": "Updated ByteHome Project"
        }
      },
      {
        "id": 29,
        "name": "qa-del-cat-317037",
        "project": {
          "id": 1,
          "name": "Updated ByteHome Project"
        }
      }
    ],
    "total_count": 9
  }
}
```

## 无权限

### curl

```bash
curl -sS -X GET 'http://127.0.0.1:3001/api/v1/projects/1/issue_categories'
```

### Response

```json
{
  "http_status": 401,
  "response": {}
}
```

