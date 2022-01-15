A test case for per-worker instance data in Neon.

## Explanation

The Neon module stores an object handle in a `Root` in a static cell, allowing the main JS thread to share an object handle unsafely with a worker thread.

## Build

```
cd my-lib
npm i
```

## Run

```
node app.js
```

