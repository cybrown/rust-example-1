# Rust example 1

Basic example to show how to inject shared dependencies of
stateless services in multiple consumers.

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

### Error handling
* [x] Seperate error between services and consumers
* [ ] Try anyhow
* [ ] Try thiserror
* [ ] Enrich errors with origin
* [ ] Enrich errors with a message

### Logging
* [ ] Add logs

### Database
* [x] Setup diesel
* [x] Run at least one data query with diesel
* [x] Run at least one data update query with diesel
* [ ] Transactions
* [x] Run sql queries in a seperate thread pool

### HTTP Server
* [x] Setup a web framework to return json
* [x] Route to get all posts
* [x] Route to create a post
* [ ] Route to publish a post
* [ ] Route to unpublish a post
* [ ] Investigate open api v3
* [ ] Return meaningfull errors

### Command line arguments

### Environment variables

### HTTP Client

### Sockets

### File
