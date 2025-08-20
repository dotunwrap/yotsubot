# YotsuBot

A self-hostable Discord bot built with Rust using Serenity and Poise, designed for streamlined member verification and periodic re-verification. Deploy-ready on [Shuttle](https://shuttle.dev) with a minimal configuration via Secrets.toml.

## Table of Contents

- [Features](#features)
- [Requirements](#requirements)
- [Setup](#setup)
- [Deployment](#deployment)
- [Commands](#commands)
- [Development Notes](#development-notes)
- [Contributing](#contributing)

## Features

-  Slash commands with Poise on top of Serenity
-  Role-based verification workflow
-  Weekly activity-based re-verification with exclusions
-  Single-guild scoped via configuration
-  Self-hosted only (no public SaaS)

## Requirements

-  Rust (stable)
-  Cargo
-  A Discord bot application and token
-  Shuttle CLI
-  A single Discord guild (server) to operate in

## Setup

1) Clone and install prerequisites
-  [Install Rust](https://www.rust-lang.org/tools/install)
-  [Install Shuttle CLI](https://docs.shuttle.dev/getting-started/installation)

2) Create your Discord bot application
-  Create an application at https://discord.com/developers/applications
-  Add a Bot user and copy the token
-  Enable the necessary Privileged Gateway Intents:
  - Server Members Intent
  - Message Content Intent
-  Invite the bot to your server with appropriate role permissions:
  - Manage Roles
  - View Channels
  - Read Message History
  - Send Messages
  - Use Slash Commands

3) Configure Secrets
-  Copy Secrets.toml.example to Secrets.toml
-  Fill in the values

Notes:
-  ID values must be Discord snowflakes (as strings).
-  The reverify exclusion lists will be respected to prevent removal of verification from certain users or users with certain roles.

## Deployment (Shuttle)

-  Ensure you’re logged in: shuttle login
-  Deploy:
  - shuttle deploy
-  Check logs:
  - shuttle logs -f

## Commands

Slash commands available in the allowed guild:

-  /verify <member>
  - Removes the NEW_MEMBER_ROLE_ID from the target member and grants VERIFIED_ROLE_ID.

-  /reverify_active_users <num_users>
  - Un-verifies all members except the top <num_users> active users for the week.
  - Respects REVERIFY_EXCLUDED_ROLE_IDS and REVERIFY_EXCLUDED_USER_IDS to ensure excluded users/roles are not affected.

## Development Notes

-  Built with Serenity + Poise for ergonomic slash commands in Rust.
-  Deployed via Shuttle for serverless-style hosting.
-  Single-guild guardrail enforced by ALLOWED_GUILD_ID.

## Contributing

Issues and PRs are welcome. Please avoid sharing any secrets or server-specific IDs in public reports.
