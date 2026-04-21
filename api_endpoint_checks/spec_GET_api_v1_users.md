# API Endpoint Check

- Method: `GET`
- Endpoint (Spec): `/api/v1/users`
- Endpoint (Executed): `/api/v1/users`

## 有权限

### curl

```bash
curl -sS -X GET 'http://127.0.0.1:3001/api/v1/users' \
  -H 'Authorization: Bearer <ADMIN_JWT>'
```

### Response

```json
{
  "http_status": 200,
  "response": {
    "users": [
      {
        "id": 1,
        "login": "admin",
        "admin": true,
        "firstname": "Admin",
        "lastname": "User",
        "mail": "",
        "created_on": null,
        "updated_on": null,
        "last_login_on": "2026-03-24T01:50:38.947803848+00:00",
        "status": 1
      },
      {
        "id": 4,
        "login": "jsmith",
        "admin": false,
        "firstname": "John",
        "lastname": "Smith",
        "mail": "",
        "created_on": "2026-03-22T15:10:24+00:00",
        "updated_on": "2026-03-22T15:10:24+00:00",
        "last_login_on": "2026-03-22T19:14:44.770407158+00:00",
        "status": 1
      },
      {
        "id": 5,
        "login": "qa_regular",
        "admin": false,
        "firstname": "NoPerm",
        "lastname": "Regular",
        "mail": "qa_regular@example.com",
        "created_on": "2026-03-24T01:02:36.528225919+00:00",
        "updated_on": "2026-03-24T01:13:44.532905009+00:00",
        "last_login_on": "2026-03-24T01:50:37.713787568+00:00",
        "status": 1
      },
      {
        "id": 6,
        "login": "qa_del_314654",
        "admin": false,
        "firstname": "Qa",
        "lastname": "Del",
        "mail": "qa_del_314654@example.com",
        "created_on": "2026-03-24T01:10:54.982171987+00:00",
        "updated_on": "2026-03-24T01:10:54.982171987+00:00",
        "last_login_on": null,
        "status": 1
      },
      {
        "id": 7,
        "login": "qa_mem_314654",
        "admin": false,
        "firstname": "Qa",
        "lastname": "Mem",
        "mail": "qa_mem_314654@example.com",
        "created_on": "2026-03-24T01:10:55.279389812+00:00",
        "updated_on": "2026-03-24T01:10:55.279389812+00:00",
        "last_login_on": null,
        "status": 1
      },
      {
        "id": 8,
        "login": "qa_del_314657",
        "admin": false,
        "firstname": "Qa",
        "lastname": "Del",
        "mail": "qa_del_314657@example.com",
        "created_on": "2026-03-24T01:10:57.928745824+00:00",
        "updated_on": "2026-03-24T01:10:57.928745824+00:00",
        "last_login_on": null,
        "status": 1
      },
      {
        "id": 9,
        "login": "qa_mem_314657",
        "admin": false,
        "firstname": "Qa",
        "lastname": "Mem",
        "mail": "qa_mem_314657@example.com",
        "created_on": "2026-03-24T01:10:58.218620010+00:00",
        "updated_on": "2026-03-24T01:10:58.218620010+00:00",
        "last_login_on": null,
        "status": 1
      },
      {
        "id": 10,
        "login": "qa_del_314766",
        "admin": false,
        "firstname": "Qa",
        "lastname": "Del",
        "mail": "qa_del_314766@example.com",
        "created_on": "2026-03-24T01:12:47.570239954+00:00",
        "updated_on": "2026-03-24T01:12:47.570239954+00:00",
        "last_login_on": null,
        "status": 1
      },
      {
        "id": 11,
        "login": "qa_mem_314766",
        "admin": false,
        "firstname": "Qa",
        "lastname": "Mem",
        "mail": "qa_mem_314766@example.com",
        "created_on": "2026-03-24T01:12:47.867916667+00:00",
        "updated_on": "2026-03-24T01:12:47.867916667+00:00",
        "last_login_on": null,
        "status": 1
      },
      {
        "id": 13,
        "login": "qa_mem_314822",
        "admin": false,
        "firstname": "Qa",
        "lastname": "Mem",
        "mail": "qa_mem_314822@example.com",
        "created_on": "2026-03-24T01:13:43.405439141+00:00",
        "updated_on": "2026-03-24T01:13:43.405439141+00:00",
        "last_login_on": null,
        "status": 1
      },
      {
        "id": 14,
        "login": "qa_new_314822",
        "admin": false,
        "firstname": "Qa",
        "lastname": "New",
        "mail": "qa_new_314822@example.com",
        "created_on": "2026-03-24T01:13:44.465078880+00:00",
        "updated_on": "2026-03-24T01:13:44.465078880+00:00",
        "last_login_on": null,
        "status": 1
      },
      {
        "id": 15,
        "login": "qa_del_316607",
        "admin": false,
        "firstname": "Qa",
        "lastname": "Del",
        "mail": "qa_del_316607@example.com",
        "created_on": "2026-03-24T01:43:28.232466114+00:00",
        "updated_on": "2026-03-24T01:43:28.232466114+00:00",
        "last_login_on": null,
        "status": 1
      },
      {
        "id": 16,
        "login": "qa_mem_316607",
        "admin": false,
        "firstname": "Qa",
        "lastname": "Mem",
        "mail": "qa_mem_316607@example.com",
        "created_on": "2026-03-24T01:43:28.530339976+00:00",
        "updated_on": "2026-03-24T01:43:28.530339976+00:00",
        "last_login_on": null,
        "status": 1
      },
      {
        "id": 17,
        "login": "qa_del_316960",
        "admin": false,
        "firstname": "Qa",
        "lastname": "Del",
        "mail": "qa_del_316960@example.com",
        "created_on": "2026-03-24T01:49:21.378042790+00:00",
        "updated_on": "2026-03-24T01:49:21.378042790+00:00",
        "last_login_on": null,
        "status": 1
      },
      {
        "id": 18,
        "login": "qa_mem_316960",
        "admin": false,
        "firstname": "Qa",
        "lastname": "Mem",
        "mail": "qa_mem_316960@example.com",
        "created_on": "2026-03-24T01:49:21.670920314+00:00",
        "updated_on": "2026-03-24T01:49:21.670920314+00:00",
        "last_login_on": null,
        "status": 1
      },
      {
        "id": 19,
        "login": "qa_del_317037",
        "admin": false,
        "firstname": "Qa",
        "lastname": "Del",
        "mail": "qa_del_317037@example.com",
        "created_on": "2026-03-24T01:50:37.940795309+00:00",
        "updated_on": "2026-03-24T01:50:37.940795309+00:00",
        "last_login_on": null,
        "status": 1
      },
      {
        "id": 20,
        "login": "qa_mem_317037",
        "admin": false,
        "firstname": "Qa",
        "lastname": "Mem",
        "mail": "qa_mem_317037@example.com",
        "created_on": "2026-03-24T01:50:38.236784679+00:00",
        "updated_on": "2026-03-24T01:50:38.236784679+00:00",
        "last_login_on": null,
        "status": 1
      }
    ],
    "total_count": 17,
    "offset": 0,
    "limit": 25
  }
}
```

## 无权限

### curl

```bash
curl -sS -X GET 'http://127.0.0.1:3001/api/v1/users'
```

### Response

```json
{
  "http_status": 401,
  "response": {}
}
```

