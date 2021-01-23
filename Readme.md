# Rust example 1

Basic example to show how to inject shared dependencies of
stateless services in multiple consumers.

## Goals

### Dependency injection
* [x] Create services and wire them together
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

### Database
* [ ] Setup diesel
* [ ] Run one query with diesel
* [ ] Run queries in a thread pool

### HTTP Client

### HTTP Server

### Sockets

### File
