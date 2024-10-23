# Data-Structures-with-Rust

## Plan of Action
1. Intro to Rust
    - Core Concepts
    - Ownership
    - Borrowing
    - Lifetimes

2. Linked Lists


---------------------------
## 1. Intro to Rust

### 1.1 Core Concepts



### 1.2 Ownership

- Every value is "owned" by a single variable, struct, vector, etc. at a time.
- Reassigning the value to another variable, passing it to a function, or putting it into a vector moves the value. The old variable can't be used anymore!
   


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






**Example #1**

```rust
fn print_student (student: Student) {
    println!("{:#?}", student);
}

fn main() {
    let school = School::new_school();
    let other_school = school;
    println!("{:#?}", school);
}
```

```rust
error[E0382]: borrow of moved value: `school`
  --> src/test.rs:53:23
   |
51 |     let school = School::new_school();
   |         ------ move occurs because `school` has type `School`, which does not implement the `Copy` trait
52 |     let other_school = school;
   |                        ------ value moved here
53 |     println!("{:#?}", school);
   |                       ^^^^^^ value borrowed here after move
   |
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

```rust
error[E0382]: use of moved value: `new_student`
  --> src/test.rs:53:19
   |
51 |     let new_student = Student::new_student(7, String::from("Joe"));
   |         ----------- move occurs because `new_student` has type `Student`, which does not implement the `Copy` trait
52 |     print_student(new_student);
   |                   ----------- value moved here
53 |     print_student(new_student);
   |                   ^^^^^^^^^^^ value used here after move
   |
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

```rust
error[E0382]: use of moved value: `student`
  --> src/test.rs:53:19
   |
51 |     let student = Student::new_student(7, String::from("Joe"));
   |         ------- move occurs because `student` has type `Student`, which does not implement the `Copy` trait
52 |     let list_of_students = vec![student];
   |                                 ------- value moved here
53 |     print_student(student);
   |                   ^^^^^^^ value used here after move
   |
```

<p align="center">
  <img src="https://github.com/user-attachments/assets/3a96f2c1-999e-46a0-b155-59dcf0254f7b" width="70%" />
</p>


**Example #4**

```rust
fn main() {
    let school = School::new_school();
    let students = school.students;
    println!("{:#?}", school.students);
}
```

```rust
error[E0382]: borrow of moved value: `school.students`
  --> src/test.rs:53:23
   |
52 |     let students = school.students;
   |                    --------------- value moved here
53 |     println!("{:#?}", school.students);
   |                       ^^^^^^^^^^^^^^^ value borrowed here after move
```


<p align="center">
  <img src="https://github.com/user-attachments/assets/6427e6c4-c027-47ea-8e01-30abd32cc7af" width="60%" />
</p>




**Example #5**


```rust
fn print_student (student: Student) {
    println!("{:#?}", student);
}


fn main() {
    let student = Student::new_student(7, String::from("Joe"));
    print_student(student);
    println!("{:#?}", student.name);
}
```

```rust
error[E0382]: borrow of moved value: `student`
  --> src/test.rs:54:23
   |
52 |     let student = Student::new_student(7, String::from("Joe"));
   |         ------- move occurs because `student` has type `Student`, which does not implement the `Copy` trait
53 |     print_student(student);
   |                   ------- value moved here
54 |     println!("{:#?}", student.name);
   |                       ^^^^^^^^^^^^ value borrowed here after move
```



<p align="center">
  <img src="https://github.com/user-attachments/assets/245854c9-63cc-4fce-951b-77c300ba4b65" width="80%" />
</p>


**Example #6**

```rust
fn print_student (student: Student) {
    println!("{:#?}", student);
}

fn print_name(name: String) {
    println!("{:#?}", name);
}

fn main() {
    let student = Student::new_student(7, String::from("Joe"));
    print_name(student.name);
    print_student(student);
}
```

```rust
error[E0382]: use of partially moved value: `student`
  --> src/test.rs:54:19
   |
53 |     print_name(student.name);
   |                ------------ value partially moved here
54 |     print_student(student);
   |                   ^^^^^^^ value used here after partial move
   |
   = note: partial move occurs because `student.name` has type `String`, which does not implement the `Copy` trait
```


<p align="center">
  <img src="https://github.com/user-attachments/assets/543ef163-7720-414a-94dd-d89a35e01363" width="80%" />
</p>

### 1.3 Borrowing


#### "Workaround"

```rust
fn print_student(student: Student) -> Student {
    println!("{:#?}", student);
    student //implicit return
}

fn main() {
    let mut student = Student::new_student(7, String::from("Joe"));
    student = print_student(student);
    student = print_student(student);
    println!("{:?}", student);
}
```

<p align="center">
  <img src="https://github.com/user-attachments/assets/b2a886b2-6c81-4e98-b3d6-64ad91482d76" width="80%" />
</p>


**Borrowing - Immutable References**

- You can create many read-only references to a value that exist at the same time.
- You can't move a value while a reference to the value exists.



```rust
fn print_student(student: &Student) {
    println!("{:#?}", student);
}

fn main() {
    let student = Student::new_student(7, String::from("Joe"));
    print_student(&student);
    println!("{:?}", student);
}
```

<p align="center">
  <img src="https://github.com/user-attachments/assets/68f46487-f298-470e-a807-0cc2e2990f23" width="80%" />
</p>


Here, we see the & operator being used on a type. This means the arguments needs to be a reference to a value. 

```rust
fn print_student(student: &Student) {
```

