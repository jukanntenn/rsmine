# Redmine API Real Request/Response Formats

> Date: 2026-03-20
> Redmine Version: 6.1.2
> Base URL: http://192.168.5.50:3498

---

This document records API request and response formats obtained from an actual Redmine system, for reference in Rsmine development.

---

## Authentication Method

All API requests use API Key authentication:

```
X-Redmine-API-Key: 753567f19a12f871f3b608cce2c87557e70b1348
```

---

## 1. Users API

### 1.1 Get User List

**Request:**
```
GET /users.json
```

**Response:** (Empty, may require admin privileges)
```json
{}
```

### 1.2 Get Single User Details

**Request:**
```
GET /users/1.json
```

**Response:**
```json
{
  "user": {
    "id": 1,
    "login": "redmine",
    "admin": true,
    "firstname": "GakuKou",
    "lastname": "You",
    "mail": "yougakukou@foxmail.com",
    "created_on": "2025-11-18T13:25:26Z",
    "updated_on": "2025-11-18T14:03:34Z",
    "last_login_on": "2026-03-18T12:25:48Z",
    "passwd_changed_on": "2025-11-18T13:57:49Z",
    "twofa_scheme": null,
    "api_key": "753567f19a12f871f3b608cce2c87557e70b1348",
    "status": 1
  }
}
```

**Field Descriptions:**
| Field | Type | Description |
|-------|------|-------------|
| id | integer | User ID |
| login | string | Login name |
| admin | boolean | Is administrator |
| firstname | string | First name |
| lastname | string | Last name |
| mail | string | Email address |
| created_on | datetime | Created time (ISO 8601) |
| updated_on | datetime | Updated time (ISO 8601) |
| last_login_on | datetime | Last login time (ISO 8601) |
| passwd_changed_on | datetime | Password changed time (ISO 8601) |
| twofa_scheme | string/null | Two-factor authentication scheme |
| api_key | string | API Key (visible only for self or admin) |
| status | integer | Status: 1=active, 2=registered, 3=locked |

---

## 2. Projects API

### 2.1 Get Project List

**Request:**
```
GET /projects.json
```

**Response:**
```json
{
  "projects": [
    {
      "id": 11,
      "name": "ByteHome",
      "identifier": "bytehome",
      "description": "",
      "homepage": "",
      "status": 1,
      "is_public": false,
      "inherit_members": false,
      "created_on": "2025-11-26T14:08:38Z",
      "updated_on": "2025-11-26T14:08:38Z"
    },
    {
      "id": 12,
      "name": "TrendRadar fork",
      "identifier": "trendradar-fork",
      "description": "",
      "homepage": "https://github.com/jukanntenn/TrendRadar",
      "status": 1,
      "is_public": false,
      "inherit_members": false,
      "created_on": "2025-12-03T13:49:22Z",
      "updated_on": "2025-12-03T13:49:22Z"
    }
  ],
  "total_count": 14,
  "offset": 0,
  "limit": 25
}
```

**Field Descriptions:**
| Field | Type | Description |
|-------|------|-------------|
| id | integer | Project ID |
| name | string | Project name |
| identifier | string | Project identifier |
| description | string | Description |
| homepage | string | Homepage URL |
| status | integer | Status: 1=active, 5=closed, 9=archived |
| is_public | boolean | Is public |
| inherit_members | boolean | Inherit members from parent |
| created_on | datetime | Created time |
| updated_on | datetime | Updated time |
| total_count | integer | Total count (pagination) |
| offset | integer | Offset (pagination) |
| limit | integer | Items per page (pagination) |

### 2.2 Get Single Project Details

**Request:**
```
GET /projects/1.json
```

**Response:**
```json
{
  "project": {
    "id": 1,
    "name": "muci",
    "identifier": "muci",
    "description": "（端）木赐",
    "homepage": "",
    "status": 1,
    "is_public": false,
    "inherit_members": false,
    "default_assignee": {
      "id": 1,
      "name": "GakuKou You"
    },
    "created_on": "2025-11-18T13:53:04Z",
    "updated_on": "2025-11-25T00:40:39Z"
  }
}
```

