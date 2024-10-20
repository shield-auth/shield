# Shield: An Advanced IAM and CIAM Solution

_Not ready for production use yet. Please use with caution._

![Introduction](https://raw.githubusercontent.com/shield-auth/shield/refs/heads/trunk/assets/images/shield-hero.png)

Shield is a robust, multi-tenant authentication and authorization solution
developed by [Mukesh Singh](https://linkedin.com/in/ca-mksingh) for modern
age applications. It provides a comprehensive set of features to secure your
applications and manage user access effectively.

## Key Features

- **Multi-tenant Support:** Manage multiple organizations or projects within a
  single instance.
- **User Management:** Efficiently handle user accounts and permissions.
- **Role-based Access Control (RBAC):** Define and manage user roles and permissions.
- **Session Management:** Secure handling of user sessions.
- **API Key Support:** Generate and manage API keys for secure programmatic access.

  - API Key Rotation
  - Rate Limiting
  - Expiration
  - Blacklisting and Whitelisting
  - Revocation

  <!-- Future Features
    - Two-factor Authentication (2FA)
    - OAuth2 Support
    - OpenID Connect Support
    - SAML Support
  -->

## Getting Started

### Prerequisites

Before you begin, ensure you have the following installed:

- [Docker](https://docs.docker.com/get-docker/)
- [Docker Compose](https://docs.docker.com/compose/install/)

### Installation

#### 1. Clone the repository

```bash
git clone https://github.com/shield-auth/shield.git
cd shield
```

#### 2. Set Up Environment Variables

```bash
cp .env.example .env
```

#### 3. Build and Start the Containers

```bash
docker compose up -d --build --wait
```

#### 4. Retrieve Default credentials

The default credentials are saved in `/usr/local/bin/logs/default_cred.json.
To view them:

```bash
docker exec shield-shield-1 cat /usr/local/bin/logs/default_cred.json
```

_Note: If the above command doesn't work, use `docker ps` to find the correct
container ID for the shield container._

### Resource Initialization Flow

The following diagram illustrates the resource initialization process:
![Resource Initialization Flow Chart](https://raw.githubusercontent.com/shield-auth/shield/refs/heads/trunk/assets/images/flow-charts/1-shield-start-transparent.svg)

### Usage Guide

#### Admin Login

To log in as an admin, use the following endpoint:

`{YOUR-SHIELD-URL}/realms/:realm_id/clients/:client_id/admin-login`

Replace `:realm_id` with your realm ID and `:client_id` with your client ID.

Example curl command:

```bash
curl -X POST \
  https://shield.example.com/realms/:realm_id/clients/:client_id/admin-login \
  -H 'Content-Type: application/json' \
  -d '{
    "email": "admin@admin.com",
    "password": "12345"
  }'
```

### Admin Login Flow

The following diagram illustrates the admin login process:
![Admin Login Flow Chart](https://raw.githubusercontent.com/shield-auth/shield/refs/heads/trunk/assets/images/flow-charts/2-admin-login-transparent.svg)