& operator being used on an owner of a value. this mens we wil create a reference to this value.

```rust
print_student(&student);
```

Rule no 4. 

```rust
fn print_student(student: &Student) {
    println!("{:#?}", student);
}

fn main() {
    let student = Student::new_student(7, String::from("Joe"));
    
    print_student(&student);      // First immutable borrow
    print_student(&student);      // Second immutable borrow

    let other_student = student;  // Ownership of 'student' is moved to 'other_student'
    println!("{:?}", student);    // Error: 'student' has been moved and can no longer be used
}
```

```rust
error[E0382]: borrow of moved value: `student`
  --> src/test.rs:63:22
   |
57 |     let student = Student::new_student(7, String::from("Joe"));
   |         ------- move occurs because `student` has type `Student`, which does not implement the `Copy` trait
...
62 |     let other_student = student;
   |                         ------- value moved here
63 |     println!("{:?}", student);
   |                      ^^^^^^^ value borrowed here after move
   |
```

<p align="center">
  <img src="https://github.com/user-attachments/assets/8485721d-4249-4f2e-b325-7fe94f10356a" width="80%" />
</p>


**Borrowing - Mutable References**

- You can make a writable (mutable) reference to a value only if there are no read-only references currently in use. One mutable reference to a value can exist at a time.
- You can't mutate a value through the owner when any reference (mutable or immutable) to the value exists.
- Some types of values are copied instead of moved (numbers, booleans, characters, arrays/tuples with copyable elements).




```rust
fn change_score (student: &mut Student) {
    student.score = 100;
}

fn main() {
    let mut student = Student::new_student(7, String::from("Joe"));
    change_score(&mut student);
    println!("{:#?}", student);
}
```


Not to do:


```rust
fn change_score (student: &mut Student) {
    student.score = 100;
}

fn main() {
    let mut student = Student::new_student(7, String::from("Joe"));
    let student_ref = &student;
    change_score(&mut student);
    println!("{:#?}", student_ref.name);
}
```

```rust
error[E0502]: cannot borrow `student` as mutable because it is also borrowed as immutable
  --> src/test.rs:49:18
   |
48 |     let student_ref = &student;
   |                       -------- immutable borrow occurs here
49 |     change_score(&mut student);
   |                  ^^^^^^^^^^^^ mutable borrow occurs here
50 |     println!("{:#?}", student_ref.name);
   |                       ---------------- immutable borrow later used here
```






```rust
fn change_score (student: &mut Student) {
    student.score = 100;
}

fn main() {
    let mut student = Student::new_student(7, String::from("Joe"));
    let student_ref = &mut student;
    change_score(&mut student);
    println!("{:#?}", student_ref.name);
}
```

```rust
error[E0499]: cannot borrow `student` as mutable more than once at a time
  --> src/test.rs:49:18
   |
48 |     let student_ref = &mut student;
   |                       ------------ first mutable borrow occurs here
49 |     change_score(&mut student);
   |                  ^^^^^^^^^^^^ second mutable borrow occurs here
50 |     println!("{:#?}", student_ref.name);
   |                       ---------------- first borrow later used here
```



```rust
fn change_score (student: &mut Student) {
    student.score = 100;
}

fn main() {
    let mut student = Student::new_student(7, String::from("Joe"));
    let student_ref = &mut student;
    student.score = 100;
    change_score(student_ref);
    println!("{:#?}", student);
}
```

```rust
error[E0506]: cannot assign to `student.score` because it is borrowed
  --> src/test.rs:49:5
   |
48 |     let student_ref = &mut student;
   |                       ------------ `student.score` is borrowed here
49 |     student.score = 100;
   |     ^^^^^^^^^^^^^^^^^^^ `student.score` is assigned to here but it was already borrowed
50 |     change_score(student_ref);
   |                  ----------- borrow later used here
```



To do:

```rust
fn add_student(school: &mut School, student: Student) {
    school.students.push(student);
}

fn main() {
    let mut school = School::new_school();
    let student = Student::new_student(7, String::from("Joe"));
    add_student(&mut school, student);
    println!("{:#?}", school);
    println!("{:#?}", school.students[0]); // print 'student' value
}
```

```
School {
    students: [
        Student {
            id: 7,
            name: "Joe",
            score: 0,
        },
    ],
}


Student {
    id: 7,
    name: "Joe",
    score: 0,
}
```

### 1.4 Lifetimes

- When a variable goes out of scope, the value owned by it is dropped (cleaned up in memory).
- Values can't be dropped if there are still active references to them.
- References to a value can't outlive the value they refer to.



```rust
fn print_score(student: &Student) {
    println!("{}", student.score);
}

fn main() {
    let mut school = School::new_school();
    let student = Student::new_student(1, String::from("me"));

    let student_ref = &student; // Immutable reference to 'account'
 
    school.students.push(student); // Error: 'student' is moved here

    print_score(student_ref); // Error: Cannot use 'student_ref' after 'student' has been moved
}
```

```rust
fn print_score(student: &Student) {
    println!("{}", student.score);
}

fn main() {
    let mut school = School::new_school();
    let student = Student::new_student(1, String::from("me"));

    let student_ref = &student; // Immutable reference to 'student'
 
    print_score(student_ref); // Borrowing 'student' as immutable for printing its score
 
    school.students.push(student); // 'student' is moved into 'school.accounts'
}
```

Storing a value --> Take ownership (receive a value)
Calculation with a value --> Read-only ref (&)
Change a value --> Mutable Ref (&mut)


-------------