**Additional Field:**
| Field | Type | Description |
|-------|------|-------------|
| default_assignee | object | Default assignee |

---

## 3. Issues API

### 3.1 Get Issue List

**Request:**
```
GET /issues.json?limit=5
```

**Response:**
```json
{
  "issues": [
    {
      "id": 59,
      "project": {
        "id": 1,
        "name": "muci"
      },
      "tracker": {
        "id": 2,
        "name": "功能"
      },
      "status": {
        "id": 2,
        "name": "进行中",
        "is_closed": false
      },
      "priority": {
        "id": 3,
        "name": "高"
      },
      "author": {
        "id": 1,
        "name": "GakuKou You"
      },
      "assigned_to": {
        "id": 1,
        "name": "GakuKou You"
      },
      "subject": "Use revm to trace swap invocations in transactions from the Arbitrum sequencer",
      "description": "To implement this feature, we need to:\r\n\r\n1. fully understand the feeds from the sequencer.\r\n2. decode all the transactions received from the sequencer. \r\n3. implement a tracer that traces all swap invocations within transactions.",
      "start_date": "2026-02-02",
      "due_date": null,
      "done_ratio": 0,
      "is_private": false,
      "estimated_hours": null,
      "total_estimated_hours": null,
      "spent_hours": 4.0,
      "total_spent_hours": 4.0,
      "created_on": "2026-02-02T03:11:57Z",
      "updated_on": "2026-02-03T01:56:40Z",
      "closed_on": null
    }
  ],
  "total_count": 56,
  "offset": 0,
  "limit": 5
}
```

**Field Descriptions:**
| Field | Type | Description |
|-------|------|-------------|
| id | integer | Issue ID |
| project | object | Project info (id, name) |
| tracker | object | Tracker info (id, name) |
| status | object | Status info (id, name, is_closed) |
| priority | object | Priority info (id, name) |
| author | object | Author info (id, name) |
| assigned_to | object/null | Assignee info |
| subject | string | Subject |
| description | string | Description (\r\n line breaks) |
| start_date | date | Start date (YYYY-MM-DD) |
| due_date | date/null | Due date |
| done_ratio | integer | Completion ratio (0-100) |
| is_private | boolean | Is private |
| estimated_hours | float/null | Estimated hours |
| total_estimated_hours | float/null | Total estimated hours (including subtasks) |
| spent_hours | float | Spent hours |
| total_spent_hours | float | Total spent hours (including subtasks) |
| created_on | datetime | Created time |
| updated_on | datetime | Updated time |
| closed_on | datetime/null | Closed time |

### 3.2 Get Single Issue Details (with Related Data)

**Request:**
```
GET /issues/59.json?include=attachments,journals,relations,children
```

### 3.3 Issue with Attachments and Children Example

**Request:**
```
GET /issues/22.json?include=attachments,journals,relations,children
```

**Response:**
```json
{
  "issue": {
    "id": 22,
    "project": {
      "id": 1,
      "name": "muci"
    },
    "tracker": {
      "id": 2,
      "name": "功能"
    },
    "status": {
      "id": 3,
      "name": "已解决",
      "is_closed": false
    },
    "priority": {
      "id": 2,
      "name": "普通"
    },
    "author": {
      "id": 1,
      "name": "GakuKou You"
    },
    "assigned_to": {
      "id": 1,
      "name": "GakuKou You"
    },
    "subject": "基于 Bellman-Ford 算法检测套利机会",
    "description": "基于 Bellman-Ford 算法检测套利机会。",
    "start_date": "2025-11-25",
    "due_date": null,
    "done_ratio": 100,
    "is_private": false,
    "estimated_hours": null,
    "total_estimated_hours": 0.0,
    "spent_hours": 24.0,
    "total_spent_hours": 26.0,
    "created_on": "2025-11-25T00:37:10Z",
    "updated_on": "2026-03-18T10:29:02Z",
    "closed_on": null,
    "children": [
      {
        "id": 23,
        "tracker": {
          "id": 2,
          "name": "功能"
        },
        "subject": "探索 petgraph 的用法"
      }
    ],
    "attachments": [
      {
        "id": 1,
        "filename": "1024x1024.png",
        "filesize": 16278,
        "content_type": "image/png",
        "description": "",
        "content_url": "http://192.168.5.50:3498/attachments/download/1/1024x1024.png",
        "thumbnail_url": "http://192.168.5.50:3498/attachments/thumbnail/1",
        "author": {
          "id": 1,
          "name": "GakuKou You"
        },
        "created_on": "2026-03-18T10:28:59Z"
      }
    ],
    "journals": [...]
  }
}
```

