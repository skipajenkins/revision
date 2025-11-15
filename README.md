# 🦀 Structs Revision in Rust

This project revisits core concepts from the Structs chapter of The Rust Programming Language.
It explores multiple ways to calculate the area of a triangle, helping reinforce:

* Struct definitions

* Borrowing & references

* Tuples

* Debugging with dbg!

* Function patterns

This revision focuses on repetition for mastery while comparing different approaches for solving the same problem.

---

## ⚙️ Setting Up the Environment

Before running the code, ensure that both Rust and Cargo are installed on your system.

✔️ Step 1: Verify Your Installation
```bash
rustc --version
cargo --version
```

If Rust isn't installed, use:
```bash
curl https://sh.rustup.rs -sSf | sh
```

Then verify again with the commands above.

📁 Step 2: Create the Project

Create a new Rust project:
```bash
cargo new revision
```

Move into the folder:
```bash
cd revision
```

Replace the contents of src/main.rs with the following code.

---

## 📜 Rust Code
```bash
fn main() {

    let tri_1 = area_of_triangle(60.0,30.0);
    println!("The area of tri1: {}.", tri_1);

    // Tuples refresher
    let tri_2:(f32,f32) = (120.0,60.0);
    let tri_2_area = area_of_triangle_2(tri_2);
    println!("The area of the tri2: {}.", tri_2_area);

    // Struct example
    let tri_3 = Triangle{
        base: 240.0,
        height: 120.0,
    };

    dbg!(&tri_3); // Debug macro

    let tri_3_area =  area_of_triangle_3(&tri_3);
    println!("This is the area of tri3: {}.", tri_3_area);
}

fn area_of_triangle(base:f32,height:f32) -> f32{
    0.5 * base * height
}

fn area_of_triangle_2(dimensions:(f32,f32)) -> f32{
    0.5 * dimensions.0 * dimensions.1
}

#[derive(Debug)]
struct Triangle{
    base: f32,
    height: f32,
}

fn area_of_triangle_3(triangle:&Triangle) -> f32{
    0.5 * triangle.base * triangle.height 
}
```

▶️ Step 3: Build and Run
🧱 Build
```bash
cargo build
```

🎬 Run
```bash
cargo run
```

---

## 🧠 Key Concepts Explained
Concept	Description
Struct (struct)	A custom data type that groups related fields.
Tuple ((T, T))	A lightweight fixed-size grouping of values without named fields.
Borrowing (&T)	Allows read-only access without taking ownership.
Debugging (dbg!)	Prints values with context for examination.
Multiple Approaches	Functions demonstrate different ways to compute the same result.

🧩 Example Output
```bash
The area of tri1: 900.
The area of tri2: 3600.
[src/main.rs:20] &tri_3 = Triangle {
    base: 240.0,
    height: 120.0,
}
This is the area of tri3: 14400.
```

---

## 🔍 How It Works
1️⃣ Direct Parameters

The simplest function accepts the base and height directly.

2️⃣ Tuple Input

Good review of tuple syntax and indexing (tuple.0, tuple.1).

3️⃣ Struct-Based Approach

A Triangle struct provides clear, descriptive fields.

4️⃣ Borrowing Instead of Owning

Borrowing with &Triangle avoids moving ownership and is the most idiomatic Rust approach.

5️⃣ Debugging

Using:
```bash
dbg!(&tri_3)
```

prints structured diagnostic output.

---

## 🎯 Learning Objectives

This project reinforces:

* Struct creation and instantiation

* Borrowing & references with functions

* Returning values vs referencing struct fields

* Tuple usage and indexing

* Debugging using the dbg! macro

* Repetition for long-term Rust mastery

---

## 🚀 Future Enhancements

Add methods using impl blocks

Implement a triangle perimeter function

Add unit tests for all area functions

Convert the struct approach into an OOP-style design

Provide a CLI interface using clap

---

## 🦀 Built With

Rust

Cargo

The Rust Book

Lots of caffeine ☕

---

## 📄 License

This project is open-source and available under the MIT License.
