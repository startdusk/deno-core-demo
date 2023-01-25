import { print } from "./basic_load.js"

async function hello() {
    return new Promise((res, _rej) => {
        print("Hello world\n");
        res("Rust: hello");
    });
}

hello();