**Attachment Field Descriptions:**
| Field | Type | Description |
|-------|------|-------------|
| id | integer | Attachment ID |
| filename | string | Original filename |
| filesize | integer | File size (bytes) |
| content_type | string | MIME type |
| description | string | Description |
| content_url | string | Download URL |
| thumbnail_url | string | Thumbnail URL (images) |
| author | object | Uploader (id, name) |
| created_on | datetime | Upload time |

**Children Field Descriptions:**
| Field | Type | Description |
|-------|------|-------------|
| id | integer | Subtask Issue ID |
| tracker | object | Tracker info (id, name) |
| subject | string | Subtask subject |

### 3.4 Issue Details Response (No Attachments/Relations/Children)

**Request:**
```
GET /issues/59.json?include=attachments,journals,relations,children
```

**Response:**
```json
{
  "issue": {
    "id": 59,
    "project": {
      "id": 1,
      "name": "muci"
    },
    "tracker": {
      "id": 2,
      "name": "功能"
    },
    "status": {
      "id": 2,
      "name": "进行中",
      "is_closed": false
    },
    "priority": {
      "id": 3,
      "name": "高"
    },
    "author": {
      "id": 1,
      "name": "GakuKou You"
    },
    "assigned_to": {
      "id": 1,
      "name": "GakuKou You"
    },
    "subject": "Use revm to trace swap invocations in transactions from the Arbitrum sequencer",
    "description": "...",
    "start_date": "2026-02-02",
    "due_date": null,
    "done_ratio": 0,
    "is_private": false,
    "estimated_hours": null,
    "total_estimated_hours": null,
    "spent_hours": 4.0,
    "total_spent_hours": 4.0,
    "created_on": "2026-02-02T03:11:57Z",
    "updated_on": "2026-02-03T01:56:40Z",
    "closed_on": null,
    "attachments": [],
    "journals": [
      {
        "id": 95,
        "user": {
          "id": 1,
          "name": "GakuKou You"
        },
        "notes": "",
        "created_on": "2026-02-02T03:16:02Z",
        "updated_on": "2026-02-02T03:16:02Z",
        "private_notes": false,
        "details": [
          {
            "property": "attr",
            "name": "status_id",
            "old_value": "1",
            "new_value": "2"
          }
        ]
      },
      {
        "id": 96,
        "user": {
          "id": 1,
          "name": "GakuKou You"
        },
        "notes": "",
        "created_on": "2026-02-02T03:16:40Z",
        "updated_on": "2026-02-02T03:16:40Z",
        "private_notes": false,
        "details": [
          {
            "property": "attr",
            "name": "description",
            "old_value": "old description...",
            "new_value": "new description..."
          }
        ]
      },
      {
        "id": 98,
        "user": {
          "id": 1,
          "name": "GakuKou You"
        },
        "notes": "for 1, the feeds fall into three core categories:\r\n- Batch Publication Messages\r\n- Standard L2 Transaction Messages\r\n- Arbitrum-Specific Transaction Messages\r\n\r\nthe batch of messages contains all L2 transactions. This is what we need.\r\n\r\nfor 2, we can utilize the crate [sequencer_client](https://github.com/nuntax/sequencer_client)",
        "created_on": "2026-02-03T01:56:40Z",
        "updated_on": "2026-02-03T01:56:40Z",
        "private_notes": false,
        "details": []
      }
    ]
  }
}
```

