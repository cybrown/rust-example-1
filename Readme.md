# Rust sample api

Example api project using Rust.

## File architecture

    /migrations
        sql schema to run with diesel-cli

    /domain
        Implementation of the business rules

    /db
        Sqlx database adapter

    /api
        Api entry points calling the domain

    /commands
        Command line utilities calling the domain

    /db_diesel
        Database access for pg using diesel

    /main
        /adapters
            Adapters wrapping external libraries to implement traits provided by the domain
            Also converts internal dependencies such as database access

        /main.rs
            Entry point of the application

        /service_registry.rs
            Factories to create the concrete types of many dependencies

        /util.rs
            Some utility functions

    /atomic_counter
        A simple thread safe counter

    /println_logger
        A simple logger based on println!

    /simple_counter
        A counter which is not thread safe.
        Used to illustrate how to use an adapter with an Arc and a Mutex

    /uppercaser
        A simple stateless library

## Goals

### Dependency injection
* [x] Create services and wire them together
* [x] Use injection through generic
* [x] Use injection through dyn Trait
* [x] Each services can be injected in many consumers
* [x] Each services can hold a mutable state
* [x] Service consumers must expose a trait
* [x] Services should not explicitely implement their consumers traits
* [x] Adapters wrap services into a struct implementing the expected trait

### Unit testing
* [x] Run unit tests with equality assertions
* [x] Create mocks implementing a trait returning a predefined value
* [ ] Generate a coverage report
* [x] Inject mocks with spies to test wether a dependency was called

### Project architecture
* [x] Split one project in multiple files
* [x] Split in multiple projects
* [x] Use cargo workspaces

### Error handling
* [x] Seperate error between services and consumers
* [ ] Try anyhow
* [ ] Try thiserror
* [ ] Enrich errors with origin
* [x] Enrich errors with a message

### Logging
* [ ] Add logs

### Database (diesel)
* [x] Setup diesel
* [x] Run at least one data query with diesel
* [x] Run at least one data update query with diesel
* [x] Run sql queries in dedicated threads for blocking apis
* [ ] Transactions (not available with an async workflow)

### Database (sqlx)
* [x] Setup sqlx
* [x] Run at least one data query with sqlx
* [x] Run at least one data update query with sqlx
* [ ] Transactions

### HTTP Server
* [x] Setup a web framework to return json
* [x] Route to get all posts
* [x] Route to create a post
* [x] Route to publish a post
* [x] Route to unpublish a post
* [ ] Investigate open api v3
* [ ] Return meaningfull errors for 4** status codes
* [x] Return 500 on internal errors
* [ ] Return a request id in a header

### Configuration
* [x] Environment variables
* [x] Read a configuration file in many formats
* [ ] Read cli args
* [ ] Use secrets from a cloud provider or vault

### Continuous integration
* [ ] Check formating
* [ ] Run additional static analysis tools
* [ ] Investigate how to get standard unit tests reports
* [ ] Investigate how to get standard coverage reports for unit tests
* [ ] Investigate how to get standard integration tests reports
* [ ] Investigate how to get standard coverage reports for integration tests

### HTTP Client

### Sockets

### File
