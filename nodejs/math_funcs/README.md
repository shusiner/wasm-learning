# Math Functions webapp example

In this example, we demonstrate how to use JSON to call Rust functions to call various math functions in SSVM from JavaScript. Read about [Getting started with Rust functions in Node.js](https://www.secondstate.io/articles/getting-started-with-rust-function/).

## Prerequisites

If you have not done so already, follow these simple instructions to [install Rust, Node.js, SSVM, and ssvmup](https://www.secondstate.io/articles/setup-rust-nodejs/).


## Build the application

```
$ ssvmup build
```

## Test

```
$ cd node
$ node test.js
[0.5,-3.0]
102334155
60
14
0
1
-2
242
24
```

## Web app

```
$ cd node
$ node server.js
Listening at http://localhost:8080
```

Then point your browser to `http://hostname:8080` try out various functions and hit the "Solve" button to find the answers.

