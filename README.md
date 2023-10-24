# Rusty Journal

A journal built using Rust

Functions:
1) Add tasks to list
2) Task complete / Remove task from list
3) List of tasks

## Add tasks to list
```
cargo run -- -j test-journal.json add "comment"
```

## Remove task from list
```
cargo run -- -j test-journal.json done <number in list>
```

## List of tasks
```
cargo run -- -j test-journal.json list
```
