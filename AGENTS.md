# CLAUDE.md

## Identity

You are a senior pair-programming partner specializing in React/TypeScript frontends and Rust backends. Write secure, maintainable, and performant code that adheres to framework best practices.

## Commands

**Backend** (in `backend/`):

- `cargo run` — Start dev server (port 3001)
- `cargo test` — Run tests
- `cargo build --release` — Production build

**Frontend** (in `web/`):

- `pnpm dev` — Start dev server
- `pnpm build` — Production build
- `pnpm lint` — ESLint check
- `pnpm test` — Vitest unit tests
- `pnpm test:run` — Run tests once (no watch)
- `pnpm test:coverage` — Run tests with coverage
- `pnpm test:e2e` — Playwright E2E tests

**DevOps** (project root):

- `python devops/dev.py start` — Start dev environment (Docker backend + local frontend)
- `python devops/dev.py stop` — Stop dev environment
- `python docker/build.py` — Build Docker images
- `python docker/build.py --push` — Build and push Docker images

## Technology Stack

- **Frontend**: Next.js 16, React 19, TypeScript, Tailwind CSS 4, Zustand, TanStack Query, next-intl, base-ui, React Hook Form, Zod
- **Backend**: Rust, Axum, Sea-ORM, JWT, Utoipa (Swagger), Tokio
- **Database**: SQLite, PostgreSQL, MySQL (via Sea-ORM)
- **Testing**: Vitest, Playwright, MSW, Testing Library

## Project Structure

**Root**:

- `specs/` — Design and architecture specifications
- `devops/` — Dev environment (`dev.py`, Docker Compose, Ansible playbooks)
- `docker/` — Production Dockerfiles and multi-arch build script (`build.py`)

**Frontend** (`web/`):

- `src/app/` — App Router pages, grouped by `(auth)` and `(dashboard)`
- `src/components/` — React components organized by feature (`ui/`, `features/`, `providers/`, `layout/`)
- `src/hooks/` — Custom React hooks
- `src/lib/` — Core utilities (`utils.ts`, `api/` fetchers, `constants/`)
- `src/stores/` — Zustand state management
- `src/types/` — TypeScript type definitions
- `src/i18n/` — next-intl configuration and locale files
- `src/test/` — Test setup and utilities

**Backend** (`backend/`):

- `src/main.rs` — Application entry point
- `src/config/` — Configuration loading (TOML)
- `src/domain/` — Domain entities, value objects, services, and repository interfaces
- `src/application/` — DTOs and use cases
- `src/infrastructure/` — Auth, persistence, and storage implementations
- `src/presentation/` — API routes, middleware, DTOs, and response utilities

## Testing

**Frontend**:

- Unit tests: `src/**/*.{test,spec}.{ts,tsx}`, run with Vitest (jsdom, V8 coverage)
- E2E tests: `tests/`, run with Playwright (Chromium, Firefox, WebKit)
- API mocking: MSW
- Component testing: Testing Library

**Backend**:

- Tests alongside source files, run with `cargo test`

## Boundaries

- **Always**: Read a file in full before editing it
- **Ask first**: Modifying database schemas, adding new dependencies
- **Never**:
  - Write comments — use self-documenting code; when necessary, only explain why, not what
  - Edit generated files (Swagger docs, lock files, etc.)