**Journal Field Descriptions:**
| Field | Type | Description |
|-------|------|-------------|
| id | integer | Journal ID |
| user | object | Operating user (id, name) |
| notes | string | Notes/comments |
| created_on | datetime | Created time |
| updated_on | datetime | Updated time |
| private_notes | boolean | Is private note |
| details | array | Change details list |

**Journal Detail Field Descriptions:**
| Field | Type | Description |
|-------|------|-------------|
| property | string | Property type: attr/cf/attachment |
| name | string | Field name |
| old_value | string/null | Old value |
| new_value | string/null | New value |

---

## 4. Memberships API

### 4.1 Get Project Member List

**Request:**
```
GET /projects/1/memberships.json
```

**Response:**
```json
{
  "memberships": [
    {
      "id": 1,
      "project": {
        "id": 1,
        "name": "muci"
      },
      "user": {
        "id": 1,
        "name": "GakuKou You"
      },
      "roles": [
        {
          "id": 3,
          "name": "管理人员"
        },
        {
          "id": 4,
          "name": "开发人员"
        }
      ]
    }
  ],
  "total_count": 1,
  "offset": 0,
  "limit": 25
}
```

**Field Descriptions:**
| Field | Type | Description |
|-------|------|-------------|
| id | integer | Membership ID |
| project | object | Project info (id, name) |
| user | object | User info (id, name) |
| roles | array | Role list (id, name) |

---

## 5. Issue Categories API

### 5.1 Get Project Category List

**Request:**
```
GET /projects/1/issue_categories.json
```

**Response:**
```json
{
  "issue_categories": [],
  "total_count": 0
}
```

---

## 6. Issue Relations API

### 6.1 Get Issue Relations List

**Request:**
```
GET /issues/59/relations.json
```

**Response:** (No relations)
```json
{
  "relations": []
}
```

### 6.2 Issue with Relations Example

**Request:**
```
GET /issues/1.json?include=relations
```

**Response:**
```json
{
  "issue": {
    "id": 1,
    "project": {
      "id": 1,
      "name": "muci"
    },
    "tracker": {
      "id": 2,
      "name": "功能"
    },
    "status": {
      "id": 3,
      "name": "已解决",
      "is_closed": false
    },
    "priority": {
      "id": 3,
      "name": "高"
    },
    "author": {
      "id": 1,
      "name": "GakuKou You"
    },
    "assigned_to": {
      "id": 1,
      "name": "GakuKou You"
    },
    "subject": "区块状态同步验证程序开发",
    "description": "开发一个验证程序...",
    "relations": [
      {
        "id": 1,
        "issue_id": 3,
        "issue_to_id": 1,
        "relation_type": "precedes",
        "delay": 0
      }
    ]
  }
}
```

**Relation Field Descriptions:**
| Field | Type | Description |
|-------|------|-------------|
| id | integer | Relation ID |
| issue_id | integer | Source Issue ID |
| issue_to_id | integer | Target Issue ID |
| relation_type | string | Relation type |
| delay | integer | Delay in days (precedes/follows) |

---

## 7. Enumerations API

### 7.1 Get Trackers List

**Request:**
```
GET /trackers.json
```

**Response:**
```json
{
  "trackers": [
    {
      "id": 1,
      "name": "错误",
      "default_status": {
        "id": 1,
        "name": "新建"
      },
      "description": null,
      "enabled_standard_fields": [
        "assigned_to_id",
        "category_id",
        "fixed_version_id",
        "parent_issue_id",
        "start_date",
        "due_date",
        "estimated_hours",
        "done_ratio",
        "description",
        "priority_id"
      ]
    },
    {
      "id": 2,
      "name": "功能",
      "default_status": {
        "id": 1,
        "name": "新建"
      },
      "description": null,
      "enabled_standard_fields": [
        "assigned_to_id",
        "category_id",
        "fixed_version_id",
        "parent_issue_id",
        "start_date",
        "due_date",
        "estimated_hours",
        "done_ratio",
        "description",
        "priority_id"
      ]
    },
    {
      "id": 3,
      "name": "支持",
      "default_status": {
        "id": 1,
        "name": "新建"
      },
      "description": null,
      "enabled_standard_fields": [
        "assigned_to_id",
        "category_id",
        "fixed_version_id",
        "parent_issue_id",
        "start_date",
        "due_date",
        "estimated_hours",
        "done_ratio",
        "description",
        "priority_id"
      ]
    }
  ]
}
```

