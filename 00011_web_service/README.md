Based on https://blog.logrocket.com/create-an-async-crud-web-service-in-rust-with-warp/

## What I do not like here

- GET `/todo/{id}` does not work as expected - it is returning an array
- PUT `/todo/{id}` requires name "and" checked inside the body
- missing unit tests (jetbrains rest client tests are not enough)
