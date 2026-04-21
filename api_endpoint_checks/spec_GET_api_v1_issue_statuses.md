# API Endpoint Check

- Method: `GET`
- Endpoint (Spec): `/api/v1/issue_statuses`
- Endpoint (Executed): `/api/v1/issue_statuses`

## 有权限

### curl

```bash
curl -sS -X GET 'http://127.0.0.1:3001/api/v1/issue_statuses' \
  -H 'Authorization: Bearer <ADMIN_JWT>'
```

### Response

```json
{
  "http_status": 200,
  "response": {
    "issue_statuses": [
      {
        "id": 1,
        "name": "New",
        "is_closed": false,
        "is_default": false,
        "default_done_ratio": 0,
        "description": null
      },
      {
        "id": 2,
        "name": "In Progress",
        "is_closed": false,
        "is_default": false,
        "default_done_ratio": null,
        "description": null
      },
      {
        "id": 3,
        "name": "Resolved",
        "is_closed": false,
        "is_default": false,
        "default_done_ratio": 100,
        "description": null
      },
      {
        "id": 4,
        "name": "Feedback",
        "is_closed": false,
        "is_default": false,
        "default_done_ratio": null,
        "description": null
      },
      {
        "id": 5,
        "name": "Closed",
        "is_closed": true,
        "is_default": false,
        "default_done_ratio": 100,
        "description": null
      },
      {
        "id": 6,
        "name": "Rejected",
        "is_closed": true,
        "is_default": false,
        "default_done_ratio": 100,
        "description": null
      },
      {
        "id": 11,
        "name": "QA Status",
        "is_closed": false,
        "is_default": false,
        "default_done_ratio": null,
        "description": null
      },
      {
        "id": 12,
        "name": "QA Status 314654",
        "is_closed": false,
        "is_default": false,
        "default_done_ratio": null,
        "description": null
      },
      {
        "id": 13,
        "name": "QA Status Del 314654",
        "is_closed": false,
        "is_default": false,
        "default_done_ratio": null,
        "description": null
      },
      {
        "id": 14,
        "name": "QA Status 314657",
        "is_closed": false,
        "is_default": false,
        "default_done_ratio": null,
        "description": null
      },
      {
        "id": 15,
        "name": "QA Status Del 314657",
        "is_closed": false,
        "is_default": false,
        "default_done_ratio": null,
        "description": null
      },
      {
        "id": 16,
        "name": "QA Status 314766",
        "is_closed": false,
        "is_default": false,
        "default_done_ratio": null,
        "description": null
      },
      {
        "id": 17,
        "name": "QA Status Del 314766",
        "is_closed": false,
        "is_default": false,
        "default_done_ratio": null,
        "description": null
      },
      {
        "id": 18,
        "name": "QA Status Upd 314822",
        "is_closed": false,
        "is_default": false,
        "default_done_ratio": null,
        "description": null
      },
      {
        "id": 20,
        "name": "QA Status New 314822",
        "is_closed": false,
        "is_default": false,
        "default_done_ratio": null,
        "description": null
      },
      {
        "id": 21,
        "name": "QA Status 317037",
        "is_closed": false,
        "is_default": false,
        "default_done_ratio": null,
        "description": null
      },
      {
        "id": 22,
        "name": "QA Status Del 317037",
        "is_closed": false,
        "is_default": false,
        "default_done_ratio": null,
        "description": null
      }
    ]
  }
}
```

## 无权限

### curl

```bash
curl -sS -X GET 'http://127.0.0.1:3001/api/v1/issue_statuses'
```

### Response

```json
{
  "http_status": 401,
  "response": {}
}
```