### 7.2 Get Issue Statuses List

**Request:**
```
GET /issue_statuses.json
```

**Response:**
```json
{
  "issue_statuses": [
    {
      "id": 1,
      "name": "新建",
      "is_closed": false,
      "description": null
    },
    {
      "id": 2,
      "name": "进行中",
      "is_closed": false,
      "description": null
    },
    {
      "id": 3,
      "name": "已解决",
      "is_closed": false,
      "description": null
    },
    {
      "id": 4,
      "name": "反馈",
      "is_closed": false,
      "description": null
    },
    {
      "id": 5,
      "name": "已关闭",
      "is_closed": true,
      "description": null
    },
    {
      "id": 6,
      "name": "已拒绝",
      "is_closed": true,
      "description": null
    }
  ]
}
```

### 7.3 Get Issue Priorities List

**Request:**
```
GET /enumerations/issue_priorities.json
```

**Response:**
```json
{
  "issue_priorities": [
    {
      "id": 1,
      "name": "低",
      "is_default": false,
      "active": true
    },
    {
      "id": 2,
      "name": "普通",
      "is_default": true,
      "active": true
    },
    {
      "id": 3,
      "name": "高",
      "is_default": false,
      "active": true
    },
    {
      "id": 4,
      "name": "紧急",
      "is_default": false,
      "active": true
    },
    {
      "id": 5,
      "name": "立刻",
      "is_default": false,
      "active": true
    }
  ]
}
```

### 7.4 Get Roles List

**Request:**
```
GET /roles.json
```

**Response:**
```json
{
  "roles": [
    {
      "id": 3,
      "name": "管理人员"
    },
    {
      "id": 4,
      "name": "开发人员"
    },
    {
      "id": 5,
      "name": "报告人员"
    }
  ]
}
```

---

## 8. APIs with No Data Available

The following API endpoints could not retrieve data in the current test environment:

| Endpoint | Status | Reason |
|----------|--------|--------|
| `/projects/:id/trackers.json` | 404 | Redmine does not support this API endpoint, use `GET /trackers.json` or `GET /projects/:id.json?include=trackers` |
| `/users.json` (list) | Empty response | Requires API Key with admin privileges |

### 8.1 How to Get Admin API Key

1. Log in to Redmine with an administrator account
2. Go to **My Account** page
3. Find **API access key** section on the right side
4. Click **Show** to view or **Create** to generate a new API Key

**Note:** Administrator user's API Key has `admin` privileges and can access all API endpoints. The current API Key belongs to a user who is an administrator (`"admin": true`), but the `/users.json` endpoint may require additional permission configuration.

### 8.2 User API Alternatives

If unable to get user list, you can use:

```
GET /users/current.json  # Get current logged-in user info
GET /projects/:id/memberships.json  # Get project member list
```

---

## 9. Key Findings

### 9.1 Date/Time Format
- All datetime values use ISO 8601 format: `YYYY-MM-DDTHH:MM:SSZ`
- Date fields use `YYYY-MM-DD` format

### 9.2 Nested Objects
- `project`, `tracker`, `status`, `priority`, `author`, `assigned_to` in Issue response are nested objects
- Only contain `id` and `name` fields

### 9.3 Description Field Line Breaks
- `description` field uses `\r\n` as line break

### 9.4 Pagination Response Format
- List responses include `total_count`, `offset`, `limit` fields
- Default `limit` is 25

### 9.5 Journal Details
- `property` field indicates change type
- `attr`: attribute change
- `cf`: custom field change
- `attachment`: attachment change

---

_End of Document_