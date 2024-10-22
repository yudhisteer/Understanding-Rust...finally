# Data-Structures-with-Rust

## Plan of Action
1. Intro to Rust
    - Core Concepts
    - Ownership and Borrowing
    - Lifetimes

2. Linked Lists


---------------------------
## 1. Intro to Rust

### 1.1 Core Concepts



### 1.2 Ownership and Borrowing


```rust
#[derive(Debug)]
struct Student {

    id: u32,
    name: String,
    score: i32,
}

impl Student {
    // Associated fn to create new student
    fn new_student(id: u32, name: String) -> Self {
        Student {
            id: id,
            name: name,
            score: 0,
        }
    }
}

#[derive(Debug)]
struct School {
    students: Vec<Student>,
}

impl School {
    // Associated fn to create new school
    fn new_school() -> Self {
        School {students: vec![]}
    }
}
```

```rust
fn print_student (student: Student) {
    println!("{:#?}", student);
}
```

```rust
fn main() {
    let new_school = School::new_school();
    let new_student = Student::new_student(7, String::from("Joe"));
    print_student(new_student);
    print_student(new_student);
}
```

```rust
error[E0382]: use of moved value: `new_account`
  --> src/main.rs:45:19
   |
41 |     let new_account = Account::new(1, String::from("Joe"));
   |         ----------- move occurs because `new_account` has type `Account`, which does not implement the `Copy` trait
...
44 |     print_account(new_account);
   |                   ----------- value moved here
45 |     print_account(new_account);
   |                   ^^^^^^^^^^^ value used here after move
   |
note: consider changing this parameter type in function `print_account` to borrow instead if owning the value isn't necessary
  --> src/main.rs:34:28
   |
34 | fn print_account (account: Account) {
   |    -------------           ^^^^^^^ this parameter takes ownership of the value
   |    |
   |    in this function
note: if `Account` implemented `Clone`, you could clone the value
  --> src/main.rs:4:1
   |
4  | struct Account {
   | ^^^^^^^^^^^^^^ consider implementing `Clone` for this type
...
44 |     print_account(new_account);
   |                   ----------- you could clone this value

```





-------------
