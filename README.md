# {{ project-name }}

An opinionated starter project for axum with sqlx, postgres, tinyid, anyhow, tracing output, and EnvConfig for loading configuration.

The structure of the source:

```
src
├── infra      // Things necessary to run the service, tracing, config, app state, etc
├── models     // Domain models and other types to support what's in the database
├── queries    // Wrappers around the database queries that return the domain models
├── migrations // Database migrations
└── routes     // Controllers and Subrouters
```
