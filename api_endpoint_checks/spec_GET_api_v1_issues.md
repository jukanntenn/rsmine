# API Endpoint Check

- Method: `GET`
- Endpoint (Spec): `/api/v1/issues`
- Endpoint (Executed): `/api/v1/issues?limit=5`

## 有权限

### curl

```bash
curl -sS -X GET 'http://127.0.0.1:3001/api/v1/issues?limit=5' \
  -H 'Authorization: Bearer <ADMIN_JWT>'
```

### Response

```json
{
  "http_status": 200,
  "response": {
    "issues": [
      {
        "id": 59,
        "project": {
          "id": 86,
          "name": "QA Base 314822"
        },
        "tracker": {
          "id": 2,
          "name": "Feature"
        },
        "status": {
          "id": 1,
          "name": "New",
          "is_closed": false
        },
        "priority": {
          "id": 2,
          "name": "Normal"
        },
        "author": {
          "id": 5,
          "name": "NoPerm Regular"
        },
        "assigned_to": null,
        "subject": "qa-deny-314822",
        "description": "",
        "start_date": null,
        "due_date": null,
        "done_ratio": 0,
        "is_private": false,
        "estimated_hours": null,
        "total_estimated_hours": null,
        "spent_hours": 0.0,
        "total_spent_hours": 0.0,
        "created_on": "2026-03-24T01:13:45.045988539+00:00",
        "updated_on": "2026-03-24T01:13:45.045988539+00:00",
        "closed_on": null
      },
      {
        "id": 58,
        "project": {
          "id": 86,
          "name": "QA Base 314822"
        },
        "tracker": {
          "id": 2,
          "name": "Feature"
        },
        "status": {
          "id": 1,
          "name": "New",
          "is_closed": false
        },
        "priority": {
          "id": 2,
          "name": "Normal"
        },
        "author": {
          "id": 1,
          "name": "Admin User"
        },
        "assigned_to": null,
        "subject": "qa-create-314822",
        "description": "",
        "start_date": null,
        "due_date": null,
        "done_ratio": 0,
        "is_private": false,
        "estimated_hours": null,
        "total_estimated_hours": null,
        "spent_hours": 0.0,
        "total_spent_hours": 0.0,
        "created_on": "2026-03-24T01:13:45.019440496+00:00",
        "updated_on": "2026-03-24T01:13:45.019440496+00:00",
        "closed_on": null
      },
      {
        "id": 56,
        "project": {
          "id": 86,
          "name": "QA Base 314822"
        },
        "tracker": {
          "id": 2,
          "name": "Feature"
        },
        "status": {
          "id": 1,
          "name": "New",
          "is_closed": false
        },
        "priority": {
          "id": 2,
          "name": "Normal"
        },
        "author": {
          "id": 1,
          "name": "Admin User"
        },
        "assigned_to": null,
        "subject": "QA B 314822",
        "description": "B",
        "start_date": null,
        "due_date": null,
        "done_ratio": 0,
        "is_private": false,
        "estimated_hours": null,
        "total_estimated_hours": null,
        "spent_hours": 0.0,
        "total_spent_hours": 0.0,
        "created_on": "2026-03-24T01:13:43.525265349+00:00",
        "updated_on": "2026-03-24T01:13:43.525265349+00:00",
        "closed_on": null
      },
      {
        "id": 55,
        "project": {
          "id": 86,
          "name": "QA Base 314822"
        },
        "tracker": {
          "id": 2,
          "name": "Feature"
        },
        "status": {
          "id": 1,
          "name": "New",
          "is_closed": false
        },
        "priority": {
          "id": 2,
          "name": "Normal"
        },
        "author": {
          "id": 1,
          "name": "Admin User"
        },
        "assigned_to": {
          "id": 5,
          "name": "NoPerm Regular"
        },
        "subject": "QA A 314822",
        "description": "A",
        "start_date": null,
        "due_date": null,
        "done_ratio": 40,
        "is_private": false,
        "estimated_hours": null,
        "total_estimated_hours": null,
        "spent_hours": 0.0,
        "total_spent_hours": 0.0,
        "created_on": "2026-03-24T01:13:43.503316424+00:00",
        "updated_on": "2026-03-24T01:13:45.130311849+00:00",
        "closed_on": null
      },
      {
        "id": 54,
        "project": {
          "id": 84,
          "name": "QA Base 314766"
        },
        "tracker": {
          "id": 2,
          "name": "Feature"
        },
        "status": {
          "id": 1,
          "name": "New",
          "is_closed": false
        },
        "priority": {
          "id": 2,
          "name": "Normal"
        },
        "author": {
          "id": 5,
          "name": "NoPerm Regular"
        },
        "assigned_to": null,
        "subject": "qa-deny-314766",
        "description": "",
        "start_date": null,
        "due_date": null,
        "done_ratio": 0,
        "is_private": false,
        "estimated_hours": null,
        "total_estimated_hours": null,
        "spent_hours": 0.0,
        "total_spent_hours": 0.0,
        "created_on": "2026-03-24T01:12:48.935624966+00:00",
        "updated_on": "2026-03-24T01:12:48.935624966+00:00",
        "closed_on": null
      }
    ],
    "total_count": 48,
    "offset": 0,
    "limit": 5
  }
}
```

## 无权限

### curl

```bash
curl -sS -X GET 'http://127.0.0.1:3001/api/v1/issues?limit=5'
```

### Response

```json
{
  "http_status": 401,
  "response": {}
}
```

