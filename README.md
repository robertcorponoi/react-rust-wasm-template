# React Rust Wasm Template

An opinionated React + Rust Wasm template that helps you understand how to develop a React Wasm app using Rust.

**Note:** The React client does **not** use `react-scripts`. It is essentially the same as an ejected app as we need the fine grain control over the build scripts. Each script should be well documented and the documentation for each tool can be helpful if you are unsure what an option does.

**Table of Contents**

- [Getting Started](#getting-started)
- [Using Wasm in the React Client](#using-wasm-in-the-react-client)
- [Changing the Default Naming Convention](#changing-the-default-naming-conventions)
- [License](#license)

## Getting Started

1. The first thing we need to do is install all of the tools that we need:

- [Node + NPM](https://nodejs.org/en/) to run the React app.
- [Rust](https://www.rust-lang.org/tools/install) to use Rust to create and build our Wasm.
- [Cargo Watch](https://github.com/watchexec/cargo-watch) to watch for changes to the Rust source and rebuild the Wasm automatically.

2. Change to the `wasm` directory. We use the `scripts/develop.sh` shell script to rebuild the Wasm bundle when changes are made to the source. Make sure that this script has the correct permissions to be executed by running:

```sh
$ chmod +x scripts/develop.sh
```

Make sure this works by running the script using:

```sh
$ scripts.develop.sh
```

You should see the the Wasm being built and output to a `wasm-build` directory at the root (alongside the `wasm` and `react` directories). The process should not exist as it'll watch the source for changes. To make sure this works, to go the `src/lib.rs` file and add a simple change and save the file. You should see it rebuild and update the `wasm-build` directory.

At this point the source and client will be watched for changes and rebuilt automatically.

## Using Wasm in the React Client

While there are some examples in the template, let's go through the process of adding a new function and calling it in the React client.

1. Open up two terminals and the following scripts in them:

```sh
$ cd wasm && ./scripts/develop.sh
```

```sh
$ cd react && npm run develop
```

2. Open up the `wasm/src/lib.rs` file so that we can add a new function. In here, add the following:

```rs
/// Takes a name as input and returns a greeting in the format of "Hello, Name".
/// 
/// `name` - The name to use in the greeting.
#[wasm_bindgen]
pub fn greeting(name: String) -> String {
    return format!("Hello, {}", name)
}
```

A pretty simple function that will take in a name as a string and return a string with a simple greeting using the provided name. Once you save the file, you should notice a new build being created.

3. Now we can head over to the React client and use our new function. Go to any component you wish and import the `greeting` function from the `wasm` package like so:

```ts
import { greeting } from "wasm";
```

Then, in the `return` part of the component, use it like so:

```tsx
return (
    <p>{greeting("Bob")}</p>
);
```

Now if you go to `http://localhost:3000` (or the page that your component is on) you should see "Hello, Bob" displayed.

## Changing the Default Naming Conventions

By default the React client is in a directory named `react`, the Wasm source is in a directory named `wasm`, the built Wasm is in a directory named `wasm-build`, and finally the Wasm is imported into the React client as `wasm`. Any of these can be changed like so:

- React client (default `react`) - You can change this to be named whatever you wish without having to change other directories or definitions.

- Wasm source (default `wasm`) - You can also change this to be named whatever you wish without having to hange other directories or definitions.

- Wasm built (default `wasm-build`) - To change this, you need to first change the `--out-dir` value in the `wasm/scripts.develop.sh` script. This will affect where the built Wasm will be placed. After this, you need to go into the React client's `package.json` and modify the following line under the `dependencies` section:

```json
{
    "dependencies": {
        "wasm": "file:../wasm-build"
    }
}
```

You need to change the `"file:../wasm-build"` to the new name you set for the Wasm build output. 

- Wasm import (default `wasm`) - In the React client's `package.json`, the built Wasm is imported as:

```json
{
    "dependencies": {
        "wasm": "file:../wasm-build"
    }
}
```

To change this, you change the key of `wasm` to whatever you wish to import the built Wasm as. However, you will also have to change this anywhere in your code where you import the `wasm` package.

## License

[MIT](./LICENSE)