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

<p align="center">
  <img src="https://github.com/user-attachments/assets/f9172b1e-4347-4278-96c2-cae4267c498c" width="50%" />
</p>



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

**Ownership**

1. Every value is "owned" by a single variable, struct, vector, etc. at a time.
2. Reassigning the value to another variable, passing it to a function, or putting it into a vector moves the value. The old variable can't be used anymore!
   
**Borrowing**

3. You can create many read-only references to a value that exist at the same time.
4. You can't move a value while a reference to the value exists.
5. You can make a writable (mutable) reference to a value only if there are no read-only references currently in use. One mutable reference to a value can exist at a time.
6. You can't mutate a value through the owner when any reference (mutable or immutable) to the value exists.
7. Some types of values are copied instead of moved (numbers, booleans, characters, arrays/tuples with copyable elements).

**Lifetimes**

8. When a variable goes out of scope, the value owned by it is dropped (cleaned up in memory).
9. Values can't be dropped if there are still active references to them.
10. References to a value can't outlive the value they refer to.



**Example #1**

```rust
fn main() {
    let school = School::new_school();
    let other_school = school;
    println!("{:#?}", school);
}
```

<p align="center">
  <img src="https://github.com/user-attachments/assets/a8e6f2c7-c036-48da-8cc1-2303b4b5cdd3" width="60%" />
</p>

**Example #2**

```rust
fn print_student (student: Student) {
    println!("{:#?}", student);
}

fn main() {
    let new_student = Student::new_student(7, String::from("Joe"));
    print_student(new_student);
    print_student(new_student);
}
```

<p align="center">
  <img src="https://github.com/user-attachments/assets/08a98042-27ab-4744-a4d0-c709e4c035f8" width="70%" />
</p>


**Example #3**

```rust
fn print_student (student: Student) {
    println!("{:#?}", student);
}


fn main() {
    let student = Student::new_student(7, String::from("Joe"));
    let list_of_students = vec![student];
    print_student(student);
}
```


-------------
