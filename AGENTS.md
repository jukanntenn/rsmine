## Commands

### Backend
| Command | Description |
|---------|-------------|
| `cd backend && cargo run` | Start backend dev server (port 3001) |
| `cd backend && cargo test` | Run backend tests |
| `cd backend && cargo build --release` | Build backend for production |

### Frontend
| Command | Description |
|---------|-------------|
| `cd web && pnpm dev` | Start frontend dev server |
| `cd web && pnpm build` | Build frontend for production |
| `cd web && pnpm test` | Run frontend unit tests |
| `cd web && pnpm test:run` | Run frontend unit tests (single run) |
| `cd web && pnpm test:coverage` | Run frontend tests with coverage |
| `cd web && pnpm test:e2e` | Run frontend E2E tests (Playwright) |
| `cd web && pnpm lint` | Lint frontend code |

### Docker
| Command | Description |
|---------|-------------|
| `docker/build.sh` | Build images locally (host architecture) |
| `PUSH=true docker/build.sh` | Build and push images to registry |

### Deploy
| Command | Description |
|---------|-------------|
| `ANSIBLE_STDOUT_CALLBACK=yaml ansible-playbook -i devops/ansible/hosts.yml devops/ansible/main.yml --vault-password-file ~/.ansible-vault/rsmine.pwd` | Deploy via Ansible |

> **Note:** The vault password file path (`~/.ansible-vault/rsmine.pwd`) is a local convention — create this file on your machine before deploying.
