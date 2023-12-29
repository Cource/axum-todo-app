# Simple todo api implementation
This is just a toy project I made to test out how I would go about making an http server in rust.

## Run / Install
you need cargo for compiling this project. get cargo at: [rustdocs](https://doc.rust-lang.org/cargo/getting-started/installation.html)  
```sh
cargo run
```

## API Documentation
This is a simple application and therefore just uses one route with the multiple **REST** methods, and runs on port 3000 on all available interfaces(0.0.0.0), so you can access it from localhost:3000.
### Endpoints / Routes
```
Route  Method  JsonBody
/      GET
/      POST    {id: number ,name string, is_completed: bool}
/:id   DELETE
/:id   PATCH   {SetStatus: bool} or {ChangeName: string}
```
#### GET
retrieves all the available todos  
#### POST
For adding a new todo, takes the json parameters mentioned above
#### DELETE
Has to be called on the id route so like: `DELETE localhost:3000/2` to delete the todo with id of 2
#### PATCH
Also has to called with the id param like DELETE and takes the parameters mentioned above
