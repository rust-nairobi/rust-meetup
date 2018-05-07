## NOTES

Concurrency is one of the goals Rust pursues.

**What's Concurrency?**

- It addresses how to create threads to run multiple pieces of code at the same time.
- Rust's ownership model prevents concurrency bugs and memory safety bugs.

Threads
Basic building blocks for concurrency.
Two types:
1. **OS threads** -- offered by OS
2. **green threads** -- sits on top os OS threads

Code Example:
```rust
use std::thread;
use std::time::Duration;

fn main() {
    thread::spawn(|| {
        for i in 1..10 {
            println!("hi number {} from the spawned thread!", i);
            thread::sleep(Duration::from_millis(1));
        }
    });

    for i in 1..5 {
        println!("hi number {} from the main thread!", i);
        thread::sleep(Duration::from_millis(1));
    }
}
```

## Base Concepts Primer
* **Mutability**
    * something can change
    * by default everything is immutable
* **Ownership & Borrowing**
    * who owns the data and who they lend it to
    * every data can be owned and passed
    * ownership is scoped
    * you can borrow mutably one
    * you can borrow immutably
* Things to note
    * Rust is statically typed
    * **variables**: type annotations are optional
    * **functions**
    * common Rust types: integers, &str, String, 
    * **structs**: tuple structs, classic C structs, Unit structs
    * **Enums**
    * **Options**: used to express the possibility of absence
    * **Result**: used to express the possibility of error
* Advanced features
    * **Generics**
    * **Traits**

## Introduction to threading in Rust

Code Example
```rust
```

* **join**
* **closures**: 
* **move** statement/keyword
* Rust uses OS threads
* `thread::sleep`: thread stop its execution temporarily
* `thread::spawn`: create a new thread
* **Sync & Send**: 
    * Sync - types that are safe to share refences between threads
    * Send - types that are safe to send refences between threads
* **Messaging passing**
    * concurrency by sharing messages
    * uses **channels** such as `mpsc::channel`
* Shared state concurrency
* Mutex: mutual exclusion primitive useful for protecting shared data
* Multiple threads ownership of mutex

## Single threading and multi threading
* Single threaded - single threaded web server
    * HTTP / TCP server

#
```rust
use std::io::prelude::*;
use std::net::TcpListener;
use std::net::TcpStream;
use std::fs::File;

fn main() {
    let listener = TcpListener::bind("127.0.0.1:8000").unwrap();

    for stream in listener.incoming() {
        let stream = stream.unwrap();

        handle_connection(stream);
    }
}

fn handle_connection(mut stream: TcpStream) {
    let mut buffer = [0; 512];
    stream.read(&mut buffer).unwrap();
    
    let get = b"GET / HTTP/1.1\r\n";

    let (status_line, filename) = if buffer.starts_with(get) {
        ("HTTP/1.1 200 OK\r\n\r\n", "hello.html")
    } else {
        ("HTTP/1.1 404 NOT FOUND\r\n\r\n", "404.html")
    };

    let mut file =File::open(filename).unwrap();

    let mut contents = String::new();
    file.read_to_string(&mut contents).unwrap();

    let response = format!("{}{}", status_line, contents);
    println!("Request: {}", String::from_utf8_lossy(&buffer[..]));

    stream.write(response.as_bytes()).unwrap();
    stream.flush().unwrap();
}
```

