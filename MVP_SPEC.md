# Rsmine Product Specification Document

> Version: 1.1.0
> Date: 2026-03-20
> Status: Draft

---

## Table of Contents

1. [Project Overview](#1-project-overview)
2. [Technology Stack](#2-technology-stack)
3. [Project Structure](#3-project-structure)
4. [Database Design](#4-database-design)
5. [API Design](#5-api-design)
6. [Permission System](#6-permission-system)
7. [Initial Data](#7-initial-data)
8. [Frontend Design](#8-frontend-design)
9. [Backend Design Decisions](#9-backend-design-decisions)
10. [Security Policy](#10-security-policy)
11. [Testing Strategy](#11-testing-strategy)
12. [Technical Implementation Details](#12-technical-implementation-details)
13. [Deployment and Configuration](#13-deployment-and-configuration)

---

## 1. Project Overview

### 1.1 Product Positioning

Rsmine is a modern project management and issue tracking system that provides:

- Project management (support for public/private projects, project nesting)
- Issue tracking (Issue management, subtasks, relationships)
- Team collaboration (member management, role permissions)
- Attachment management

### 1.2 MVP Feature Scope

| Feature Module        | MVP Includes                           |
| --------------------- | -------------------------------------- |
| User Authentication   | ✅ Login/Logout/Token Management       |
| User Management       | ✅ CRUD                                |
| Project Management    | ✅ CRUD (Public/Private/Nested)        |
| Member Management     | ✅ Project members and role assignment |
| Issue Management      | ✅ CRUD + Subtasks                     |
| Issue Relations       | ✅ Relationship management             |
| Attachment Management | ✅ Upload/Download/Delete              |
| Notes System          | ✅ Issue notes + Change history        |
| Permission Control    | ✅ Fine-grained role-based permissions |
| CLI Tool              | ⏸️ Future iteration                    |

### 1.3 Core Design Principles

1. **Table structure fully aligned with Redmine** - No new tables, no field modifications. All database tables and fields are identical to Redmine's schema for seamless data migration.
2. **RESTful API** - Follow REST standards, versioned interfaces
3. **Onion Architecture** - Backend uses layered architecture, dependencies flow from outside to inside
4. **Multi-database Support** - PostgreSQL / MySQL / SQLite
5. **Progressive Enhancement** - MVP is a subset, future features can be added seamlessly

---

## 2. Technology Stack

### 2.1 Backend Technology Stack

| Category           | Technology         | Version     |
| ------------------ | ------------------ | ----------- |
| Language           | Rust               | 1.94.0      |
| Web Framework      | Axum               | 0.8.8       |
| Middleware         | Tower              | 0.5.3       |
| Middleware         | Tower-HTTP         | 0.6.8       |
| Middleware         | Tower-Cookies      | 0.11.0      |
| Async Runtime      | Tokio              | 1.50.0      |
| ORM                | SeaORM             | 2.0.0-rc.37 |
| Database Migration | SeaORM Migration   | 2.0.0-rc.37 |
| Serialization      | Serde              | 1.0.228     |
| Serialization      | Serde JSON         | 1.0.149     |
| Authentication     | Argon2             | 0.5.3       |
| Authentication     | JWT (jsonwebtoken) | 10.3.0      |
| Authentication     | UUID               | 1.22.0      |
| Logging            | Tracing            | 0.1.44      |
| Logging            | Tracing-Subscriber | 0.3.23      |
| Configuration      | dotenvy            | 0.15.7      |
| Configuration      | config             | 0.15.22     |
| Error Handling     | Thiserror          | 2.0.18      |
| API Documentation  | Utoipa             | 5.4.0       |
| API Documentation  | Utoipa-Swagger-UI  | 9.0.2       |

### 2.2 Frontend Technology Stack

| Category             | Technology               | Version    |
| -------------------- | ------------------------ | ---------- |
| Package Manager      | pnpm                     | 10.x       |
| Framework            | Next.js                  | 16.2.0     |
| UI Library           | React                    | 19.2.4     |
| Component Library    | Base UI                  | 1.0.0-rc.0 |
| Icons                | Lucide React             | 0.577.0    |
| State Management     | Zustand                  | 5.0.12     |
| Server State         | TanStack Query           | 5.91.2     |
| Data Tables          | TanStack Table           | 8.21.3     |
| Form Handling        | React Hook Form          | 7.71.2     |
| Validation           | Zod                      | 4.3.6      |
| Styling              | Tailwind CSS             | 4.2.2      |
| Internationalization | next-intl                | 4.8.3      |
| Theme                | next-themes              | 0.4.6      |
| Date Handling        | Day.js                   | 1.11.20    |
| Toast Notifications  | Sonner                   | 2.0.7      |
| Utilities            | clsx                     | 2.1.1      |
| Utilities            | tailwind-merge           | 3.5.0      |
| Utilities            | class-variance-authority | 0.7.1      |

### 2.3 Development Tools

| Category          | Tool            |
| ----------------- | --------------- |
| Language          | TypeScript 5.x  |
| Linting           | ESLint          |
| Test Framework    | Vitest          |
| Testing Utilities | Testing Library |

---

## 3. Project Structure

### 3.1 Repository Structure

```
rsmine/
├── backend/                 # Backend Rust project
│   ├── Cargo.toml
│   ├── Cargo.lock
│   ├── .env.example
│   ├── src/
│   └── migrations/
│
├── web/                     # Frontend Next.js project
│   ├── package.json
│   ├── package-lock.json
│   ├── tsconfig.json
│   ├── tailwind.config.ts
│   ├── next.config.js
│   ├── .env.example
│   └── src/
│
├── specs/                    # specification
│   └── RSMINE_SPEC.md       # This document
│
├── .gitignore
└── README.md
```

### 3.2 Backend Directory Structure (Onion Architecture)

**Key Decisions:**

- **Pure Domain Entities**: Domain entities are pure Rust structs with Serde traits, independent of any ORM
- **Repository Trait + Impl Separation**: Repository trait in domain layer, SeaORM implementation in infrastructure layer
- **Use Case: One File Per Use Case**: Each use case is a separate file for modularity
- **Error Handling: Thiserror Types**: Custom error types at each layer with From traits for conversion

```
backend/
├── Cargo.toml
├── Cargo.lock
├── .env.example
│
└── src/
    ├── main.rs                    # Entry point: start server, load configuration
    ├── lib.rs                     # Module exports
    │
    ├── config/                    # Configuration module
    │   ├── mod.rs
    │   ├── app_config.rs          # Application configuration struct
    │   └── database.rs            # Database configuration
    │
    ├── domain/                    # Domain layer (core, no external dependencies)
    │   ├── mod.rs
    │   │
    │   ├── entities/              # Pure domain entities (Serde-annotated structs)
    │   │   ├── mod.rs
    │   │   ├── user.rs
    │   │   ├── project.rs
    │   │   ├── issue.rs
    │   │   ├── attachment.rs
    │   │   ├── member.rs
    │   │   ├── role.rs
    │   │   ├── tracker.rs
    │   │   ├── issue_status.rs
    │   │   ├── issue_category.rs
    │   │   ├── issue_relation.rs
    │   │   ├── enumeration.rs
    │   │   ├── journal.rs
    │   │   ├── token.rs
    │   │   └── email_address.rs
    │   │
    │   ├── repositories/          # Repository traits (interfaces)
    │   │   ├── mod.rs
    │   │   ├── user_repository.rs
    │   │   ├── project_repository.rs
    │   │   ├── issue_repository.rs
    │   │   ├── member_repository.rs
    │   │   ├── role_repository.rs
    │   │   ├── attachment_repository.rs
    │   │   ├── tracker_repository.rs
    │   │   ├── issue_status_repository.rs
    │   │   ├── issue_category_repository.rs
    │   │   ├── issue_relation_repository.rs
    │   │   ├── enumeration_repository.rs
    │   │   ├── journal_repository.rs
    │   │   └── token_repository.rs
    │   │
    │   ├── services/              # Domain services
    │   │   ├── mod.rs
    │   │   ├── permission_service.rs    # Permission checking logic
    │   │   ├── nested_set_service.rs    # Nested set operations for projects/issues
    │   │   └── password_service.rs      # Password hashing/validation
    │   │
    │   ├── value_objects/         # Value objects
    │   │   ├── mod.rs
    │   │   ├── pagination.rs
    │   │   └── sort_order.rs
    │   │
    │   └── errors.rs              # Domain error types
    │
    ├── application/               # Application layer (business process orchestration)
    │   ├── mod.rs
    │   │
    │   ├── dto/                   # Request DTOs
    │   │   ├── mod.rs
    │   │   ├── auth/
    │   │   │   ├── mod.rs
    │   │   │   ├── login_request.rs
    │   │   │   └── register_request.rs
    │   │   ├── user/
    │   │   │   ├── mod.rs
    │   │   │   ├── create_user_request.rs
    │   │   │   └── update_user_request.rs
    │   │   ├── project/
    │   │   │   ├── mod.rs
    │   │   │   ├── create_project_request.rs
    │   │   │   └── update_project_request.rs
    │   │   ├── issue/
    │   │   │   ├── mod.rs
    │   │   │   ├── create_issue_request.rs
    │   │   │   ├── update_issue_request.rs
    │   │   │   └── issue_query_params.rs
    │   │   ├── member/
    │   │   │   ├── mod.rs
    │   │   │   └── create_member_request.rs
    │   │   ├── attachment/
    │   │   │   ├── mod.rs
    │   │   │   └── upload_request.rs
    │   │   └── relation/
    │   │       ├── mod.rs
    │   │       └── create_relation_request.rs
    │   │
    │   ├── use_cases/             # Use cases (one file per use case)
    │   │   ├── mod.rs
    │   │   │
    │   │   ├── auth/
    │   │   │   ├── login.rs
    │   │   │   ├── logout.rs
    │   │   │   └── get_current_user.rs
    │   │   │
    │   │   ├── user/
    │   │   │   ├── list_users.rs
    │   │   │   ├── get_user.rs
    │   │   │   ├── create_user.rs
    │   │   │   ├── update_user.rs
    │   │   │   └── delete_user.rs
    │   │   │
    │   │   ├── project/
    │   │   │   ├── list_projects.rs
    │   │   │   ├── get_project.rs
    │   │   │   ├── create_project.rs
    │   │   │   ├── update_project.rs
    │   │   │   ├── delete_project.rs
    │   │   │   └── close_project.rs
    │   │   │
    │   │   ├── issue/
    │   │   │   ├── list_issues.rs
    │   │   │   ├── get_issue.rs
    │   │   │   ├── create_issue.rs
    │   │   │   ├── update_issue.rs
    │   │   │   ├── delete_issue.rs
    │   │   │   └── bulk_update_issues.rs
    │   │   │
    │   │   ├── member/
    │   │   │   ├── list_members.rs
    │   │   │   ├── add_member.rs
    │   │   │   ├── update_member.rs
    │   │   │   └── remove_member.rs
    │   │   │
    │   │   ├── attachment/
    │   │   │   ├── upload_attachment.rs
    │   │   │   ├── download_attachment.rs
    │   │   │   └── delete_attachment.rs
    │   │   │
    │   │   └── relation/
    │   │       ├── list_relations.rs
    │   │       ├── create_relation.rs
    │   │       └── delete_relation.rs
    │   │
    │   └── errors.rs              # Application error types
    │
    ├── infrastructure/            # Infrastructure layer (external dependency implementations)
    │   ├── mod.rs
    │   │
    │   ├── persistence/           # Database persistence
    │   │   ├── mod.rs
    │   │   ├── db.rs              # Database connection pool
    │   │   │
    │   │   ├── entities/          # SeaORM database entities
    │   │   │   ├── mod.rs
    │   │   │   ├── prelude.rs
    │   │   │   ├── users.rs
    │   │   │   ├── projects.rs
    │   │   │   ├── issues.rs
    │   │   │   ├── members.rs
    │   │   │   ├── roles.rs
    │   │   │   ├── member_roles.rs
    │   │   │   ├── trackers.rs
    │   │   │   ├── issue_statuses.rs
    │   │   │   ├── issue_categories.rs
    │   │   │   ├── issue_relations.rs
    │   │   │   ├── enumerations.rs
    │   │   │   ├── attachments.rs
    │   │   │   ├── journals.rs
    │   │   │   ├── journal_details.rs
    │   │   │   ├── tokens.rs
    │   │   │   ├── email_addresses.rs
    │   │   │   └── projects_trackers.rs
    │   │   │
    │   │   ├── repositories/      # Repository trait implementations
    │   │   │   ├── mod.rs
    │   │   │   ├── user_repository_impl.rs
    │   │   │   ├── project_repository_impl.rs
    │   │   │   ├── issue_repository_impl.rs
    │   │   │   ├── member_repository_impl.rs
    │   │   │   ├── role_repository_impl.rs
    │   │   │   ├── attachment_repository_impl.rs
    │   │   │   ├── tracker_repository_impl.rs
    │   │   │   ├── issue_status_repository_impl.rs
    │   │   │   ├── issue_category_repository_impl.rs
    │   │   │   ├── issue_relation_repository_impl.rs
    │   │   │   ├── enumeration_repository_impl.rs
    │   │   │   ├── journal_repository_impl.rs
    │   │   │   └── token_repository_impl.rs
    │   │   │
    │   │   └── migrations/        # Database migrations
    │   │       ├── mod.rs
    │   │       ├── m20240101_000001_create_users.rs
    │   │       ├── m20240101_000002_create_projects.rs
    │   │       └── ...
    │   │
    │   ├── auth/                  # Authentication infrastructure
    │   │   ├── mod.rs
    │   │   ├── jwt.rs             # JWT token generation/validation
    │   │   ├── password.rs        # Argon2 password hashing
    │   │   └── hash.rs            # File hashing (SHA256)
    │   │
    │   └── storage/               # File storage
    │       ├── mod.rs
    │       └── local_storage.rs   # Local disk storage implementation
    │
    └── presentation/              # Presentation layer (HTTP API)
        ├── mod.rs
        │
        ├── api/                   # API routes
        │   ├── mod.rs
        │   ├── routes.rs          # Route definitions
        │   │
        │   └── v1/                # API version 1
        │       ├── mod.rs
        │       ├── auth.rs        # POST /auth/login, /auth/logout, GET /auth/me
        │       ├── users.rs       # User CRUD endpoints
        │       ├── projects.rs    # Project CRUD endpoints
        │       ├── issues.rs      # Issue CRUD endpoints
        │       ├── memberships.rs # Member management endpoints
        │       ├── relations.rs   # Issue relation endpoints
        │       ├── attachments.rs # Attachment endpoints
        │       ├── uploads.rs     # File upload endpoint
        │       ├── enums.rs       # Enumeration endpoints (trackers, statuses, priorities, roles)
        │       └── categories.rs  # Issue category endpoints
        │
        ├── middleware/            # HTTP middleware
        │   ├── mod.rs
        │   ├── auth.rs            # Authentication middleware
        │   ├── logging.rs         # Request/response logging
        │   └── cors.rs            # CORS configuration
        │
        ├── response/              # Response structures
        │   ├── mod.rs
        │   ├── api_response.rs    # Unified API response format
        │   └── paged_response.rs  # Paginated list response
        │
        ├── dto/                   # Response DTOs
        │   ├── mod.rs
        │   ├── user_response.rs
        │   ├── project_response.rs
        │   ├── issue_response.rs
        │   ├── member_response.rs
        │   ├── attachment_response.rs
        │   └── error_response.rs
        │
        └── errors.rs              # HTTP error handling
```

### 3.3 Frontend Directory Structure

**Key Decisions:**

- **API Client: Fetch Wrapper**: Manual typed fetch functions with full control
- **State Management: Server + Client Split**: TanStack Query for server state, Zustand for client state
- **Form Handling: React Hook Form + Zod**: Industry standard with schema validation
- **Components: Feature-Based**: Organized by feature domain

```
web/
├── package.json
├── package-lock.json
├── tsconfig.json
├── tailwind.config.ts
├── next.config.js
├── next-env.d.ts
├── .env.example
│
└── src/
    ├── app/                       # Next.js App Router
    │   ├── layout.tsx             # Root layout
    │   ├── page.tsx               # Homepage
    │   ├── globals.css            # Global styles
    │   │
    │   ├── (auth)/                # Auth route group (no sidebar)
    │   │   ├── layout.tsx
    │   │   ├── login/
    │   │   │   └── page.tsx
    │   │   └── register/
    │   │       └── page.tsx
    │   │
    │   ├── (dashboard)/           # Dashboard route group (with sidebar)
    │   │   ├── layout.tsx
    │   │   ├── page.tsx           # Dashboard home
    │   │   │
    │   │   ├── projects/
    │   │   │   ├── page.tsx       # Project list
    │   │   │   ├── new/
    │   │   │   │   └── page.tsx   # Create project
    │   │   │   └── [id]/
    │   │   │       ├── page.tsx   # Project details
    │   │   │       ├── edit/
    │   │   │       │   └── page.tsx
    │   │   │       ├── issues/
    │   │   │       │   ├── page.tsx       # Project issue list
    │   │   │       │   └── new/
    │   │   │       │       └── page.tsx   # Create issue
    │   │   │       ├── members/
    │   │   │       │   └── page.tsx
    │   │   │       ├── settings/
    │   │   │       │   └── page.tsx
    │   │   │       └── categories/
    │   │   │           └── page.tsx
    │   │   │
    │   │   ├── issues/
    │   │   │   ├── page.tsx       # Global issue list
    │   │   │   └── [id]/
    │   │   │       ├── page.tsx   # Issue details
    │   │   │       └── edit/
    │   │   │           └── page.tsx
    │   │   │
    │   │   ├── users/
    │   │   │   ├── page.tsx       # User list (admin)
    │   │   │   ├── new/
    │   │   │   │   └── page.tsx   # Create user
    │   │   │   └── [id]/
    │   │   │       ├── page.tsx   # User details
    │   │   │       └── edit/
    │   │   │           └── page.tsx
    │   │   │
    │   │   └── settings/
    │   │       ├── page.tsx       # Settings overview
    │   │       ├── roles/
    │   │       │   └── page.tsx
    │   │       ├── trackers/
    │   │       │   └── page.tsx
    │   │       ├── statuses/
    │   │       │   └── page.tsx
    │   │       └── priorities/
    │   │           └── page.tsx
    │   │
    │   └── api/                   # Next.js API Routes (if needed)
    │       └── ...
    │
    ├── components/                # React components
    │   │
    │   ├── ui/                    # Base UI components (from Base UI / custom)
    │   │   ├── button.tsx
    │   │   ├── input.tsx
    │   │   ├── select.tsx
    │   │   ├── dialog.tsx
    │   │   ├── dropdown-menu.tsx
    │   │   ├── table.tsx
    │   │   ├── card.tsx
    │   │   ├── badge.tsx
    │   │   ├── avatar.tsx
    │   │   ├── tabs.tsx
    │   │   ├── toast.tsx
    │   │   ├── skeleton.tsx
    │   │   ├── separator.tsx
    │   │   ├── tooltip.tsx
    │   │   ├── popover.tsx
    │   │   ├── command.tsx        # For command palette
    │   │   ├── form.tsx           # Form components wrapper
    │   │   └── data-table.tsx     # TanStack Table wrapper
    │   │
    │   ├── layout/                # Layout components
    │   │   ├── header.tsx
    │   │   ├── sidebar.tsx
    │   │   ├── footer.tsx
    │   │   ├── breadcrumb.tsx
    │   │   ├── user-nav.tsx       # User dropdown in header
    │   │   └── nav-item.tsx
    │   │
    │   └── features/              # Feature components (organized by domain)
    │       │
    │       ├── auth/
    │       │   ├── login-form.tsx
    │       │   └── register-form.tsx
    │       │
    │       ├── project/
    │       │   ├── project-card.tsx
    │       │   ├── project-list.tsx
    │       │   ├── project-form.tsx
    │       │   ├── project-header.tsx
    │       │   └── project-stats.tsx
    │       │
    │       ├── issue/
    │       │   ├── issue-list.tsx
    │       │   ├── issue-card.tsx
    │       │   ├── issue-form.tsx
    │       │   ├── issue-detail.tsx
    │       │   ├── issue-sidebar.tsx
    │       │   ├── issue-filter.tsx
    │       │   ├── issue-status-badge.tsx
    │       │   ├── issue-priority-badge.tsx
    │       │   └── issue-journals.tsx    # Change history
    │       │
    │       ├── member/
    │       │   ├── member-list.tsx
    │       │   ├── member-form.tsx
    │       │   └── member-row.tsx
    │       │
    │       ├── attachment/
    │       │   ├── attachment-list.tsx
    │       │   ├── attachment-upload.tsx
    │       │   └── attachment-preview.tsx
    │       │
    │       ├── relation/
    │       │   ├── relation-list.tsx
    │       │   └── relation-form.tsx
    │       │
    │       ├── user/
    │       │   ├── user-list.tsx
    │       │   ├── user-form.tsx
    │       │   ├── user-card.tsx
    │       │   └── user-select.tsx     # User dropdown selector
    │       │
    │       ├── settings/
    │       │   ├── role-form.tsx
    │       │   ├── tracker-form.tsx
    │       │   ├── status-form.tsx
    │       │   └── priority-form.tsx
    │       │
    │       └── common/
    │           ├── loading.tsx
    │           ├── error-boundary.tsx
    │           ├── empty-state.tsx
    │           ├── confirm-dialog.tsx
    │           ├── pagination.tsx
    │           └── search-input.tsx
    │
    ├── hooks/                     # Custom Hooks
    │   ├── use-auth.ts
    │   ├── use-permission.ts
    │   ├── use-current-user.ts
    │   └── use-debounce.ts
    │
    ├── stores/                    # Zustand state management (client state only)
    │   ├── index.ts
    │   ├── auth-store.ts          # Auth state (user info, login status)
    │   ├── sidebar-store.ts       # Sidebar collapse state
    │   └── filter-store.ts        # Filter panel state
    │
    ├── lib/                       # Utility libraries
    │   │
    │   ├── api/                   # API client (Fetch Wrapper)
    │   │   ├── index.ts
    │   │   ├── client.ts          # Base fetch wrapper with auth
    │   │   ├── auth.ts            # Auth API functions
    │   │   ├── users.ts           # User API functions
    │   │   ├── projects.ts        # Project API functions
    │   │   ├── issues.ts          # Issue API functions
    │   │   ├── members.ts         # Member API functions
    │   │   ├── attachments.ts     # Attachment API functions
    │   │   ├── uploads.ts         # Upload API functions
    │   │   ├── enums.ts           # Enumeration API functions
    │   │   └── types.ts           # API response types
    │   │
    │   ├── utils/
    │   │   ├── index.ts
    │   │   ├── cn.ts              # Class name utility (clsx + tailwind-merge)
    │   │   ├── format.ts          # Date/number formatting
    │   │   └── validation.ts      # Common validation helpers
    │   │
    │   └── constants/
    │       ├── index.ts
    │       ├── routes.ts          # Route constants
    │       ├── status.ts          # Status constants
    │       └── permissions.ts     # Permission constants
    │
    ├── types/                     # TypeScript type definitions
    │   ├── index.ts
    │   ├── api.ts                 # API types (pagination, etc.)
    │   ├── user.ts
    │   ├── project.ts
    │   ├── issue.ts
    │   ├── member.ts
    │   ├── attachment.ts
    │   ├── journal.ts
    │   ├── role.ts
    │   └── enumeration.ts
    │
    └── i18n/                      # Internationalization
        ├── index.ts
        ├── request.ts             # next-intl configuration
        └── messages/              # Translation files
            ├── en.json
            └── zh.json
```

---

## 4. Database Design

### 4.1 MVP Table Structure (17 Tables)

```
┌─────────────────────────────────────────────────────────────────┐
│                        Database Table Structure                  │
├─────────────────────────────────────────────────────────────────┤
│  Authentication Module (3 Tables)                                │
│  ├── users              Users table                             │
│  ├── email_addresses    Email addresses                         │
│  └── tokens             Session/API Token                       │
├─────────────────────────────────────────────────────────────────┤
│  Permission Module (3 Tables)                                    │
│  ├── roles              Roles table                             │
│  ├── members            Project members                         │
│  └── member_roles       Member-role associations                │
├─────────────────────────────────────────────────────────────────┤
│  Project Module (2 Tables)                                       │
│  ├── projects           Projects table                          │
│  └── projects_trackers  Project-tracker associations            │
├─────────────────────────────────────────────────────────────────┤
│  Issue Basic Configuration (4 Tables)                            │
│  ├── trackers           Tracker types                           │
│  ├── issue_statuses     Statuses                                │
│  ├── issue_categories   Categories                              │
│  └── enumerations       Enumerations (priorities, etc.)         │
├─────────────────────────────────────────────────────────────────┤
│  Issue Core Data (3 Tables)                                      │
│  ├── issues             Issues table (including subtasks)       │
│  ├── issue_relations    Issue relations                         │
│  └── attachments        Attachments                             │
├─────────────────────────────────────────────────────────────────┤
│  Audit Log (2 Tables)                                            │
│  ├── journals           Change records (including notes)        │
│  └── journal_details    Change details                          │
└─────────────────────────────────────────────────────────────────┘
```

### 4.2 Core Table Field Descriptions

#### 4.2.1 users - Users Table

| Field                   | Type         | Constraint          | Description                              |
| ----------------------- | ------------ | ------------------- | ---------------------------------------- |
| id                      | INTEGER      | PK, AUTO            | Primary key                              |
| login                   | VARCHAR      | NOT NULL            | Login name                               |
| hashed_password         | VARCHAR(40)  |                     | Password hash                            |
| firstname               | VARCHAR(30)  | NOT NULL            | First name                               |
| lastname                | VARCHAR(255) | NOT NULL            | Last name                                |
| admin                   | BOOLEAN      | NOT NULL, DEFAULT 0 | Is administrator                         |
| status                  | INTEGER      | NOT NULL, DEFAULT 1 | Status: 1=active, 2=registered, 3=locked |
| last_login_on           | DATETIME     |                     | Last login time                          |
| language                | VARCHAR(5)   |                     | Language preference                      |
| auth_source_id          | INTEGER      | FK                  | External authentication source           |
| created_on              | DATETIME     |                     | Created time                             |
| updated_on              | DATETIME     |                     | Updated time                             |
| type                    | VARCHAR      |                     | Type: User/Group                         |
| mail_notification       | VARCHAR      | NOT NULL            | Email notification settings              |
| salt                    | VARCHAR(64)  |                     | Password salt                            |
| must_change_passwd      | BOOLEAN      | NOT NULL, DEFAULT 0 | Must change password                     |
| passwd_changed_on       | DATETIME     |                     | Password changed time                    |
| twofa_scheme            | VARCHAR      |                     | Two-factor authentication scheme         |
| twofa_totp_key          | VARCHAR      |                     | TOTP secret key                          |
| twofa_totp_last_used_at | INTEGER      |                     | TOTP last used time                      |
| twofa_required          | BOOLEAN      | NOT NULL, DEFAULT 0 | Requires two-factor authentication       |

#### 4.2.2 projects - Projects Table

| Field                  | Type     | Constraint          | Description                            |
| ---------------------- | -------- | ------------------- | -------------------------------------- |
| id                     | INTEGER  | PK, AUTO            | Primary key                            |
| name                   | VARCHAR  | NOT NULL            | Project name                           |
| description            | TEXT     |                     | Description                            |
| homepage               | VARCHAR  |                     | Homepage URL                           |
| is_public              | BOOLEAN  | NOT NULL, DEFAULT 1 | Is public                              |
| parent_id              | INTEGER  | FK                  | Parent project ID                      |
| created_on             | DATETIME |                     | Created time                           |
| updated_on             | DATETIME |                     | Updated time                           |
| identifier             | VARCHAR  | UNIQUE              | Project identifier                     |
| status                 | INTEGER  | NOT NULL, DEFAULT 1 | Status: 1=active, 5=closed, 9=archived |
| lft                    | INTEGER  |                     | Nested set left value                  |
| rgt                    | INTEGER  |                     | Nested set right value                 |
| inherit_members        | BOOLEAN  | NOT NULL, DEFAULT 0 | Inherit members                        |
| default_version_id     | INTEGER  | FK                  | Default version                        |
| default_assigned_to_id | INTEGER  | FK                  | Default assignee                       |
| default_issue_query_id | INTEGER  | FK                  | Default query                          |

#### 4.2.3 issues - Issues Table

| Field            | Type     | Constraint          | Description               |
| ---------------- | -------- | ------------------- | ------------------------- |
| id               | INTEGER  | PK, AUTO            | Primary key               |
| tracker_id       | INTEGER  | FK, NOT NULL        | Tracker ID                |
| project_id       | INTEGER  | FK, NOT NULL        | Project ID                |
| subject          | VARCHAR  | NOT NULL            | Subject                   |
| description      | TEXT     |                     | Description               |
| due_date         | DATE     |                     | Due date                  |
| category_id      | INTEGER  | FK                  | Category ID               |
| status_id        | INTEGER  | FK, NOT NULL        | Status ID                 |
| assigned_to_id   | INTEGER  | FK                  | Assignee ID               |
| priority_id      | INTEGER  | FK, NOT NULL        | Priority ID               |
| fixed_version_id | INTEGER  | FK                  | Target version ID         |
| author_id        | INTEGER  | FK, NOT NULL        | Author ID                 |
| lock_version     | INTEGER  | NOT NULL, DEFAULT 0 | Optimistic lock version   |
| created_on       | DATETIME |                     | Created time              |
| updated_on       | DATETIME |                     | Updated time              |
| start_date       | DATE     |                     | Start date                |
| done_ratio       | INTEGER  | NOT NULL, DEFAULT 0 | Completion ratio (0-100)  |
| estimated_hours  | FLOAT    |                     | Estimated hours           |
| parent_id        | INTEGER  | FK                  | Parent Issue ID (subtask) |
| root_id          | INTEGER  |                     | Root Issue ID             |
| lft              | INTEGER  |                     | Nested set left value     |
| rgt              | INTEGER  |                     | Nested set right value    |
| is_private       | BOOLEAN  | NOT NULL, DEFAULT 0 | Is private                |
| closed_on        | DATETIME |                     | Closed time               |

#### 4.2.4 roles - Roles Table

| Field                          | Type        | Constraint                  | Description                                        |
| ------------------------------ | ----------- | --------------------------- | -------------------------------------------------- |
| id                             | INTEGER     | PK, AUTO                    | Primary key                                        |
| name                           | VARCHAR     |                             | Role name                                          |
| position                       | INTEGER     |                             | Sort position                                      |
| assignable                     | BOOLEAN     | NOT NULL, DEFAULT 1         | Is assignable                                      |
| builtin                        | INTEGER     | NOT NULL, DEFAULT 0         | Built-in flag: 0=custom, 1=non-member, 2=anonymous |
| permissions                    | TEXT        |                             | Permission list (JSON format)                      |
| issues_visibility              | VARCHAR(30) | NOT NULL, DEFAULT 'default' | Issue visibility: all/default/own                  |
| users_visibility               | VARCHAR(30) | NOT NULL                    | Users visibility                                   |
| time_entries_visibility        | VARCHAR(30) | NOT NULL                    | Time entries visibility                            |
| all_roles_managed              | BOOLEAN     | NOT NULL, DEFAULT 1         | Manage all roles                                   |
| settings                       | TEXT        |                             | Role settings                                      |
| default_time_entry_activity_id | INTEGER     | FK                          | Default time entry activity                        |

#### 4.2.5 members - Project Members Table

| Field             | Type     | Constraint          | Description                |
| ----------------- | -------- | ------------------- | -------------------------- |
| id                | INTEGER  | PK, AUTO            | Primary key                |
| user_id           | INTEGER  | FK, NOT NULL        | User ID                    |
| project_id        | INTEGER  | FK, NOT NULL        | Project ID                 |
| created_on        | DATETIME |                     | Created time               |
| mail_notification | BOOLEAN  | NOT NULL, DEFAULT 0 | Email notification enabled |

#### 4.2.6 member_roles - Member Roles Association Table

| Field          | Type    | Constraint   | Description    |
| -------------- | ------- | ------------ | -------------- |
| id             | INTEGER | PK, AUTO     | Primary key    |
| member_id      | INTEGER | FK, NOT NULL | Member ID      |
| role_id        | INTEGER | FK, NOT NULL | Role ID        |
| inherited_from | INTEGER | FK           | Inherited from |

#### 4.2.7 attachments - Attachments Table

| Field          | Type        | Constraint          | Description          |
| -------------- | ----------- | ------------------- | -------------------- |
| id             | INTEGER     | PK, AUTO            | Primary key          |
| container_id   | INTEGER     |                     | Container ID         |
| container_type | VARCHAR(30) |                     | Container type       |
| filename       | VARCHAR     | NOT NULL            | Original filename    |
| disk_filename  | VARCHAR     | NOT NULL            | Disk filename        |
| filesize       | BIGINT      | NOT NULL, DEFAULT 0 | File size            |
| content_type   | VARCHAR     |                     | MIME type            |
| digest         | VARCHAR(64) |                     | File digest (SHA256) |
| downloads      | INTEGER     | NOT NULL, DEFAULT 0 | Download count       |
| author_id      | INTEGER     | FK, NOT NULL        | Uploader ID          |
| created_on     | DATETIME    |                     | Created time         |
| description    | VARCHAR     |                     | Description          |
| disk_directory | VARCHAR     |                     | Disk directory       |

#### 4.2.8 journals - Change Records Table

| Field            | Type        | Constraint          | Description            |
| ---------------- | ----------- | ------------------- | ---------------------- |
| id               | INTEGER     | PK, AUTO            | Primary key            |
| journalized_id   | INTEGER     | NOT NULL            | Associated object ID   |
| journalized_type | VARCHAR(30) | NOT NULL            | Associated object type |
| user_id          | INTEGER     | FK, NOT NULL        | Operator ID            |
| notes            | TEXT        |                     | Notes                  |
| created_on       | DATETIME    | NOT NULL            | Created time           |
| private_notes    | BOOLEAN     | NOT NULL, DEFAULT 0 | Is private note        |
| updated_on       | DATETIME    |                     | Updated time           |
| updated_by_id    | INTEGER     | FK                  | Updater ID             |

#### 4.2.9 issue_relations - Issue Relations Table

| Field         | Type    | Constraint   | Description                          |
| ------------- | ------- | ------------ | ------------------------------------ |
| id            | INTEGER | PK, AUTO     | Primary key                          |
| issue_from_id | INTEGER | FK, NOT NULL | Source Issue ID                      |
| issue_to_id   | INTEGER | FK, NOT NULL | Target Issue ID                      |
| relation_type | VARCHAR | NOT NULL     | Relation type (see below)            |
| delay         | INTEGER |              | Delay in days (for precedes/follows) |

**Relation Types:**

| Type       | Description | Directionality                        |
| ---------- | ----------- | ------------------------------------- |
| relates    | Related     | Bidirectional                         |
| duplicates | Duplicates  | Bidirectional (reverse: duplicated)   |
| blocks     | Blocks      | Unidirectional (reverse: blocked)     |
| precedes   | Precedes    | Unidirectional (reverse: follows)     |
| follows    | Follows     | Unidirectional (reverse: precedes)    |
| copied_to  | Copied to   | Unidirectional (reverse: copied_from) |

> Note: Redmine internally stores both directions for bidirectional relations. When creating a relation with type `duplicates`, the reverse relation with type `duplicated` is automatically created.

### 4.3 Database Indexes

**Strategy: Mirror Redmine's existing indexes**

Key indexes from Redmine (to be replicated):

```sql
-- Users
CREATE INDEX idx_users_login ON users(login);
CREATE INDEX idx_users_type ON users(type);
CREATE INDEX idx_users_status ON users(status);

-- Projects
CREATE INDEX idx_projects_parent_id ON projects(parent_id);
CREATE INDEX idx_projects_status ON projects(status);
CREATE INDEX idx_projects_lft_rgt ON projects(lft, rgt);
CREATE INDEX idx_projects_identifier ON projects(identifier);

-- Issues
CREATE INDEX idx_issues_project_id ON issues(project_id);
CREATE INDEX idx_issues_status_id ON issues(status_id);
CREATE INDEX idx_issues_assigned_to_id ON issues(assigned_to_id);
CREATE INDEX idx_issues_author_id ON issues(author_id);
CREATE INDEX idx_issues_tracker_id ON issues(tracker_id);
CREATE INDEX idx_issues_priority_id ON issues(priority_id);
CREATE INDEX idx_issues_category_id ON issues(category_id);
CREATE INDEX idx_issues_parent_id ON issues(parent_id);
CREATE INDEX idx_issues_root_id ON issues(root_id);
CREATE INDEX idx_issues_lft_rgt ON issues(root_id, lft, rgt);
CREATE INDEX idx_issues_created_on ON issues(created_on);
CREATE INDEX idx_issues_closed_on ON issues(closed_on);

-- Members
CREATE INDEX idx_members_user_id ON members(user_id);
CREATE INDEX idx_members_project_id ON members(project_id);

-- Attachments
CREATE INDEX idx_attachments_container ON attachments(container_id, container_type);
CREATE INDEX idx_attachments_author_id ON attachments(author_id);

-- Journals
CREATE INDEX idx_journals_journalized ON journals(journalized_id, journalized_type);
CREATE INDEX idx_journals_user_id ON journals(user_id);
CREATE INDEX idx_journals_created_on ON journals(created_on);

-- Issue Relations
CREATE INDEX idx_issue_relations_issue_from ON issue_relations(issue_from_id);
CREATE INDEX idx_issue_relations_issue_to ON issue_relations(issue_to_id);
```

### 4.4 Database Compatibility

| Database   | Support Level      | Development Environment    |
| ---------- | ------------------ | -------------------------- |
| SQLite     | ✅ Fully supported | Default                    |
| PostgreSQL | ✅ Fully supported | Recommended for production |
| MySQL      | ✅ Fully supported | Optional for production    |

---

## 5. API Design

### 5.1 General Specifications

#### 5.1.1 Base URL

```
/api/v1/
```

#### 5.1.2 Authentication Methods

```
# JWT Token
Authorization: Bearer <jwt_token>
```

#### 5.1.3 Request Format

```
Content-Type: application/json
```

#### 5.1.4 Pagination Parameters

```
GET /api/v1/issues?offset=0&limit=25
```

| Parameter | Type    | Default | Description              |
| --------- | ------- | ------- | ------------------------ |
| offset    | integer | 0       | Offset                   |
| limit     | integer | 25      | Items per page (max 100) |

#### 5.1.5 Include Parameter

```
GET /api/v1/issues/1?include=attachments,journals,relations,children
```

#### 5.1.6 Response Format

**List Response:**

```json
{
  "issues": [...],
  "total_count": 100,
  "offset": 0,
  "limit": 25
}
```

**Single Resource Response:**

```json
{
  "issue": {
    "id": 1,
    "subject": "...",
    ...
  }
}
```

**Error Response:**

```json
{
  "errors": ["Subject cannot be blank", "Project cannot be blank"]
}
```

#### 5.1.7 HTTP Status Codes

| Status Code               | Description                 |
| ------------------------- | --------------------------- |
| 200 OK                    | Success (GET/PUT)           |
| 201 Created               | Successfully created (POST) |
| 204 No Content            | Success (DELETE)            |
| 400 Bad Request           | Request format error        |
| 401 Unauthorized          | Not authenticated           |
| 403 Forbidden             | No permission               |
| 404 Not Found             | Resource not found          |
| 422 Unprocessable Entity  | Validation error            |
| 500 Internal Server Error | Server error                |

### 5.2 API Endpoint List

#### 5.2.1 Authentication

| Method | Endpoint            | Description       | Permission     |
| ------ | ------------------- | ----------------- | -------------- |
| POST   | /api/v1/auth/login  | Login             | Public         |
| POST   | /api/v1/auth/logout | Logout            | Login required |
| GET    | /api/v1/auth/me     | Current user info | Login required |

**Login Request:**

```json
{
  "username": "admin",
  "password": "password123"
}
```

**Login Response:**

```json
{
  "token": "eyJhbGciOiJIUzI1NiIsInR5cCI6IkpXVCJ9...",
  "user": {
    "id": 1,
    "login": "admin",
    "firstname": "Admin",
    "lastname": "User",
    "admin": true
  }
}
```

#### 5.2.2 Users

| Method | Endpoint          | Description  | Permission     |
| ------ | ----------------- | ------------ | -------------- |
| GET    | /api/v1/users     | User list    | Login required |
| POST   | /api/v1/users     | Create user  | admin          |
| GET    | /api/v1/users/:id | User details | Login required |
| PUT    | /api/v1/users/:id | Update user  | admin or self  |
| DELETE | /api/v1/users/:id | Delete user  | admin          |

#### 5.2.3 Projects

| Method | Endpoint             | Description     | Permission           |
| ------ | -------------------- | --------------- | -------------------- |
| GET    | /api/v1/projects     | Project list    | view_project         |
| POST   | /api/v1/projects     | Create project  | add_project (global) |
| GET    | /api/v1/projects/:id | Project details | view_project         |
| PUT    | /api/v1/projects/:id | Update project  | edit_project         |
| DELETE | /api/v1/projects/:id | Delete project  | delete_project       |

#### 5.2.4 Project Members

| Method | Endpoint                         | Description    | Permission     |
| ------ | -------------------------------- | -------------- | -------------- |
| GET    | /api/v1/projects/:id/memberships | Member list    | view_members   |
| POST   | /api/v1/projects/:id/memberships | Add member     | manage_members |
| GET    | /api/v1/memberships/:id          | Member details | view_members   |
| PUT    | /api/v1/memberships/:id          | Update roles   | manage_members |
| DELETE | /api/v1/memberships/:id          | Remove member  | manage_members |

#### 5.2.5 Project Trackers

| Method | Endpoint                      | Description      | Permission   |
| ------ | ----------------------------- | ---------------- | ------------ |
| GET    | /api/v1/projects/:id/trackers | Project trackers | view_project |

#### 5.2.6 Issue Categories

| Method | Endpoint                              | Description      | Permission        |
| ------ | ------------------------------------- | ---------------- | ----------------- |
| GET    | /api/v1/projects/:id/issue_categories | Category list    | view_project      |
| POST   | /api/v1/projects/:id/issue_categories | Create category  | manage_categories |
| GET    | /api/v1/issue_categories/:id          | Category details | view_project      |
| PUT    | /api/v1/issue_categories/:id          | Update category  | manage_categories |
| DELETE | /api/v1/issue_categories/:id          | Delete category  | manage_categories |

#### 5.2.7 Issues

| Method | Endpoint           | Description   | Permission                    |
| ------ | ------------------ | ------------- | ----------------------------- |
| GET    | /api/v1/issues     | Issue list    | view_issues                   |
| POST   | /api/v1/issues     | Create issue  | add_issues                    |
| GET    | /api/v1/issues/:id | Issue details | view_issues                   |
| PUT    | /api/v1/issues/:id | Update issue  | edit_issues / edit_own_issues |
| DELETE | /api/v1/issues/:id | Delete issue  | delete_issues                 |

**Create Issue Request:**

```json
{
  "issue": {
    "project_id": 1,
    "tracker_id": 1,
    "subject": "Fix the bug",
    "description": "Description here...",
    "status_id": 1,
    "priority_id": 2,
    "assigned_to_id": 3,
    "category_id": 1,
    "parent_id": null,
    "start_date": "2026-03-19",
    "due_date": "2026-03-26",
    "estimated_hours": 4.0
  }
}
```

**Update Issue (with notes):**

```json
{
  "issue": {
    "status_id": 2,
    "done_ratio": 50,
    "notes": "Progress update: completed initial analysis."
  }
}
```

#### 5.2.8 Issue Relations

| Method | Endpoint                     | Description      | Permission             |
| ------ | ---------------------------- | ---------------- | ---------------------- |
| GET    | /api/v1/issues/:id/relations | Relation list    | view_issues            |
| POST   | /api/v1/issues/:id/relations | Create relation  | manage_issue_relations |
| GET    | /api/v1/relations/:id        | Relation details | view_issues            |
| DELETE | /api/v1/relations/:id        | Delete relation  | manage_issue_relations |

**Create Relation Request:**

```json
{
  "relation": {
    "issue_to_id": 42,
    "relation_type": "relates",
    "delay": 0
  }
}
```

#### 5.2.9 Issue Attachments

| Method | Endpoint                       | Description       | Permission               |
| ------ | ------------------------------ | ----------------- | ------------------------ |
| GET    | /api/v1/issues/:id/attachments | Attachment list   | view_issues + view_files |
| POST   | /api/v1/issues/:id/attachments | Upload attachment | manage_files             |
| GET    | /api/v1/issues/:id/journals    | Change history    | view_issues              |

#### 5.2.10 Attachments

| Method | Endpoint                         | Description         | Permission   |
| ------ | -------------------------------- | ------------------- | ------------ |
| GET    | /api/v1/attachments/:id          | Attachment metadata | view_files   |
| GET    | /api/v1/attachments/download/:id | Download attachment | view_files   |
| PATCH  | /api/v1/attachments/:id          | Update description  | manage_files |
| DELETE | /api/v1/attachments/:id          | Delete attachment   | manage_files |

#### 5.2.11 Uploads

| Method | Endpoint        | Description | Permission     |
| ------ | --------------- | ----------- | -------------- |
| POST   | /api/v1/uploads | Upload file | Login required |

**Upload Request:**

```
POST /api/v1/uploads
Content-Type: multipart/form-data

file: <binary>
```

**Upload Response:**

```json
{
  "upload": {
    "token": "7b8d9f2e3a1c4b5d6e7f8a9b0c1d2e3f4a5b6c7d"
  }
}
```

#### 5.2.12 Enumerations (Configuration Data)

| Method | Endpoint                              | Description     | Permission     |
| ------ | ------------------------------------- | --------------- | -------------- |
| GET    | /api/v1/trackers                      | Tracker list    | Login required |
| POST   | /api/v1/trackers                      | Create tracker  | admin          |
| GET    | /api/v1/trackers/:id                  | Tracker details | Login required |
| PUT    | /api/v1/trackers/:id                  | Update tracker  | admin          |
| DELETE | /api/v1/trackers/:id                  | Delete tracker  | admin          |
| GET    | /api/v1/issue_statuses                | Status list     | Login required |
| POST   | /api/v1/issue_statuses                | Create status   | admin          |
| GET    | /api/v1/issue_statuses/:id            | Status details  | Login required |
| PUT    | /api/v1/issue_statuses/:id            | Update status   | admin          |
| DELETE | /api/v1/issue_statuses/:id            | Delete status   | admin          |
| GET    | /api/v1/enumerations/issue_priorities | Priority list   | Login required |
| GET    | /api/v1/roles                         | Role list       | Login required |
| POST   | /api/v1/roles                         | Create role     | admin          |
| GET    | /api/v1/roles/:id                     | Role details    | Login required |
| PUT    | /api/v1/roles/:id                     | Update role     | admin          |
| DELETE | /api/v1/roles/:id                     | Delete role     | admin          |

#### 5.2.13 API Documentation

| Method | Endpoint          | Description                |
| ------ | ----------------- | -------------------------- |
| GET    | /api/docs         | Swagger UI                 |
| GET    | /api/docs/openapi | OpenAPI JSON specification |

**Implementation:** Auto-generated using utoipa crate from Rust code annotations.

### 5.3 Query Filtering

#### 5.3.1 Issue List Filtering

```
GET /api/v1/issues?project_id=1&status_id=open&assigned_to_id=me&tracker_id=1
```

| Parameter      | Description                         |
| -------------- | ----------------------------------- |
| project_id     | Project ID                          |
| status_id      | Status ID or `open`/`closed`        |
| tracker_id     | Tracker ID                          |
| priority_id    | Priority ID                         |
| category_id    | Category ID                         |
| assigned_to_id | Assignee ID or `me`                 |
| author_id      | Author ID                           |
| subject        | Subject fuzzy search (`~keyword`)   |
| parent_id      | Parent Issue ID                     |
| created_on     | Created time range (`>=2026-01-01`) |
| updated_on     | Updated time range                  |

---

## 6. Permission System

### 6.1 Permission Definitions

#### 6.1.1 Global Permissions

| Permission               | Description                |
| ------------------------ | -------------------------- |
| view_project             | View project               |
| add_project              | Create project             |
| edit_project             | Edit project               |
| close_project            | Close/reopen project       |
| delete_project           | Delete project             |
| select_project_publicity | Set project public/private |
| view_members             | View members               |
| manage_members           | Manage members             |

#### 6.1.2 Issue Tracking Module Permissions

| Permission             | Description               |
| ---------------------- | ------------------------- |
| view_issues            | View Issues               |
| add_issues             | Create Issues             |
| edit_issues            | Edit all Issues           |
| edit_own_issues        | Edit own Issues           |
| delete_issues          | Delete Issues             |
| set_issues_private     | Set Issues as private     |
| set_own_issues_private | Set own Issues as private |
| manage_issue_relations | Manage Issue relations    |
| manage_subtasks        | Manage subtasks           |
| add_issue_notes        | Add notes                 |
| view_private_notes     | View private notes        |
| set_notes_private      | Set notes as private      |
| manage_categories      | Manage categories         |

#### 6.1.3 Files Module Permissions

| Permission   | Description                        |
| ------------ | ---------------------------------- |
| view_files   | View attachments                   |
| manage_files | Manage attachments (upload/delete) |

### 6.2 Role Definitions

#### 6.2.1 Preset Roles

| Role       | builtin | issues_visibility | Description                                   |
| ---------- | ------- | ----------------- | --------------------------------------------- |
| Manager    | 0       | all               | Administrator, has all permissions            |
| Developer  | 0       | all               | Developer, can edit Issues                    |
| Reporter   | 0       | default           | Reporter, can create and add notes            |
| Non-Member | 1       | default           | Non-member (logged in but not project member) |
| Anonymous  | 2       | default           | Anonymous user (not logged in)                |

#### 6.2.2 Role Permission Matrix

```
┌──────────────────────┬─────────┬───────────┬──────────┬───────────┬───────────┐
│ Permission           │ Manager │ Developer │ Reporter │ Non-Member │ Anonymous │
├──────────────────────┼─────────┼───────────┼──────────┼───────────┼───────────┤
│ Global Permissions                                                                     │
│ view_project         │    ✅    │     ✅     │    ✅     │     ✅     │     ✅    │
│ add_project          │    ✅    │     -     │    -     │     -     │     -    │
│ edit_project         │    ✅    │     -     │    -     │     -     │     -    │
│ close_project        │    ✅    │     -     │    -     │     -     │     -    │
│ delete_project       │    ✅    │     -     │    -     │     -     │     -    │
│ select_project_publicity │ ✅   │     -     │    -     │     -     │     -    │
│ view_members         │    ✅    │     ✅     │    ✅     │     ✅     │     ✅    │
│ manage_members       │    ✅    │     -     │    -     │     -     │     -    │
├──────────────────────┼─────────┼───────────┼──────────┼───────────┼───────────┤
│ Issue Tracking                                                                         │
│ view_issues          │    ✅    │     ✅     │    ✅     │     ✅     │     ✅    │
│ add_issues           │    ✅    │     ✅     │    ✅     │     -     │     -    │
│ edit_issues          │    ✅    │     ✅     │    -     │     -     │     -    │
│ edit_own_issues      │    ✅    │     ✅     │    ✅     │     ✅     │    -    │
│ delete_issues        │    ✅    │     -     │    -     │     -     │     -    │
│ set_issues_private   │    ✅    │     ✅     │    -     │     -     │     -    │
│ set_own_issues_private│   ✅    │     ✅     │    ✅     │     -     │     -    │
│ manage_issue_relations│   ✅    │     ✅     │    -     │     -     │     -    │
│ manage_subtasks      │    ✅    │     ✅     │    -     │     -     │     -    │
│ add_issue_notes      │    ✅    │     ✅     │    ✅     │     ✅     │    -    │
│ view_private_notes   │    ✅    │     ✅     │    -     │     -     │     -    │
│ set_notes_private    │    ✅    │     ✅     │    -     │     -     │     -    │
│ manage_categories    │    ✅    │     -     │    -     │     -     │     -    │
├──────────────────────┼─────────┼───────────┼──────────┼───────────┼───────────┤
│ Files (Attachments)                                                                   │
│ view_files           │    ✅    │     ✅     │    ✅     │     ✅     │     ✅    │
│ manage_files         │    ✅    │     ✅     │    -     │     -     │     -    │
└──────────────────────┴─────────┴───────────┴──────────┴───────────┴───────────┘
```

### 6.3 Permission Check Logic

#### 6.3.1 Administrator Skip Check

Users with `users.admin = true` skip all permission checks.

#### 6.3.2 issues_visibility Logic

| Value   | Visibility Scope                                                |
| ------- | --------------------------------------------------------------- |
| all     | All Issues                                                      |
| default | Non-private Issues + private Issues created by/assigned to self |
| own     | Only Issues created by/assigned to self                         |

#### 6.3.3 Permission Check Flow

```
1. Check if user is admin → if yes, allow
2. Check if project exists and is not archived
3. Get user's roles in the project
   - Public project: use user roles or Non-Member/Anonymous role
   - Private project: must have member relationship
4. Check if role contains required permission
5. Check issues_visibility constraint (for Issue-related operations)
```

### 6.4 Permission Storage Format

The `roles.permissions` field uses JSON format storage:

```json
["view_issues", "add_issues", "edit_issues", "delete_issues"]
```

---

## 7. Initial Data

### 7.1 Roles (roles)

| id  | name       | position | builtin | issues_visibility | users_visibility            |
| --- | ---------- | -------- | ------- | ----------------- | --------------------------- |
| 1   | Manager    | 1        | 0       | all               | all                         |
| 2   | Developer  | 2        | 0       | all               | members_of_visible_projects |
| 3   | Reporter   | 3        | 0       | default           | members_of_visible_projects |
| 4   | Non-Member | 4        | 1       | default           | members_of_visible_projects |
| 5   | Anonymous  | 5        | 2       | default           | -                           |

### 7.2 Trackers (trackers)

| id  | name    | position | is_in_roadmap | default_status_id |
| --- | ------- | -------- | ------------- | ----------------- |
| 1   | Bug     | 1        | true          | 1                 |
| 2   | Feature | 2        | true          | 1                 |
| 3   | Support | 3        | false         | 1                 |

### 7.3 Issue Statuses (issue_statuses)

| id  | name        | position | is_closed | default_done_ratio |
| --- | ----------- | -------- | --------- | ------------------ |
| 1   | New         | 1        | false     | 0                  |
| 2   | In Progress | 2        | false     | null               |
| 3   | Resolved    | 3        | false     | 100                |
| 4   | Feedback    | 4        | false     | null               |
| 5   | Closed      | 5        | true      | 100                |
| 6   | Rejected    | 6        | true      | 100                |

### 7.4 Priorities (enumerations)

| id  | name      | position | is_default | type          | active |
| --- | --------- | -------- | ---------- | ------------- | ------ |
| 1   | Low       | 1        | false      | IssuePriority | true   |
| 2   | Normal    | 2        | true       | IssuePriority | true   |
| 3   | High      | 3        | false      | IssuePriority | true   |
| 4   | Urgent    | 4        | false      | IssuePriority | true   |
| 5   | Immediate | 5        | false      | IssuePriority | true   |

### 7.5 Administrator User (users)

| login | firstname | lastname | admin | status | language |
| ----- | --------- | -------- | ----- | ------ | -------- |
| admin | Admin     | User     | true  | 1      | en       |

> Password is specified by user during installation

### 7.6 Data Loading Method

Execute through SQL seed files during database initialization.

---

## 8. Frontend Design

### 8.1 Page Routes

```
/                           # Homepage (redirect to project list or login)
/login                      # Login page

/projects                   # Project list
/projects/new               # Create project
/projects/:id               # Project details
/projects/:id/edit          # Edit project
/projects/:id/issues        # Project Issue list
/projects/:id/issues/new    # Create Issue
/projects/:id/settings      # Project settings
/projects/:id/members       # Member management
/projects/:id/categories    # Category management

/issues/:id                 # Issue details
/issues/:id/edit            # Edit Issue

/issues                     # Global Issue list

/users                      # User list (admin)
/users/:id                  # User details
/users/:id/edit             # Edit user

/settings                   # System settings (admin)
/settings/roles             # Role management
/settings/trackers          # Tracker management
/settings/statuses          # Status management
/settings/priorities        # Priority management
```

### 8.2 Core Pages

| Page              | Description                                      |
| ----------------- | ------------------------------------------------ |
| Login Page        | User login                                       |
| Project List      | Display all visible projects                     |
| Project Details   | Project information, Issue statistics            |
| Issue List        | Filterable, sortable Issue list                  |
| Issue Details     | Issue information, notes, attachments, relations |
| Issue Form        | Create/edit Issue                                |
| Member Management | Project members and roles                        |
| User Management   | User CRUD (admin)                                |
| System Settings   | Enumeration management (admin)                   |

### 8.3 Core Components

| Component      | Description                                      |
| -------------- | ------------------------------------------------ |
| Layout         | Page layout (Header + Sidebar + Content)         |
| DataTable      | TanStack Table wrapper (sorting, pagination)     |
| IssueList      | Issue list component                             |
| IssueForm      | Issue form component (React Hook Form + Zod)     |
| IssueCard      | Issue card component                             |
| ProjectCard    | Project card component                           |
| MemberList     | Member list component                            |
| AttachmentList | Attachment list component                        |
| JournalList    | Change history component                         |
| Select         | Dropdown selector (Tracker/Status/Priority/User) |

### 8.4 Theme System

**Decision: CSS Variables Only**

- Use Tailwind CSS dark mode with `next-themes`
- Theme toggle in header
- CSS variables for colors (light/dark variants)
- No runtime theme customization in MVP

### 8.5 Responsive Design

**Decision: Desktop First**

- Primary target: desktop browsers (1280px+)
- Responsive adjustments for tablet (768px-1279px)
- Mobile support as secondary (below 768px)
- Data tables with horizontal scroll on smaller screens
- Sidebar collapses to hamburger menu on mobile

### 8.6 Rich Text Editor

**Decision: None (MVP)**

- Use plain textarea for description and notes
- Future iteration: Add lightweight Markdown editor (Tiptap)

---

## 9. Backend Design Decisions

### 9.1 Domain Entity Pattern

**Decision: Pure Domain Entities**

Domain entities are pure Rust structs with Serde traits, independent of any ORM. This provides:

- Clean separation between domain and persistence
- No database coupling in domain layer
- Easy to test business logic in isolation

```rust
// domain/entities/issue.rs
#[derive(Debug, Clone, Serialize, Deserialize)]
pub struct Issue {
    pub id: i32,
    pub project_id: i32,
    pub tracker_id: i32,
    pub subject: String,
    pub description: Option<String>,
    pub status_id: i32,
    pub priority_id: i32,
    // ... other fields
}
```

### 9.2 Repository Pattern

**Decision: Trait + Impl Separation**

Repository traits defined in domain layer, implementations in infrastructure:

```rust
// domain/repositories/issue_repository.rs
#[async_trait]
pub trait IssueRepository: Send + Sync {
    async fn find_by_id(&self, id: i32) -> Result<Option<Issue>>;
    async fn find_all(&self, params: IssueQueryParams) -> Result<Vec<Issue>>;
    async fn create(&self, issue: &CreateIssueDto) -> Result<Issue>;
    async fn update(&self, id: i32, issue: &UpdateIssueDto) -> Result<Issue>;
    async fn delete(&self, id: i32) -> Result<()>;
}

// infrastructure/persistence/repositories/issue_repository_impl.rs
pub struct SeaOrmIssueRepository {
    db: DatabaseConnection,
}

#[async_trait]
impl IssueRepository for SeaOrmIssueRepository {
    // Implementation using SeaORM entities
}
```

### 9.3 Use Case Organization

**Decision: One File Per Use Case**

Each use case is a separate file for modularity and clarity:

```rust
// application/use_cases/issue/create_issue.rs
pub struct CreateIssueUseCase<R: IssueRepository> {
    issue_repo: Arc<R>,
    project_repo: Arc<dyn ProjectRepository>,
    permission_service: Arc<dyn PermissionService>,
}

impl<R: IssueRepository> CreateIssueUseCase<R> {
    pub async fn execute(&self, dto: CreateIssueDto, user: &CurrentUser) -> Result<Issue> {
        // Business logic here
    }
}
```

### 9.4 Error Handling

**Decision: Thiserror Types**

Custom error types at each layer with From traits for conversion:

```rust
// domain/errors.rs
#[derive(Debug, thiserror::Error)]
pub enum DomainError {
    #[error("Entity not found: {0}")]
    NotFound(String),
    #[error("Validation failed: {0}")]
    Validation(String),
    #[error("Permission denied: {0}")]
    PermissionDenied(String),
}

// application/errors.rs
#[derive(Debug, thiserror::Error)]
pub enum ApplicationError {
    #[error("Domain error: {0}")]
    Domain(#[from] DomainError),
    #[error("Infrastructure error: {0}")]
    Infrastructure(String),
}

// presentation/errors.rs
#[derive(Debug, thiserror::Error)]
pub enum ApiError {
    #[error("{0}")]
    BadRequest(String),
    #[error("Unauthorized")]
    Unauthorized,
    #[error("Forbidden: {0}")]
    Forbidden(String),
    #[error("Not found: {0}")]
    NotFound(String),
    #[error("Validation error: {0}")]
    UnprocessableEntity(String),
    #[error("Internal server error")]
    Internal,
}

impl From<ApplicationError> for ApiError {
    fn from(err: ApplicationError) -> Self {
        match err {
            ApplicationError::Domain(DomainError::NotFound(msg)) => ApiError::NotFound(msg),
            ApplicationError::Domain(DomainError::PermissionDenied(msg)) => ApiError::Forbidden(msg),
            // ... other mappings
        }
    }
}
```

### 9.5 Nested Set Implementation

**Decision: Domain Service**

Nested set logic implemented as domain service for reusability:

```rust
// domain/services/nested_set_service.rs
pub struct NestedSetService;

impl NestedSetService {
    /// Calculate lft/rgt for new node
    pub fn calculate_position(parent_rgt: i32) -> (i32, i32);

    /// Update positions when moving node
    pub fn move_node(lft: i32, rgt: i32, new_parent_rgt: i32) -> NestedSetUpdate;

    /// Get all descendants
    pub fn get_descendants(root_lft: i32, root_rgt: i32) -> Vec<i32>;

    /// Check if node is ancestor of another
    pub fn is_ancestor_of(ancestor_lft: i32, ancestor_rgt: i32,
                          descendant_lft: i32, descendant_rgt: i32) -> bool;
}
```

### 9.6 DTO Organization

**Decision: Request/Response Split**

- Request DTOs in `application/dto/`
- Response DTOs in `presentation/dto/`

```rust
// application/dto/issue/create_issue_request.rs
#[derive(Debug, Deserialize)]
pub struct CreateIssueRequest {
    pub issue: CreateIssueDto,
}

#[derive(Debug, Deserialize, Validate)]
pub struct CreateIssueDto {
    #[validate(length(min = 1, max = 255))]
    pub subject: String,
    pub project_id: i32,
    pub tracker_id: i32,
    pub description: Option<String>,
    // ... other fields
}

// presentation/dto/issue_response.rs
#[derive(Debug, Serialize)]
pub struct IssueResponse {
    pub issue: IssueDetail,
}

#[derive(Debug, Serialize)]
pub struct IssueDetail {
    pub id: i32,
    pub subject: String,
    pub project: ProjectSummary,
    pub tracker: TrackerSummary,
    pub status: StatusSummary,
    // ... nested objects for includes
}
```

---

## 10. Security Policy

### 10.1 Password Policy

**Decision: Align with Redmine**

Password policy stored in `settings` table for runtime configuration:

| Setting                        | Type     | Default | Description                                 |
| ------------------------------ | -------- | ------- | ------------------------------------------- |
| password_min_length            | int      | 8       | Minimum password length                     |
| password_required_char_classes | string[] | []      | Required character classes                  |
| password_max_age               | int      | 0       | Maximum password age in days (0 = disabled) |

**Available Character Classes:**

| Class         | Description                                |
| ------------- | ------------------------------------------ |
| uppercase     | At least one uppercase letter (A-Z)        |
| lowercase     | At least one lowercase letter (a-z)        |
| digits        | At least one digit (0-9)                   |
| special_chars | At least one special character (!@#$%^&\*) |

**Validation Logic:**

```rust
pub fn validate_password(password: &str, settings: &PasswordSettings) -> Vec<String> {
    let mut errors = Vec::new();

    if password.len() < settings.min_length {
        errors.push(format!("Password must be at least {} characters", settings.min_length));
    }

    for class in &settings.required_char_classes {
        match class.as_str() {
            "uppercase" if !password.chars().any(|c| c.is_uppercase()) => {
                errors.push("Password must contain at least one uppercase letter".to_string());
            }
            "lowercase" if !password.chars().any(|c| c.is_lowercase()) => {
                errors.push("Password must contain at least one lowercase letter".to_string());
            }
            "digits" if !password.chars().any(|c| c.is_ascii_digit()) => {
                errors.push("Password must contain at least one digit".to_string());
            }
            "special_chars" if !password.chars().any(|c| SPECIAL_CHARS.contains(c)) => {
                errors.push("Password must contain at least one special character".to_string());
            }
            _ => {}
        }
    }

    errors
}
```

### 10.2 Authentication

**Decision: JWT Only (MVP)**

- JWT token with configurable expiration (default: 24 hours)
- Token stored in HTTP-only cookie (recommended) or Authorization header
- No API key support in MVP (future iteration)

### 10.3 Two-Factor Authentication

**Decision: None (MVP)**

- 2FA support deferred to future iteration
- Database fields for 2FA remain in schema (twofa_scheme, twofa_totp_key, etc.)

---

## 11. Testing Strategy

### 11.1 Test Coverage Requirement

**Decision: 80% Minimum**

All PRs must pass coverage check. Coverage measured by:

- Backend: `cargo tarpaulin`
- Frontend: `vitest --coverage`

### 11.2 Test Scope

**Decision: Balanced**

Equal focus on:

- Unit tests for backend domain/application layers
- Unit tests for frontend components/hooks
- Integration tests for API endpoints
- E2E tests for critical user flows

### 11.3 Backend Testing

| Test Type   | Tool          | Scope                                  |
| ----------- | ------------- | -------------------------------------- |
| Unit Tests  | Built-in test | Domain entities, services, use cases   |
| Integration | Built-in test | Repository implementations, API routes |
| API Tests   | Reqwest       | Full HTTP request/response testing     |

### 11.4 Frontend Testing

| Test Type       | Tool                     | Scope                             |
| --------------- | ------------------------ | --------------------------------- |
| Unit Tests      | Vitest                   | Utility functions, hooks          |
| Component Tests | Vitest + Testing Library | Component rendering, interactions |
| E2E Tests       | Playwright               | Critical user flows               |

### 11.5 Critical Test Scenarios

**Backend:**

- [ ] User authentication (login, logout, token validation)
- [ ] Permission checking (all permission types)
- [ ] Issue CRUD with permission checks
- [ ] Nested set operations (project/issue hierarchy)
- [ ] File upload and download

**Frontend:**

- [ ] Login flow
- [ ] Project creation and navigation
- [ ] Issue creation with form validation
- [ ] Issue list filtering and sorting
- [ ] Member management

---

## 12. Technical Implementation Details

### 12.1 File Storage

**Decision: Local Disk Only (MVP), Fully Aligned with Redmine**

The file storage structure is identical to Redmine for seamless data migration.

**Storage Path Configuration:**

- Default: `{APP_ROOT}/files/`
- Configurable via `attachments_storage_path` setting

**Directory Structure:**

```
files/
├── 2025/
│   ├── 11/
│   │   ├── 251118135304_abc123_document.pdf
│   │   └── 251118140530_def456_image.png
│   └── 12/
│       └── 251203134922_ghi789_report.docx
└── 2026/
    ├── 01/
    │   └── 260101121738_jkl012_data.csv
    └── 03/
        └── 260320103045_mno345_config.yml
```

**File Naming Convention:**

Files are named using the format: `{timestamp}_{ascii_filename}`

| Component      | Description                                  |
| -------------- | -------------------------------------------- |
| timestamp      | `YYMMDDHHMMSS` format (e.g., `260320103045`) |
| ascii_filename | Sanitized original filename                  |

**Filename Sanitization Rules:**

1. Extract only the filename (remove path components)
2. Replace invalid characters (`/ ? % * : | " ' < > \n \r`) with underscore (`_`)
3. If filename contains only ASCII alphanumeric characters, underscores, hyphens, and dots, and length ≤ 50: use as-is
4. Otherwise: hash the filename and append the extension

**Examples:**

```
document.pdf          → 260320103045_document.pdf
my file (1).txt       → 260320103045_my_file__1_.txt
中文文件.pdf           → 260320103045_a1b2c3d4e5f6.pdf
very-long-filename... → 260320103045_abc123def456.pdf
```

**Database Fields:**

| Field            | Description                                       |
| ---------------- | ------------------------------------------------- |
| `disk_directory` | Relative directory path (e.g., `2026/03`)         |
| `disk_filename`  | Disk filename (e.g., `260320103045_document.pdf`) |
| `filename`       | Original filename                                 |
| `digest`         | SHA256 hash of file content                       |
| `filesize`       | File size in bytes                                |

**Full Disk Path:**

```
{STORAGE_PATH}/{disk_directory}/{disk_filename}
```

**Implementation:**

```rust
// infrastructure/storage/local_storage.rs
pub struct LocalFileStorage {
    base_path: PathBuf,
}

impl LocalFileStorage {
    /// Create a new file and return the disk filename
    pub fn create_diskfile(&self, filename: &str, directory: Option<&str>) -> Result<(File, String)>;

    /// Save file content and return (disk_directory, disk_filename, digest)
    pub fn save(&self, filename: &str, content: &[u8]) -> Result<(String, String, String)>;

    /// Load file content by disk_directory and disk_filename
    pub fn load(&self, disk_directory: &str, disk_filename: &str) -> Result<Vec<u8>>;

    /// Delete file from disk
    pub fn delete(&self, disk_directory: &str, disk_filename: &str) -> Result<()>;

    /// Get full disk path
    pub fn diskfile(&self, disk_directory: &str, disk_filename: &str) -> PathBuf;

    /// Sanitize filename for disk storage
    fn sanitize_filename(&self, filename: &str) -> String;

    /// Generate target directory based on current date
    fn target_directory(&self) -> String;
}
```

### 12.2 File Upload

**Decision: Simple Upload (MVP)**

- Single-file upload via multipart/form-data
- Maximum file size: 10MB (configurable)
- No chunked upload or resume support

### 12.3 Rate Limiting

**Decision: None (MVP)**

- No rate limiting in MVP
- Add in future iteration if needed
- Consider `governor` crate for implementation

### 12.4 Caching

**Decision: None (MVP)**

- No caching in MVP
- HTTP caching headers (ETag, Cache-Control) can be added later
- Consider in-memory cache (moka crate) for frequently accessed data

### 12.5 Logging

**Decision: Structured to Stdout**

```rust
// Using tracing for structured logging
use tracing::{info, error, instrument};
use tracing_subscriber::fmt::format::FmtContext;

#[instrument(skip(db))]
async fn create_issue(db: &DatabaseConnection, dto: CreateIssueDto) -> Result<Issue> {
    info!(project_id = dto.project_id, "Creating issue");
    // ...
}
```

**Log Format (JSON):**

```json
{
  "timestamp": "2026-03-19T10:30:00Z",
  "level": "INFO",
  "target": "rsmine::api::issues",
  "message": "Creating issue",
  "project_id": 1
}
```

### 12.6 Health Endpoints

**Decision: Included in MVP**

Health endpoints for service monitoring and orchestration.

| Endpoint      | Purpose                                                                 |
| ------------- | ----------------------------------------------------------------------- |
| `GET /health` | Returns 200 if service is running                                       |
| `GET /ready`  | Returns 200 if service is ready to accept requests (database connected) |

**Response Format:**

```json
{
  "status": "ok",
  "timestamp": "2026-03-20T10:30:00Z"
}
```

**Implementation:**

```rust
// presentation/api/v1/health.rs
pub async fn health() -> impl IntoResponse {
    Json(HealthResponse {
        status: "ok".to_string(),
        timestamp: chrono::Utc::now().to_rfc3339(),
    })
}

pub async fn ready(State(db): State<DatabaseConnection>) -> impl IntoResponse {
    // Check database connectivity
    match db.ping().await {
        Ok(_) => Json(HealthResponse {
            status: "ok".to_string(),
            timestamp: chrono::Utc::now().to_rfc3339(),
        }),
        Err(_) => (
            StatusCode::SERVICE_UNAVAILABLE,
            Json(HealthResponse {
                status: "error".to_string(),
                timestamp: chrono::Utc::now().to_rfc3339(),
            }),
        ).into_response(),
    }
}
```

### 12.7 Prometheus Metrics

**Decision: None (MVP)**

- No Prometheus metrics in MVP
- Consider `axum-prometheus` for future iteration

---

## 13. Deployment and Configuration

### 13.1 Environment Variables

#### 13.1.1 Configuration Management

Rsmine uses a comprehensive configuration management system that supports both environment variables and configuration files (TOML), with carefully designed defaults.

**Priority Order:**

```
Environment Variables > Configuration File > Default Values
```

**Configuration File:**

Default location: `config.toml` (can be overridden via `RSMINE_CONFIG_PATH`)

```toml
# config.toml

[server]
host = "0.0.0.0"
port = 3000

[database]
url = "sqlite://./data/rsmine.db"
# url = "postgres://user:pass@localhost:5432/rsmine"
# url = "mysql://user:pass@localhost:3306/rsmine"
max_connections = 10

[jwt]
secret = ""  # Required: set via environment variable or config file
expiration = 86400  # seconds (24 hours)

[storage]
path = "./data/files"
max_file_size = 10485760  # bytes (10MB)

[logging]
level = "info"  # trace, debug, info, warn, error
format = "json"  # json, pretty

[password]
min_length = 8
max_age = 0  # days (0 = disabled)
# required_char_classes = ["uppercase", "lowercase", "digits", "special_chars"]
```

**Environment Variables Format:**

All environment variables are prefixed with `RSMINE_`. Nested configuration uses double underscores (`__`) as separator.

| Environment Variable               | Config Path              | Default                     |
| ---------------------------------- | ------------------------ | --------------------------- |
| `RSMINE_SERVER__HOST`              | server.host              | `0.0.0.0`                   |
| `RSMINE_SERVER__PORT`              | server.port              | `3000`                      |
| `RSMINE_DATABASE__URL`             | database.url             | `sqlite://./data/rsmine.db` |
| `RSMINE_DATABASE__MAX_CONNECTIONS` | database.max_connections | `10`                        |
| `RSMINE_JWT__SECRET`               | jwt.secret               | _(required)_                |
| `RSMINE_JWT__EXPIRATION`           | jwt.expiration           | `86400`                     |
| `RSMINE_STORAGE__PATH`             | storage.path             | `./data/files`              |
| `RSMINE_STORAGE__MAX_FILE_SIZE`    | storage.max_file_size    | `10485760`                  |
| `RSMINE_LOGGING__LEVEL`            | logging.level            | `info`                      |
| `RSMINE_LOGGING__FORMAT`           | logging.format           | `json`                      |
| `RSMINE_PASSWORD__MIN_LENGTH`      | password.min_length      | `8`                         |
| `RSMINE_PASSWORD__MAX_AGE`         | password.max_age         | `0`                         |
| `RSMINE_CONFIG_PATH`               | _(config file path)_     | `./config.toml`             |

**Example Environment Variables:**

```bash
export RSMINE_SERVER__HOST=0.0.0.0
export RSMINE_SERVER__PORT=3000
export RSMINE_DATABASE__URL=postgres://user:pass@localhost:5432/rsmine
export RSMINE_JWT__SECRET=your-secret-key-here
export RSMINE_STORAGE__PATH=/var/lib/rsmine/files
export RSMINE_LOGGING__LEVEL=debug
```

**Legacy Environment Variables (Supported for Compatibility):**

For backward compatibility, the following legacy format is also supported:

```bash
# Legacy format (still works)
SERVER_HOST=0.0.0.0
SERVER_PORT=3000
DATABASE_URL=sqlite://./data/rsmine.db
JWT_SECRET=your-secret-key
JWT_EXPIRATION=86400
STORAGE_PATH=./data/files
MAX_FILE_SIZE=10485760
RUST_LOG=info
```

**Implementation:**

```rust
// config/app_config.rs
use config::{Config, ConfigError, File, Environment};
use serde::Deserialize;

#[derive(Debug, Deserialize, Clone)]
pub struct AppConfig {
    pub server: ServerConfig,
    pub database: DatabaseConfig,
    pub jwt: JwtConfig,
    pub storage: StorageConfig,
    pub logging: LoggingConfig,
    pub password: PasswordConfig,
}

#[derive(Debug, Deserialize, Clone)]
pub struct ServerConfig {
    #[serde(default = "default_host")]
    pub host: String,
    #[serde(default = "default_port")]
    pub port: u16,
}

#[derive(Debug, Deserialize, Clone)]
pub struct JwtConfig {
    pub secret: String,
    #[serde(default = "default_jwt_expiration")]
    pub expiration: u64,
}

// ... other config structs

impl AppConfig {
    pub fn load() -> Result<Self, ConfigError> {
        let config_path = std::env::var("RSMINE_CONFIG_PATH")
            .unwrap_or_else(|_| "./config.toml".to_string());

        let config = Config::builder()
            // Load from config file
            .add_source(File::with_name(&config_path).required(false))
            // Load from environment variables (RSMINE_ prefix, __ separator)
            .add_source(Environment::with_prefix("RSMINE").separator("__"))
            .build()?;

        config.try_deserialize()
    }
}

fn default_host() -> String { "0.0.0.0".to_string() }
fn default_port() -> u16 { 3000 }
fn default_jwt_expiration() -> u64 { 86400 }
```

#### 13.1.2 Frontend Environment Variables

```bash
# API Address
NEXT_PUBLIC_API_URL=http://localhost:3000/api/v1
```

### 13.2 Installation Process

1. **Install Backend Dependencies**

```bash
cd backend
cargo build --release
```

2. **Install Frontend Dependencies**

```bash
cd web
npm install
npm run build
```

3. **Initialize Database**

```bash
./rsmine db:migrate
./rsmine db:seed
```

4. **Create Administrator User**

```bash
./rsmine install
# Interactively enter admin password
```

5. **Start Services**

```bash
# Start backend
./rsmine serve

# Start frontend
cd web && npm start
```

---

## Appendix

> **Note:** Future iterations have been moved to [FUTURE_ITERATIONS.md](./FUTURE_ITERATIONS.md).

### A. Relation Type Descriptions

| Type       | Description | Directionality                          |
| ---------- | ----------- | --------------------------------------- |
| relates    | Related     | Bidirectional                           |
| duplicates | Duplicates  | Bidirectional                           |
| blocks     | Blocks      | Unidirectional                          |
| precedes   | Precedes    | Unidirectional (can include delay days) |
| follows    | Follows     | Unidirectional (can include delay days) |
| copied_to  | Copied to   | Unidirectional                          |

### B. Project Status Descriptions

| Value | Status   | Description |
| ----- | -------- | ----------- |
| 1     | Active   | Active      |
| 5     | Closed   | Closed      |
| 9     | Archived | Archived    |

### C. User Status Descriptions

| Value | Status     | Description                     |
| ----- | ---------- | ------------------------------- |
| 1     | Active     | Active                          |
| 2     | Registered | Registered (pending activation) |
| 3     | Locked     | Locked                          |

---

_End of Document_
