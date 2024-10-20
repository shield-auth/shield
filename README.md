# Shield: An Advanced IAM and CIAM Solution

_Not ready for production use yet. Please use with caution._

![Shield Hero](https://raw.githubusercontent.com/shield-auth/shield/refs/heads/trunk/assets/images/shield-hero.png)

Rust-based Open Source Identity and Access Management

[![Website](https://img.shields.io/badge/website-shield.rs-cyan)](https://shield.rs)
[![Language](https://img.shields.io/badge/language-Rust-orange.svg)](https://github.com/AutomationTank/shield/search?l=rust)
[![Documentation](https://img.shields.io/badge/🛡️-docs.shield.rs-cyan)](https://docs.shield.rs/)
[![Crate Docs](https://img.shields.io/badge/docs-docs.rs-orange?logo=rust)](https://docs.rs/rust-shield)
[![Crates.io](https://img.shields.io/crates/d/rust-shield)](https://crates.io/crates/rust-shield)
[![Discord](https://img.shields.io/discord/1159247000093609994?logo=discord)](https://discord.gg/KtYeDyBm)
[![Twitter Follow](https://img.shields.io/twitter/follow/shield_auth)](https://twitter.com/shield_auth)

<div align="center">

| Branch                                                |                                                                                                                            Status                                                                                                                            |                                                                        Coverage                                                                        |
| :---------------------------------------------------- | :----------------------------------------------------------------------------------------------------------------------------------------------------------------------------------------------------------------------------------------------------------: | :----------------------------------------------------------------------------------------------------------------------------------------------------: |
| ![trunk](https://img.shields.io/badge/trunk-orange)   |   [![CircleCI](https://dl.circleci.com/status-badge/img/circleci/PKLAPqiFRA5ffRQTi5dtfY/HvBHutnD43T9HBYcqAWgD4/tree/trunk.svg?style=svg)](https://dl.circleci.com/status-badge/redirect/circleci/PKLAPqiFRA5ffRQTi5dtfY/HvBHutnD43T9HBYcqAWgD4/tree/trunk)   |  [![Codecov](https://codecov.io/gh/AutomationTank/shield/branch/trunk/graph/badge.svg?token=1S0S4T1Z1J)](https://codecov.io/gh/AutomationTank/shield)  |
| ![develop](https://img.shields.io/badge/develop-blue) | [![CircleCI](https://dl.circleci.com/status-badge/img/circleci/PKLAPqiFRA5ffRQTi5dtfY/HvBHutnD43T9HBYcqAWgD4/tree/develop.svg?style=svg)](https://dl.circleci.com/status-badge/redirect/circleci/PKLAPqiFRA5ffRQTi5dtfY/HvBHutnD43T9HBYcqAWgD4/tree/develop) | [![Codecov](https://codecov.io/gh/AutomationTank/shield/branch/develop/graph/badge.svg?token=1S0S4T1Z1J)](https://codecov.io/gh/AutomationTank/shield) |

</div>

## About Shield 🔮

Shield is a robust, multi-tenant authentication and authorization solution
developed by [Mukesh Singh](https://linkedin.com/in/ca-mksingh) for modern
age applications. It provides a comprehensive set of features to secure your
applications and manage user access effectively.

## Key Features 🔑

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

> Coming Soon...
>
> - Two-factor Authentication (2FA)
> - OAuth2 Support
> - OpenID Connect Support
> - SAML Support

## Documentation 📑

For detailed documentation, visit [docs.shield.rs](https://docs.shield.rs/).

## Development 🛠️

### 1. Clone the repository

```bash
git clone https://github.com/shield-auth/shield.git
cd shield
```

### 2. Set Up Environment Variables

```bash
cp .env.example .env
```

### 3. Start the applications

```bash
cargo run
```

👉 _Note: You can get the default credentials in terminal or on later stage you can find it in `./logs/default_cred.json`_

### Sea-ORM Entity Generation Command

To generate Sea-ORM entities, use the following command:

```bash
sea-orm-cli generate entity -o entity/src/models --with-serde both --enum-extra-attributes 'serde(rename_all = "snake_case")'
```

## Contributing 🤝

[Add information about how to contribute to the project]

## License 📜

This project is dual-licensed under the following terms (see the LICENSE file for details):

- [MIT License](https://raw.githubusercontent.com/shield-auth/shield/refs/heads/trunk/LICENSE)
- [Apache License, Version 2.0](https://raw.githubusercontent.com/shield-auth/shield/refs/heads/trunk/LICENSE-APACHE)

Users are free to choose the license that best suits their needs.

## Contact 📞

- Website: [shield.rs](https://shield.rs)
- X: [@shield_auth](https://x.com/shield_auth)
- Discord: [Join our community](https://discord.gg/KtYeDyBm)

---

An open source initiative with ❤️ by [Mukesh Singh](https://linkedin.com/in/ca-mksingh)
