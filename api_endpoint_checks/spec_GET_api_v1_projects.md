# API Endpoint Check

- Method: `GET`
- Endpoint (Spec): `/api/v1/projects`
- Endpoint (Executed): `/api/v1/projects`

## 有权限

### curl

```bash
curl -sS -X GET 'http://127.0.0.1:3001/api/v1/projects' \
  -H 'Authorization: Bearer <ADMIN_JWT>'
```

### Response

```json
{
  "http_status": 200,
  "response": {
    "projects": [
      {
        "id": 1,
        "name": "Updated ByteHome Project",
        "identifier": "updated-bytehome",
        "description": "Home automation project",
        "homepage": "https://bytehome.io",
        "status": 1,
        "is_public": false,
        "inherit_members": false,
        "created_on": "2026-03-22T15:10:05+00:00",
        "updated_on": "2026-03-22T16:09:40.721475538+00:00"
      },
      {
        "id": 2,
        "name": "TrendRadar",
        "identifier": "trendradar",
        "description": "Trend analysis tool",
        "homepage": "",
        "status": 5,
        "is_public": true,
        "inherit_members": false,
        "created_on": "2026-03-22T15:10:05+00:00",
        "updated_on": "2026-03-22T16:09:30.146778168+00:00"
      },
      {
        "id": 3,
        "name": "PrivateProject",
        "identifier": "private-project",
        "description": "Internal project",
        "homepage": "",
        "status": 1,
        "is_public": false,
        "inherit_members": false,
        "created_on": "2026-03-22T15:10:05+00:00",
        "updated_on": "2026-03-22T15:10:05+00:00"
      },
      {
        "id": 4,
        "name": "ClosedProject",
        "identifier": "closed-project",
        "description": "Archived project",
        "homepage": "",
        "status": 5,
        "is_public": true,
        "inherit_members": false,
        "created_on": "2026-03-22T15:10:05+00:00",
        "updated_on": "2026-03-22T15:10:05+00:00"
      },
      {
        "id": 5,
        "name": "Updated by Member",
        "identifier": "test-project-alpha",
        "description": "New description by member",
        "homepage": "",
        "status": 1,
        "is_public": true,
        "inherit_members": false,
        "created_on": "2026-03-22T15:47:04.724465685+00:00",
        "updated_on": "2026-03-22T16:10:21.110287379+00:00"
      },
      {
        "id": 6,
        "name": "Subproject Alpha",
        "identifier": "subproject-alpha",
        "description": "A subproject under Test Project Alpha",
        "homepage": "",
        "status": 1,
        "is_public": true,
        "inherit_members": false,
        "created_on": "2026-03-22T15:47:39.419948457+00:00",
        "updated_on": "2026-03-22T16:09:40.749000631+00:00"
      },
      {
        "id": 7,
        "name": "Project With Trackers",
        "identifier": "project-with-trackers",
        "description": "",
        "homepage": "",
        "status": 1,
        "is_public": true,
        "inherit_members": false,
        "created_on": "2026-03-22T15:47:39.462268910+00:00",
        "updated_on": "2026-03-22T15:47:39.462268910+00:00"
      },
      {
        "id": 8,
        "name": "Project With Homepage",
        "identifier": "project-homepage",
        "description": "",
        "homepage": "https://example.com/project",
        "status": 1,
        "is_public": true,
        "inherit_members": false,
        "created_on": "2026-03-22T15:47:39.558541925+00:00",
        "updated_on": "2026-03-22T15:47:39.558541925+00:00"
      },
      {
        "id": 9,
        "name": "Private Project Test",
        "identifier": "private-project-test-2",
        "description": "",
        "homepage": "",
        "status": 1,
        "is_public": false,
        "inherit_members": false,
        "created_on": "2026-03-22T15:48:03.196106888+00:00",
        "updated_on": "2026-03-22T15:48:03.196106888+00:00"
      },
      {
        "id": 10,
        "name": "Integration Test Updated",
        "identifier": "new-test-identifier",
        "description": "A test project for integration testing",
        "homepage": "",
        "status": 5,
        "is_public": false,
        "inherit_members": false,
        "created_on": "2026-03-22T15:51:48.499501172+00:00",
        "updated_on": "2026-03-22T16:13:37.276786992+00:00"
      },
      {
        "id": 11,
        "name": "Subproject Test",
        "identifier": "subproject-test",
        "description": "",
        "homepage": "",
        "status": 1,
        "is_public": true,
        "inherit_members": true,
        "created_on": "2026-03-22T15:52:12.819565644+00:00",
        "updated_on": "2026-03-22T15:52:12.819565644+00:00"
      },
      {
        "id": 12,
        "name": "Updated Project Name",
        "identifier": "new-identifier",
        "description": "This is an updated description",
        "homepage": "",
        "status": 5,
        "is_public": true,
        "inherit_members": false,
        "created_on": "2026-03-22T16:04:06.531617409+00:00",
        "updated_on": "2026-03-22T16:04:18.211929768+00:00"
      },
      {
        "id": 13,
        "name": "Test Member Delete",
        "identifier": "test-member-delete-api",
        "description": "",
        "homepage": "",
        "status": 1,
        "is_public": true,
        "inherit_members": false,
        "created_on": "2026-03-22T18:02:05.029179792+00:00",
        "updated_on": "2026-03-22T18:02:05.029179792+00:00"
      },
      {
        "id": 14,
        "name": "Test Member Delete API",
        "identifier": "test-member-delete-api-2",
        "description": "",
        "homepage": "",
        "status": 1,
        "is_public": true,
        "inherit_members": false,
        "created_on": "2026-03-22T18:05:00.983341549+00:00",
        "updated_on": "2026-03-22T18:05:00.983341549+00:00"
      },
      {
        "id": 16,
        "name": "Test Project for Issue",
        "identifier": "test-issue-project",
        "description": "",
        "homepage": "",
        "status": 1,
        "is_public": true,
        "inherit_members": false,
        "created_on": "2026-03-22T20:44:58.611871919+00:00",
        "updated_on": "2026-03-22T20:44:58.611871919+00:00"
      },
      {
        "id": 17,
        "name": "Test",
        "identifier": "test-proj",
        "description": "",
        "homepage": "",
        "status": 1,
        "is_public": true,
        "inherit_members": false,
        "created_on": "2026-03-22T20:46:51.245593873+00:00",
        "updated_on": "2026-03-22T20:46:51.245593873+00:00"
      },
      {
        "id": 18,
        "name": "Test Project",
        "identifier": "test-project",
        "description": "",
        "homepage": "",
        "status": 1,
        "is_public": true,
        "inherit_members": false,
        "created_on": "2026-03-22T23:18:15.286652905+00:00",
        "updated_on": "2026-03-22T23:18:15.286652905+00:00"
      },
      {
        "id": 19,
        "name": "E2E Project 1774281389248-9260",
        "identifier": "e2e-project-1774281389248-9260",
        "description": "",
        "homepage": "",
        "status": 1,
        "is_public": true,
        "inherit_members": false,
        "created_on": "2026-03-23T15:56:29.843481401+00:00",
        "updated_on": "2026-03-23T15:56:29.843481401+00:00"
      },
      {
        "id": 20,
        "name": "Parent 1774281389240-4561",
        "identifier": "parent-1774281389240-4561",
        "description": "",
        "homepage": "",
        "status": 1,
        "is_public": true,
        "inherit_members": false,
        "created_on": "2026-03-23T15:56:29.894514108+00:00",
        "updated_on": "2026-03-23T15:56:29.894514108+00:00"
      },
      {
        "id": 21,
        "name": "Project Edit 1774281389249-3158",
        "identifier": "project-edit-1774281389249-3158",
        "description": "",
        "homepage": "",
        "status": 1,
        "is_public": true,
        "inherit_members": false,
        "created_on": "2026-03-23T15:56:29.968290444+00:00",
        "updated_on": "2026-03-23T15:56:29.968290444+00:00"
      },
      {
        "id": 22,
        "name": "Visibility 1774281389712-663",
        "identifier": "visibility-1774281389712-663",
        "description": "",
        "homepage": "",
        "status": 1,
        "is_public": true,
        "inherit_members": false,
        "created_on": "2026-03-23T15:56:30.113185227+00:00",
        "updated_on": "2026-03-23T15:56:30.113185227+00:00"
      },
      {
        "id": 23,
        "name": "Project Delete 1774281389242-4895",
        "identifier": "project-delete-1774281389242-4895",
        "description": "",
        "homepage": "",
        "status": 1,
        "is_public": true,
        "inherit_members": false,
        "created_on": "2026-03-23T15:56:30.166210557+00:00",
        "updated_on": "2026-03-23T15:56:30.166210557+00:00"
      },
      {
        "id": 24,
        "name": "Project Delete 1774281458565-445",
        "identifier": "project-delete-1774281458565-445",
        "description": "",
        "homepage": "",
        "status": 1,
        "is_public": true,
        "inherit_members": false,
        "created_on": "2026-03-23T15:57:39.129284157+00:00",
        "updated_on": "2026-03-23T15:57:39.129284157+00:00"
      },
      {
        "id": 25,
        "name": "Visibility 1774281458565-8583",
        "identifier": "visibility-1774281458565-8583",
        "description": "",
        "homepage": "",
        "status": 1,
        "is_public": true,
        "inherit_members": false,
        "created_on": "2026-03-23T15:57:39.274247469+00:00",
        "updated_on": "2026-03-23T15:57:39.274247469+00:00"
      },
      {
        "id": 26,
        "name": "Parent 1774281458565-291",
        "identifier": "parent-1774281458565-291",
        "description": "",
        "homepage": "",
        "status": 1,
        "is_public": true,
        "inherit_members": false,
        "created_on": "2026-03-23T15:57:39.325413003+00:00",
        "updated_on": "2026-03-23T15:57:39.325413003+00:00"
      }
    ],
    "total_count": 85,
    "offset": 0,
    "limit": 25
  }
}
```

## 无权限

### curl

```bash
curl -sS -X GET 'http://127.0.0.1:3001/api/v1/projects'
```

### Response

```json
{
  "http_status": 401,
  "response": {}
}
```

